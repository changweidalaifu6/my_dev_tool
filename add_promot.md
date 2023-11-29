## 背景
在上一个教程里，教了如何实现一个命令行，以及命令行支持cargo安装。

但是这个命令行工具，在使用的时候有个致命的问题，就是没有命令提示。

比如我要执行`my_dev_tool`，按`tab`键我看不到任何命令的提示，这样太不人性化了。

## 命令补全原理

用clap的生成的命令工具可以通过`clap_complete`包生成一个补全脚本，把这个脚本加到bash环境就可以自动提示了。

为了生成这些脚本，并让系统能自动提示命令参数，你需要按以下步骤：


## 1. 修改依赖关系
新增两个依赖`clap_complete`和`dirs`。
1. `clap_complete`是用来生成补全脚本的。
2. `dirs`是用来定位用户目录的。
```
[dependencies]
clap_complete = "4.4.0"
dirs = "4.0"
```


## 2. 在你的 Rust 程序中生成补全脚本

首先，你需要修改你的 Rust 程序，使其能够生成相应的补全脚本。这可以在程序的一个特定命令或选项下实现。

### 1、重构buildCommand方法
把命令构造的内容单独重构为一个方法，后面生成补全命令还需要这个。并添加一个添加bash脚本的命令`add-completion`。
```rust
use clap::{Command, generate};
use clap_complete::{shells::{Bash, Zsh}, generate_to};
use std::env;
use std::io;
fn build_cli() -> Command {
    Command::new("my_dev_tool")
        .version("1.0")
        .author("tommy <mcg91881127@163.com>")
        .about("Developer's tool for urlencode and time format!")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // 省略。。。。
        .subcommand(Command::new("add-completion")
            .about("Generates completion scripts for your shell"))
}
```
### 2. 生成补全脚本
这一步主要是生成补全命令的脚本，bash或者zsh脚本是根据这个脚本进行命令提示的。

```rust
fn add_completion(matches: &ArgMatches){
    let mut app = build_cli();
    let shell = env::var("SHELL").unwrap_or_default();
    let home_dir = dirs::home_dir().expect("Could not find the home directory");
    let config_file;
    if shell.contains("zsh") {
        config_file = home_dir.join(".zshrc");
        let _ = generate_to(Zsh, &mut app, "my_dev_tool", "./").expect("generate_to failed");
        println!("Generated Zsh completion script.");
    } else {
        config_file = home_dir.join(".bashrc");
        // 默认生成 Bash 补全脚本
        let _ = generate_to(Bash, &mut app, "my_dev_tool", "./").expect("generate_to failed");
        println!("Generated Bash completion script.");
    }
    let completion_script_path = PathBuf::from("./_my_dev_tool");

    let _ = add_completion_to_shell(&config_file, &completion_script_path);
}
```

在上面的代码中，我们添加了一个名为 `add-completion` 的子命令，当运行此命令时，程序将生成 Bash 的自动补全脚本。这里需要注意的是：
1. `generate_to(Bash, &mut app, "my_dev_tool", "./")`这行代码里的`my_dev_tool`，是你要补全命令的名字，比如你的工具叫啥这里就填啥。我这个工具命令就是`my_dev_tool`，所以这里传`my_dev_tool`。
2. 这里生成的自动补全脚本的名字是`_my_dev_tool`，这个是默认的命名规则，生成的补全脚本叫“_”+“命令名”。
3. 为了程序能够区分并生成 `.bashrc` 或 `.zshrc` 的补全命令，你需要确定用户正在使用的是哪种 shell。这可以通过检查环境变量 `SHELL` 来实现，然后根据这个环境变量的值决定修改 `.bashrc` 或 `.zshrc`。

### 3. 将生成的补全脚本添加到你的 shell 配置中

生成的补全脚本需要被 source（或等效地添加）到你的 shell 配置文件中（例如 `.bashrc`, `.zshrc` 等），这样你的 shell 就能够利用这些脚本进行命令补全。有两种方式可以实现：

#### 第一种，手动添加
将以下行添加到 `.bashrc` 或 `.bash_profile`，zsh为`.zshrc`。

```shell
source ~/_my_dev_tool
```

然后，重新加载配置文件，zsh为`source ~/.zshrc`：

```shell
source ~/.bashrc
```

或者重新启动你的终端。


#### 第二种，自动添加到bash或者zsh

然后，使用以下 Rust 代码来自动化添加 shell 配置的过程：

```rust
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn add_completion_to_shell(config_file: &PathBuf, completion_script_path: &PathBuf) -> std::io::Result<()> {
    let completion_script_str = format!("source {}", completion_script_path.display());
    
    let mut config = OpenOptions::new().append(true).open(config_file)?;

    if fs::read_to_string(config_file)?.contains(&completion_script_str) {
        println!("Completion script already added to {}.", config_file.display());
    } else {
        config.write_all(completion_script_str.as_bytes())?;
        println!("Added completion script to {}.", config_file.display());
    }

    Ok(())
}
```
为了程序能够区分并自动向 `.bashrc` 或 `.zshrc` 添加 shell 配置，你需要确定用户正在使用的是哪种 shell。这可以通过检查环境变量 `SHELL` 来实现，然后根据这个环境变量的值决定修改 `.bashrc` 或 `.zshrc`。以下是一个示例实现：
### 4. source一下命令行或者重启命令行
```
### zsh执行
source ~/.zshrc
### bash执行
source ~/bashrc
```

## 展示成果

在命令行输入 `my_dev_tool`后，按tab键会提示所有命令。

```
~ % my_dev_tool time
add-completion  -- Generates completion scripts for your shell
help            -- Print this message or the help of the given subcommand(s)
timestamp       -- Convert a UNIX timestamp to local datetime
urldecode       -- URL-decode a string
urlencode       -- URL-encode a string
```

