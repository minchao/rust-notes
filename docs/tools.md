---
tags: rust
---

# 工具

## 開發

### rustup

- https://rustup.rs/
- https://rust-lang.github.io/rustup/

Rust 的安裝工具。可以輕鬆地在 stable、beta 與 nightly 版本間切換，並讓 cross-compiling 變得更簡單。

安裝：

```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

更新：

```console
$ rustup self update
```

安裝指定的 Rust 版本：

> ```
> rustup toolchain <channel>[-<date>][-<host>]
>                   
>   <channel>       = stable|beta|nightly|<version>
>   <date>          = YYYY-MM-DD
>   <host>          = <target-triple>
> ```

```console
$ rustup toolchain install nightly
```

設定系統預設的 Rust 版本：

```console
$ rustup default nightly
info: using existing install for 'nightly-x86_64-apple-darwin'
info: default toolchain set to 'nightly-x86_64-apple-darwin'

  nightly-x86_64-apple-darwin unchanged - rustc 1.48.0-nightly (d006f5734 2020-08-28)

$ rustc --version
rustc 1.48.0-nightly (d006f5734 2020-08-28)
```

更新 Rust：

```console
$ rustup update
```

### Compile and run

有時候我們可能會希望能快速編譯並執行某個小 Rust 程式， 但由於 `cargo run` 必須依賴特別的專案結構與 `Cargo.toml` 才能執行，所以必須兩步驟，先使用 `rustc` 編譯出程式的執行檔，然後才能執行它。

一個簡單的方法是，透過一 bash script 來編譯並執行 Rust 程式，例如：

```rust
#!/bin/bash

set -eo pipefail

function compile_and_run() {
  local readonly name="$([ -n "${1}" ] && basename -- "${1}" '.rs')"

  rustc ${1} "${@:2}" \
    && [ -n "${name}" -a -f "${name}" ] \
    && ./${name} \
    && rm ${name}
}

compile_and_run "${@}"
```

參考資料：

- [What is rustc?](https://doc.rust-lang.org/rustc/what-is-rustc.html)
- [A single command to compile and run Rust programs](http://blog.joncairns.com/2015/10/a-single-command-to-compile-and-run-rust-programs/)

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

自動修正編譯器警告的錯誤。

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
