#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::Manager;

const PORT: u16 = 30141;
const PI_WEB_SCRIPT: &str = "/opt/homebrew/lib/node_modules/@agegr/pi-web/bin/pi-web.js";
const NODE_BIN: &str = "/opt/homebrew/bin/node";

struct ServerState {
    child: Mutex<Option<Child>>,
    shutdown: AtomicBool,
}

fn log_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    let dir = PathBuf::from(home).join("Library").join("Logs");
    let _ = fs::create_dir_all(&dir);
    dir.join("pi-web-app.log")
}

fn log_line(msg: &str) {
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path()) {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = writeln!(f, "[{}] {}", ts, msg);
    }
}

/// 杀掉占用 30141 端口的残留 node 进程（上次没退干净的 pi-web）
fn free_port() {
    let out = Command::new("lsof")
        .args(["-nP", &format!("-i:{}", PORT), "-sTCP:LISTEN", "-t"])
        .output();
    let Ok(out) = out else { return };
    for pid in String::from_utf8_lossy(&out.stdout).lines() {
        let pid = pid.trim();
        if pid.is_empty() {
            continue;
        }
        // 只杀 pi-web 相关进程（node 包装器 / next-server），避免误伤
        let comm = Command::new("ps").args(["-o", "comm=", "-p", pid]).output();
        let is_pi_web = comm
            .map(|c| {
                let s = String::from_utf8_lossy(&c.stdout);
                s.contains("node") || s.contains("next-server")
            })
            .unwrap_or(false);
        if is_pi_web {
            log_line(&format!("killing stale process on port {}: pid {}", PORT, pid));
            let _ = Command::new("kill").args(["-9", pid]).output();
        }
    }
}

fn spawn_server() -> Option<Child> {
    free_port();
    let log = OpenOptions::new().create(true).append(true).open(log_path()).ok()?;
    let log_err = log.try_clone().ok()?;
    match Command::new(NODE_BIN)
        .arg(PI_WEB_SCRIPT)
        .arg("--hostname")
        .arg("127.0.0.1")
        .arg("--no-open")
        .env("PATH", "/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin")
        .stdout(Stdio::from(log))
        .stderr(Stdio::from(log_err))
        .spawn()
    {
        Ok(child) => {
            log_line(&format!("pi-web spawned, pid {}", child.id()));
            Some(child)
        }
        Err(e) => {
            log_line(&format!("failed to spawn pi-web: {}", e));
            None
        }
    }
}

fn main() {
    let state = Arc::new(ServerState {
        child: Mutex::new(None),
        shutdown: AtomicBool::new(false),
    });

    // 看门狗：pi-web 意外退出后自动重启
    {
        let state = state.clone();
        std::thread::spawn(move || loop {
            if state.shutdown.load(Ordering::SeqCst) {
                break;
            }
            std::thread::sleep(Duration::from_secs(2));
            let mut guard = state.child.lock().unwrap();
            let needs_respawn = match guard.as_mut() {
                Some(c) => !matches!(c.try_wait(), Ok(None)),
                None => false,
            };
            if needs_respawn {
                log_line("pi-web exited unexpectedly, respawning");
                *guard = spawn_server();
                if guard.is_none() {
                    // 失败则 5 秒后再试，避免疯狂空转
                    drop(guard);
                    std::thread::sleep(Duration::from_secs(5));
                }
            }
        });
    }

    let app_state = state.clone();
    let app = tauri::Builder::default()
        .setup(move |app| {
            let child = spawn_server().expect("failed to start pi-web");
            *app_state.child.lock().unwrap() = Some(child);
            app.manage(app_state.clone());
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("failed to build app");

    app.run(move |app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            let state = app_handle.state::<Arc<ServerState>>();
            state.shutdown.store(true, Ordering::SeqCst);
            let mut guard = state.child.lock().unwrap();
            if let Some(mut child) = guard.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
            drop(guard);
            // pi-web.js 会派生 next-server 孙进程，只杀直接子进程会留下孤儿占着端口
            free_port();
            log_line("app exit, pi-web killed");
        }
    });
}
