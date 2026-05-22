use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use strum::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Color, Style,
        object::{Columns, Rows},
    },
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Best Ls  command ever")]
struct Cli {
    path: Option<PathBuf>,
    #[arg(short, long, help = "Output in JSON format")]
    json: bool,
}
// 宏 Display 使得，枚举 直接输出为 字符串
#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    entry_type: EntryType,
    #[tabled(rename = "Size B")]
    len_bytes: u64,
    modified_timestamp: String,
}
// 优化为 函数式
fn get_files(path: &Path) -> Vec<FileEntry> {
    fs::read_dir(path) // // 读取目录
        .ok() // 把 Result 转成 Option，错误直接忽略
        .into_iter() // 让空的也能迭代
        .flatten() // 嵌套的 多层结构 展平目录项
        //filter_map 过滤掉 None
        .filter_map(|entry| {
            let file_entry = entry.ok()?; //   错误的直接 return 掉过迭代
            let file_name = file_entry.file_name().into_string().ok()?;

            let metadata = file_entry.metadata().ok()?; // 失败的 返回 None

            Some(FileEntry {
                name: file_name,
                //  entry_type: if file_entry.file_type().ok()?.is_dir() {
                entry_type: if metadata.is_dir() {
                    EntryType::Dir
                } else {
                    EntryType::File
                },
                len_bytes: metadata.len(),
                // modified_timestamp: "".to_string(),
                modified_timestamp: if let Ok(time) = metadata.modified() {
                    let date: DateTime<Utc> = time.into();
                    format!("{}", date.format("%Y-%m-%d %H:%M:%S"))
                } else {
                    "".to_string()
                },
            })
        })
        .collect()
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.unwrap_or(PathBuf::from("."));

    // 提前退出
    if let Ok(false) = fs::exists(&path) {
        println!("{}", "Path does not exist".red());
        return;
    }
    // 继续正常流程

    let file_entries = get_files(&path);
    if cli.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&file_entries).unwrap_or("cannot parse json".to_string())
        );
    } else {
        let mut table = Table::new(file_entries);
        table.with(Style::rounded());
        table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
        table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
        table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
        table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);

        println!("{}", table);
    }
}
