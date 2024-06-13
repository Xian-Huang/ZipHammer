use std::{env, fs::File};
use clap::{command, Parser};



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 文件路径
    #[arg(short, long)]
    path: String,
 
    /// 是否暴力破解
    #[arg(short,long,default_value_t=false)]
    e:bool
}

fn main(){
    let args_matcher = Args::parse();
}
