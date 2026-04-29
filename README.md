<div align = center>
<h1>babyicon</h1>

![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)
![Binary Size](https://img.shields.io/badge/bin%20size-321KB-blue)

A lightweight CLI tool to download SVG icons from Hugeicons.
</div>

## Usage

```bash
babyicon "https://hugeicons.com/icon/github?style=stroke-rounded" [output_path]
```

* `output_path` is **optional**
* If not provided, the file will be saved using the default filename (e.g. `github-stroke-rounded.svg`)

### Example

```bash
babyicon "https://hugeicons.com/icon/github?style=stroke-rounded" icons/github.svg
```

---

> **Note:** Unofficial tool. Created to fill the gap until Hugeicons adds official Rust/Java support.
