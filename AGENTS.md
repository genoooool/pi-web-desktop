# AGENTS.md

## Pi and Pi Web upgrades

When the user asks to upgrade Pi, Pi Web, or both, follow the runbook in
[`README.md`](README.md#updating-pi-and-pi-web). Do not rediscover the package
names or installation layout unless a documented command fails or the upstream
layout has changed.

- Pi is the global npm package `@earendil-works/pi-coding-agent`.
- Pi Web is the global npm package `@agegr/pi-web`.
- This desktop wrapper expects Homebrew Node.js and npm prefix
  `/opt/homebrew`.
- Before upgrading, run `npm config get prefix`. Stop and report the mismatch
  instead of installing into a different prefix.
- Upgrade both stable packages with:

  ```bash
  npm install -g \
    @earendil-works/pi-coding-agent@latest \
    @agegr/pi-web@latest
  ```

- Verify with:

  ```bash
  pi --version
  npm ls -g @earendil-works/pi-coding-agent @agegr/pi-web --depth=1
  npm outdated -g @earendil-works/pi-coding-agent @agegr/pi-web --depth=0
  ```

- Never run `pi-web --version`; it starts the server instead of printing a
  version. Read the Pi Web version from `npm ls`.
- A Pi entry marked `deduped` beneath Pi Web is the same physical installation,
  not a duplicate package.
- If port 30141 is occupied, identify the exact listener before stopping it.
- An upstream-only upgrade does not require changing or rebuilding this
  repository while the expected Node.js and Pi Web paths remain unchanged.
