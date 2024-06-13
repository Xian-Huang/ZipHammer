use clap::{command, Parser};
use std::{
    fs::File, path::Path
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 文件路径
    #[arg(short, long)]
    path: String,

    /// 是否暴力破解
    #[arg(short, long, default_value_t = false)]
    e: bool,
}

fn main() {
    let args_matcher = Args::parse();

    let path = Path::new("test.zip");
    let file = File::open(path);
    let archive = zip::ZipArchive::new(file.unwrap()).unwrap();
    
}
