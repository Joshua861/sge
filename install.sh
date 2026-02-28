#!/usr/bin/env sh

rustup toolchain install nightly
rustup default nightly
rustup component add rustc-codegen-cranelift
