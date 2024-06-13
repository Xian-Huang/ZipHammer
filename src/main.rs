use std::{env, fs::File};
use clap::{command, Parser};



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 文件路径
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args_matcher = Args::parse();
    dbg!(args_matcher);
}
