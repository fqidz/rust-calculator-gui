# Rust GUI Calculator

## Overview

A simple Rust GUI calculator app built using [Slint](https://slint.dev).
Made entirely for the purpose of learning basic Rust and Slint API.

## Showcase

![showcase.gif]()

## Build

```pwsh
cargo rustc --release --bin rust-calculator-gui -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"
```