# Pi Web Desktop

<p align="center">
  <img src="src-tauri/icons/icon.png" width="160" alt="Pi Web Desktop macOS app icon">
</p>

<p align="center">
  A lightweight macOS desktop app wrapper for Pi Web and the Pi coding agent, built with Tauri.
</p>

## Screenshot

![Pi Web Desktop screenshot showing the Pi Web plugin manager](docs/pi-web-desktop.png)

## What is Pi Web Desktop?

Pi Web Desktop turns the local Pi Web interface into a native macOS application. Open the app to start Pi Web automatically and use it inside a dedicated WKWebView window—no browser tab or terminal command required.

This repository is intentionally small. It does **not** fork, bundle, or modify Pi Web or Pi. It only provides an application shell that:

- starts the locally installed `@agegr/pi-web` service on `127.0.0.1:30141`;
- displays Pi Web in a native Tauri window;
- restarts the service if it exits unexpectedly;
- cleans up the local service when the app quits;
- writes runtime logs to `~/Library/Logs/pi-web-app.log`.

> [!IMPORTANT]
> This project depends on [agegr/pi-web](https://github.com/agegr/pi-web) and the original [Pi coding agent](https://github.com/earendil-works/pi). Pi was previously hosted at [`badlogic/pi-mono`](https://github.com/badlogic/pi-mono), which redirects to its current repository. This project is an independent desktop wrapper and is not affiliated with the upstream maintainers.

## 中文说明

Pi Web Desktop 是一个轻量的 macOS 应用壳：它自动启动本地安装的 [Pi Web](https://github.com/agegr/pi-web)，并通过 Tauri/WKWebView 将界面装进独立桌面窗口。

本项目没有修改或重新打包 Pi Web 与原版 [Pi](https://github.com/earendil-works/pi)，只增加了启动、窗口、进程看护、退出清理和日志功能。

## Requirements

- macOS on Apple Silicon
- [Node.js](https://nodejs.org/) 22.19.0 or newer
- [Rust](https://www.rust-lang.org/tools/install)
- [Pi Web](https://github.com/agegr/pi-web) installed globally with Homebrew's Node.js

## Download

For Apple Silicon Macs, download the latest DMG installer from the [v1.0.1 GitHub Release](https://github.com/genoooool/pi-web-desktop/releases/download/v1.0.1/Pi.Web_1.0.1_aarch64.dmg), or browse the [Releases page](https://github.com/genoooool/pi-web-desktop/releases).

The current launcher expects these Homebrew paths:

```text
/opt/homebrew/bin/node
/opt/homebrew/lib/node_modules/@agegr/pi-web/bin/pi-web.js
```

## Build

Install Pi Web and the desktop wrapper dependencies:

```bash
npm install -g @agegr/pi-web
npm install
```

Build the macOS application:

```bash
npm run build -- --bundles app
```

The app bundle will be created at:

```text
src-tauri/target/release/bundle/macos/Pi Web.app
```

You can open the build directly or copy it to `/Applications`.

## Development

```bash
npm install
npm run dev
```

Pi Web Desktop loads a small startup page while the native wrapper waits for the local service, then the wrapper navigates the window to `http://127.0.0.1:30141`.

## Project structure

```text
frontend/index.html         Startup screen and local service polling
src-tauri/src/main.rs       Pi Web process lifecycle and Tauri app
src-tauri/tauri.conf.json   Window and bundle configuration
src-tauri/icons/            macOS application icons
```

## Troubleshooting

View the application log:

```bash
tail -50 ~/Library/Logs/pi-web-app.log
```

Check whether Pi Web is listening:

```bash
lsof -nP -i:30141 -sTCP:LISTEN
```

## Updating Pi and Pi Web

This desktop wrapper does not bundle Pi or Pi Web. It launches the globally
installed Homebrew Node.js copy of Pi Web, which in turn uses Pi. The expected
packages and paths are:

```text
Pi:      @earendil-works/pi-coding-agent
Pi Web:  @agegr/pi-web
prefix:  /opt/homebrew
```

Quit Pi Web Desktop before upgrading, then confirm that the active npm uses the
expected prefix:

```bash
npm config get prefix
# Expected on this Mac: /opt/homebrew
```

Upgrade both packages to their current stable releases. Re-running this command
is safe: npm updates/reuses the packages in the same global prefix rather than
creating another installation.

```bash
npm install -g \
  @earendil-works/pi-coding-agent@latest \
  @agegr/pi-web@latest
```

Verify the installed versions and check whether either package is still
outdated:

```bash
pi --version
npm ls -g \
  @earendil-works/pi-coding-agent \
  @agegr/pi-web \
  --depth=1
npm outdated -g \
  @earendil-works/pi-coding-agent \
  @agegr/pi-web \
  --depth=0
```

No output from `npm outdated` means both are current. In the dependency tree,
Pi shown as `deduped` under Pi Web means both entries reuse the same physical
installation; it is not a duplicate copy.

> [!WARNING]
> Do not use `pi-web --version`: Pi Web does not currently implement that
> option and will start the server on port 30141 instead. Read its version from
> `npm ls` above. If port 30141 is unexpectedly occupied, identify the listener
> with `lsof -nP -iTCP:30141 -sTCP:LISTEN` before stopping anything.

Reopen Pi Web Desktop after the upgrade. A desktop-app rebuild is unnecessary
as long as Node.js and Pi Web remain at the expected `/opt/homebrew` paths.

## Upstream projects and trademarks

- [Pi Web](https://github.com/agegr/pi-web) provides the local web interface.
- [Pi](https://github.com/earendil-works/pi) provides the coding agent and related tooling.
- All upstream names, logos, and trademarks belong to their respective owners.

## License

The wrapper code in this repository is available under the [ISC License](LICENSE). Pi Web and Pi remain subject to their own licenses.
