# babyicon

![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)

A lightweight CLI tool to download SVG icons from Hugeicons.

## Usage

```bash
babyicon "https://hugeicons.com/icon/github?style=stroke-rounded" -o icons/github.svg
```

Or with cargo:

```bash
cargo run -- "https://hugeicons.com/icon/github?style=stroke-rounded" -o icons/github.svg
```

---

> **Note:** Unofficial tool. Created to fill the gap until Hugeicons adds official Rust/Java support.
