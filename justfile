#!/usr/bin/env -S just

build:
    cargo build

release:
    cargo build --release

run:
    just log_clean
    just build
    zellij -l test-layout.kdl

log_clean:
    rm -f .zellij-what-time.log

log:
    tail -f .zellij-what-time.log

default:
    just run
