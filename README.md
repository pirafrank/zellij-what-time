# 🕔 zellij-what-time

[![CI](https://github.com/pirafrank/zellij-what-time/actions/workflows/ci.yml/badge.svg)](https://github.com/pirafrank/zellij-what-time/actions/workflows/ci.yml)

zellij-what-time is a Zellij plugin to show the host system date and/or time in the status bar. Zero-config, but customizable. Inspired by zellij-datetime.

## Why?

One of my setups is Mosh + Zellij + Blink Shell on my iPad. When Blink Shell is in full screen the iOS status bar with the time is not visible.

This plugin is designed to show the current time in the status bar of Zellij.

## Requirements

- Zellij 0.38.0 or later
- `date` (very unlikely you don't have it)

## Install

Download `zellij-what-time.wasm` from the [latest release](https://github.com/pirafrank/zellij-what-time/releases/latest) and place it to your Zellij plugins directory.

If you're using the default directory, just run:

```sh
mkdir -p ~/.config/zellij/plugins/
curl -L "https://github.com/pirafrank/zellij-what-time/releases/latest/download/zellij-what-time.wasm" \
  -o ~/.config/zellij/plugins/zellij-what-time.wasm
```

## Contributing

If you run into a glitch or if you want to suggest an idea, please [open an issue](https://github.com/pirafrank/zellij-what-time/issues/new).

## Development

Requirements:

- Rust
- cargo
- [watchexec](https://github.com/watchexec/watchexec)
- zellij (ofc!)
- wasm32-wasi target (`rustup target add wasm32-wasi`)
- neovim or any other terminal editor

Then run:

```sh
zellij -l zj-workspace.kdl
```

The Zellij workspace is configured to show your editor, the plugin, and a pane with build output. Build and reload happens automatically thanks to `watchexec`.

To just build the plugin, run as usual:

```sh
cargo build
```
