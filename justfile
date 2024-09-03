#!/usr/bin/env -S just

build:
    cargo build

release:
    cargo build --release

run:
    just build
    zellij -l test-layout.kdl

log:
    tail -f .zellij-what-time.log

default:
    just run
