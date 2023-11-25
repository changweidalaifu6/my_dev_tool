## 手撸一个命令行工具
本项目会使用 Rust 和 `clap` 4.4.0 创建一个命令行工具 `my_dev_tool`，先实现 urlencode、urldecode 和时间戳转换为本地时间三个功能。如果你也想实现一个自己的命令行工具，可以按照以下步骤进行：

### 第 1 步：创建项目并添加依赖

1. **创建新的 Rust 项目**：在终端中运行以下命令：

    ```bash
    cargo new my_dev_tool
    cd my_dev_tool
    ```

2. **添加依赖**：在 `Cargo.toml` 中添加 `clap`、`serde`、`serde_json` 和 `chrono` 作为依赖：

    ```toml
    [dependencies]
    clap = "4.4.0"
    chrono = "0.4"
    urlencoding = "2.1"
    ```

### 第 2 步：编写代码

在 `src/main.rs` 中，使用 `clap` 定义命令行参数并实现功能。

```rust
use clap::{Arg, ArgMatches, Command};
use chrono::{Local, TimeZone};
use urlencoding::{decode, encode};

fn main() {
    let matches = Command::new("my_dev_tool")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Developer's tool for urlencode and time format!")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("urlencode")
                .about("URL-encode a string")
                .arg(Arg::new("input").help("String to encode").required(true)),
        )
        .subcommand(
            Command::new("urldecode")
                .about("URL-decode a string")
                .arg(Arg::new("input").help("String to decode").required(true)),
        )
        .subcommand(
            Command::new("timestamp")
                .about("Convert a UNIX timestamp to local datetime")
                .arg(Arg::new("timestamp").help("UNIX timestamp").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("urlencode", sub_matches)) => url_encode(sub_matches),
        Some(("urldecode", sub_matches)) => url_decode(sub_matches),
        Some(("timestamp", sub_matches)) => convert_timestamp(sub_matches),
        _ => unreachable!(),
    }
}

fn url_encode(matches: &ArgMatches) {
    if let Some(input) = matches.get_one::<String>("input") {
        println!("{}", encode(input));
    }
}

fn url_decode(matches: &ArgMatches) {
    if let Some(input) = matches.get_one::<String>("input") {
        println!("{}", decode(input).unwrap());
    }
}

fn convert_timestamp(matches: &ArgMatches) {
    if let Some(timestamp_str) = matches.get_one::<String>("timestamp") {
        let timestamp = timestamp_str.parse::<i64>().unwrap();
        let datetime = Local.timestamp_opt(timestamp, 0).unwrap();
        println!("{}", datetime.to_rfc3339());
    }
}
```

### 第 3 步：编译和安装

1. **编译项目**：

    ```bash
    cargo build --release
    ```

2. **安装到系统**：

    - 在 Linux 或 macOS 上，您可以将编译后的可执行文件复制到 `/usr/local/bin` 或其他 PATH 包含的目录：

      ```bash
      sudo cp target/release/my_dev_tool /usr/local/bin/
      ```

    - 在 Windows 上，您可以将可执行文件复制到任何 PATH 包含的目录，或者手动添加其所在目录到系统 PATH。

### 第 4 步：使用工具

一旦安装，您就可以直接在命令行中使用 `my_dev_tool`，例如：

```bash
my_dev_tool urlencode "https://example.com"
my_dev_tool urldecode "https%3A%2F%2Fexample.com"
my_dev_tool timestamp 1609459200
```

### 第5步，支持cargo安装

要使您的 `my_dev_tool` 命令行工具能够通过 `cargo install` 安装，您需要将其发布到 [crates.io](https://crates.io/)，这是 Rust 的包管理仓库。在发布之前，您需要创建一个帐户并获取一个 API 令牌用于身份验证。以下是将您的工具准备并发布到 crates.io 的步骤：

#### 第（1）步：注册 crates.io 帐户

1. 访问 [crates.io](https://crates.io/) 并注册一个帐户。
2. 登录后，在 "Account Settings" 中获取您的 API 令牌。
3. 验证自己的邮箱，邮箱只有验证成功才可以publish包。

#### 第（2）步：登录 Cargo

在您的终端中，使用以下命令登录 Cargo：

```bash
cargo login [your_api_token]
```

将 `[your_api_token]` 替换为您在 crates.io 上的 API 令牌。

#### 第（3）步：准备发布

确保您的 `Cargo.toml` 文件包含所有必要的信息，这对于发布至 crates.io 是必要的。下面是一个示例：

```toml
[package]
name = "my_dev_tool"
version = "0.1.0"
authors = ["Your Name <youremail@example.com>"]
edition = "2018"

# 以下是描述和文档链接等可选字段
description = "A useful development tool for various tasks"
documentation = "https://example.com/my_dev_tool/docs"
license = "MIT OR Apache-2.0"

[dependencies]
clap = "3.0"
chrono = "0.4"
urlencoding = "2.1"
```

确保更新 `authors`、`description`、`documentation`（如果适用），以及任何其他相关信息。

#### 第（4）步：发布到 crates.io

在您的项目目录中运行以下命令来发布您的包：

```bash
cargo publish
```

这将会把您的包上传到 crates.io。

### 第6步：通过 Cargo 安装

一旦您的包被成功发布到 crates.io，其他人就可以通过运行以下命令来安装您的工具：

```bash
cargo install my_dev_tool
```

### 注意事项

- 在发布之前，请确保您的代码和文档是清晰和完整的，这对于其他人使用您的工具非常重要。
- 您可能需要更新版本号（在 `Cargo.toml` 中的 `version` 字段）每次您想发布新的更改。
- 如果您的工具包含敏感或专有信息，请在发布前仔细检查。

发布到 crates.io 是 Rust 生态系统中分享和分发代码的标准方式，它使其他人能够轻松安装并使用您的工具。如果您在发布过程中遇到任何问题，请随时告诉我。