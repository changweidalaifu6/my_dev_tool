use std::{env, path::PathBuf, io::Write};

use clap::{Arg, ArgMatches, Command};
use clap_complete::{shells::{Bash, Zsh}, generate_to};
use chrono::{Local, TimeZone};
use urlencoding::{decode, encode};
use std::fs::{self, OpenOptions};
/// ## my_dev_tool
/// my_dev_tool，是一个简单的开发者命令行工具。
/// 目前支持urlEncode，urlDecode和时间戳转换时间。后续会增加更多功能。
/// 
/// ### 使用方式如下：
/// ```bash
/// my_dev_tool urlencode "https://example.com"
/// my_dev_tool urldecode "https%3A%2F%2Fexample.com"
/// my_dev_tool timestamp 1609459200
/// ```
/// 

fn main() {
    let app = build_cli();

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("urlencode", sub_matches)) => url_encode(sub_matches),
        Some(("urldecode", sub_matches)) => url_decode(sub_matches),
        Some(("timestamp", sub_matches)) => convert_timestamp(sub_matches),
        Some(("add-completion", sub_matches)) => add_completion(sub_matches),
        _ => unreachable!(),
    }
}

fn build_cli() -> Command {
    Command::new("my_dev_tool")
        .version("1.0")
        .author("tommy <mcg91881127@163.com>")
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
        .subcommand(Command::new("add-completion")
                .about("Generates completion scripts for your shell"))
}
/// 进行urlEncode
fn url_encode(matches: &ArgMatches) {
    if let Some(input) = matches.get_one::<String>("input") {
        println!("{}", encode(input));
    }
}

/// 进行urlDecode
fn url_decode(matches: &ArgMatches) {
    if let Some(input) = matches.get_one::<String>("input") {
        println!("{}", decode(input).unwrap());
    }
}

/// 对时间戳进行转换
fn convert_timestamp(matches: &ArgMatches) {
    if let Some(timestamp_str) = matches.get_one::<String>("timestamp") {
        let timestamp = timestamp_str.parse::<i64>().unwrap();
        let datetime = Local.timestamp_opt(timestamp, 0).unwrap();
        println!("{}", datetime.to_rfc3339());
    }
}
fn add_completion(matches: &ArgMatches){
    let mut app = build_cli();
    let shell = env::var("SHELL").unwrap_or_default();
    let home_dir = dirs::home_dir().expect("Could not find the home directory");
    let config_file;
    if shell.contains("zsh") {
        config_file = home_dir.join(".zshrc");
        let _ = generate_to(Zsh, &mut app, "my_dev_tool_completion", "~/");
        println!("Generated Zsh completion script.");
    } else {
        config_file = home_dir.join(".bashrc");
        // 默认生成 Bash 补全脚本
        let _ = generate_to(Bash, &mut app, "my_dev_tool_completion", "~/");
        println!("Generated Bash completion script.");
    }
    let completion_script_path = PathBuf::from("~/my_dev_tool_completion");

    let _ = add_completion_to_shell(&config_file, &completion_script_path);
}
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