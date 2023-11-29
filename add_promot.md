在上一个教程里，教了如何实现一个命令行，以及命令行支持cargo安装。

但是这个命令行工具，在使用的时候有个致命的问题，就是没有提示。

比如我要执行my_dev_tool，按tal键我看不到任何命令的提示，这样太不人性化了。

### 命令补全原理

clap的命令可以通过clap_complete包生成一个补全脚本，把这个脚本加到bash环境就可以自动提示了。

为了生成这些脚本，并让系统能自动提示命令参数，你需要按以下步骤：


### 1. 修改依赖关系
新增两个两个clap_complete和dirs。
```
[dependencies]
clap_complete = "4.4.0"
dirs = "4.0"
```


### 2. 在你的 Rust 程序中生成补全脚本

首先，你需要修改你的 Rust 程序，使其能够生成相应的补全脚本。这可以在程序的一个特定命令或选项下实现。

#### 示例代码

```rust
use clap::{Command, generate};
use clap_complete::{shells::{Bash, Zsh}, generate_to};
use std::env;
use std::io;

fn build_cli() -> Command<'static> {
    Command::new("myapp")
        .version("1.0")
        .about("Does awesome things")
        // ... 更多的命令和参数 ...
}

fn main() -> io::Result<()> {
    let mut app = build_cli();
    let shell = env::var("SHELL").unwrap_or_default();

    if shell.contains("zsh") {
        generate_to(Zsh, &mut app, "myapp", "./")?;
        println!("Generated Zsh completion script.");
    } else {
        // 默认生成 Bash 补全脚本
        generate_to(Bash, &mut app, "myapp", "./")?;
        println!("Generated Bash completion script.");
    }

    Ok(())
}

```

在这个示例中，我们添加了一个名为 `generate-completions` 的子命令，当运行此命令时，程序将生成 Bash 的自动补全脚本。

### 3. 把补全脚本添加到环境

在命令行中运行你的程序，并使用刚刚添加的命令生成补全脚本。

```bash
./myapp generate-completions
```

### 3. 将生成的补全脚本添加到你的 shell 配置中

生成的补全脚本需要被 source（或等效地添加）到你的 shell 配置文件中（例如 `.bashrc`, `.zshrc` 等），这样你的 shell 就能够利用这些脚本进行命令补全。

#### 对于 Bash 用户

将以下行添加到 `.bashrc` 或 `.bash_profile`：

```bash
source /path/to/your/bash_completion_script
```

然后，重新加载配置文件：

```bash
source ~/.bashrc
```

或者重新启动你的终端。

### 注意事项

- 确保你为正确的 shell 生成补全脚本（在上面的例子中，我们为 Bash 生成了脚本）。
- 生成的补全脚本文件需要正确地添加到你的 shell 配置中，以便 shell 能够使用它。
- 如果你的程序频繁更新，你可能需要定期重新生成这些脚本，以反映新的命令和选项。
- `clap` 的自动补全功能依赖于你如何设置命令行参数和子命令，确保你的 `clap` 配置准确地反映了你的程序的功能。

为了程序能够区分并自动向 `.bashrc` 或 `.zshrc` 添加 shell 配置，你需要确定用户正在使用的是哪种 shell。这可以通过检查环境变量 `SHELL` 来实现，然后根据这个环境变量的值决定修改 `.bashrc` 或 `.zshrc`。以下是一个示例实现：

### Rust 示例代码

首先，确保你的 `Cargo.toml` 包含所需的依赖：

```toml
[dependencies]
dirs = "4.0"
```

然后，使用以下 Rust 代码来自动化添加 shell 配置的过程：

```rust
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let home_dir = dirs::home_dir().expect("Could not find the home directory");

    let shell = env::var("SHELL").unwrap_or_default();
    let config_file = if shell.contains("zsh") {
        home_dir.join(".zshrc")
    } else {
        // 默认为 bash
        home_dir.join(".bashrc")
    };

    let completion_script_path = PathBuf::from("myapp-completion.bash");

    if let Err(e) = add_completion_script_to_shell_config(&config_file, &completion_script_path) {
        eprintln!("Failed to add completion script: {}", e);
    }
}

fn add_completion_script_to_shell_config(config_file: &PathBuf, completion_script_path: &PathBuf) -> std::io::Result<()> {
    let completion_script_str = format!("source {}", completion_script_path.display());
  
    let mut config = OpenOptions::new().append(true).open(config_file)?;

    if fs::read_to_string(config_file)?.contains(&completion_script_str) {
        println!("Completion script already added to {}.", config_file.display());
    } else {
        writeln!(config, "\n{}", completion_script_str)?;
        println!("Added completion script to {}.", config_file.display());
    }

    Ok(())
}
```

### 展示成果

在命令行输入 `my_dev_tool`后，按tab键会提示所有命令。

```
~ % my_dev_tool time
add-completion  -- Generates completion scripts for your shell
help            -- Print this message or the help of the given subcommand(s)
timestamp       -- Convert a UNIX timestamp to local datetime
urldecode       -- URL-decode a string
urlencode       -- URL-encode a string
```

### 说明

- 该程序首先使用 `dirs::home_dir()` 来获取用户的家目录。
- 使用 `std::env::var("SHELL")` 获取当前 shell 的类型。如果 `SHELL` 环境变量包含 "zsh"，程序假定用户使用的是 Zsh，并选择 `.zshrc` 作为配置文件；否则，默认为 Bash，并选择 `.bashrc`。
- 函数 `add_completion_script_to_shell_config` 负责向指定的 shell 配置文件中添加补全脚本。

### 安全考虑

- 自动修改用户的配置文件应谨慎进行，并且最好在用户明确同意的情况下执行。
- 这段代码假设了 `.bashrc` 和 `.zshrc` 的位置，但在某些环境下，这些文件的位置可能有所不同。
- 确保在修改任何配置文件之前通知用户并获得其同意。
