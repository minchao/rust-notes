---
tags: rust
---

# 工具

## 開發

### rustfmt

https://github.com/rust-lang/rustfmt

程式碼格式化工具，會根據 Rust 程式碼風格指南進行格式化。

安裝：

```shell
$ rustup component add rustfmt
```

執行：

```shell
$ rustfmt [options] <file>...
```

或在使用 Cargo 管理的專案下執行：

```shell
$ cargo fmt
```

### rustfix

https://github.com/rust-lang/rustfix

自動修正編譯器請告的錯誤。

在使用 Cargo 管理的專案下執行：

```shell
$ cargo install cargo-fix
```

### Clippy

https://github.com/rust-lang/rust-clippy

Clippy 是一個 lints 工具集合，用於發現並修正常見的程式碼錯誤。

安裝：

```shell
$ rustup component add clippy
```

執行：

```shell
$ cargo clippy
```

### Cargo

https://crates.io/

Cargo 是 Rust 的套件管理工具，用於下載依賴，編譯套件及建立可供發行的套件。

## 整合開發環境（IDE）

### IntelliJ Rust

https://intellij-rust.github.io/

IntelliJ IDE 的 Rust 語言外掛。推薦在 [CLion](https://www.jetbrains.com/clion/) 上安裝，可以獲得更多的功能，例如完整的除錯工具、CPU profiler 與 Valgrind memcheck。

- [Features](https://intellij-rust.github.io/features/)
- [Quick start guide](https://intellij-rust.github.io/docs/quick-start.html)

## 參考資料

- [Tools](https://www.rust-lang.org/tools)
- [Useful Development Tools](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html)
- [Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
