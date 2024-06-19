/*
 Project:ZipHammer
 Author:@LiDingyiii
*/

/*
  TODO 引进toko 实现多线程尝试密码
*/
use clap::{command, Parser};
use ZipHammer::create_pwds;
use core::panic;
use std::{fs::File, path::Path};
use zip::ZipArchive;





#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 文件路径
    #[arg(short, long)]
    path: String,

    /// 密码中是否包含数字[0-9],默认包含
    #[arg(short, long, default_value_t = true)]
    number: bool,

    // 密码中是否包含字母[a-z],默认包含
    #[arg(short, long, default_value_t = true)]
    letter: bool,

    // 字母是否开启大小写
    #[arg(short, long, default_value_t = true)]
    capital: bool,
}

fn create_archive(path: &Path) -> Result<ZipArchive<File>, String> {
    let file = File::open(path);
    let archive = zip::ZipArchive::new(file.unwrap()).unwrap();
    Ok(archive)
}



fn main() {
    let args_matcher = Args::parse();

    let path = args_matcher.path;

    let mut archive = match create_archive(Path::new(path.as_str())) {
        Ok(f) => f,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let passwords = create_pwds(10).unwrap();

    let mut progress_sum = passwords.len();
    let mut current_progress = 0;

    for password in passwords {
        println!("TRY TO APPLY PASSWORD {password:20} progress:{current_progress}/{progress_sum}");
        let file = archive.by_index_decrypt(0, password.as_bytes());
        if let Ok(_) = file {
            println!("RIGHT PASSWORD=>{}", password);
        }
        current_progress += 1;
    }
}
