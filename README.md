# Rust GUI Calculator

## Overview

A simple Rust GUI calculator app built using [Slint](https://slint.dev).\
Made entirely for the purpose of learning basic Rust and Slint API.

## Showcase

![showcase.gif](https://github.com/fqidz/rust-calculator-gui/blob/main/showcase.gif)

## Build
Windows:
```pwsh
cargo rustc --release --bin rust-calculator-gui -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"
```