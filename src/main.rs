use clap::{Arg, ArgMatches, Command};
use chrono::{Local, TimeZone};
use urlencoding::{decode, encode};
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
    let matches = Command::new("my_dev_tool")
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
        .get_matches();

    match matches.subcommand() {
        Some(("urlencode", sub_matches)) => url_encode(sub_matches),
        Some(("urldecode", sub_matches)) => url_decode(sub_matches),
        Some(("timestamp", sub_matches)) => convert_timestamp(sub_matches),
        _ => unreachable!(),
    }
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