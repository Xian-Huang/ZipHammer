/*
 Project:ZipHammer
 Author:@LiDingyiii
*/

/*
  TODO 引进toko 实现多线程尝试密码
*/
use clap::Parser;
use core::panic;
use std::{fs::File, path::Path};
use zip::ZipArchive;
use ZipHammer::Args;
use ZipHammer::{create_pwds, get_passwordconfig, password::PasswordCreater};

fn create_archive(path: &Path) -> Result<ZipArchive<File>, String> {
    let file = File::open(path);
    let archive = zip::ZipArchive::new(file.unwrap()).unwrap();
    Ok(archive)
}

fn main() {
    let args_matcher: &Args = &Args::parse();

    let path = args_matcher.path.clone();

    let mut archive = match create_archive(Path::new(path.as_str())) {
        Ok(f) => f,
        Err(e) => {
            panic!("{}", e);
        }
    };

    // 根据参数生成密码配置
    let passwordconfig: ZipHammer::password::PasswordConfig = match get_passwordconfig(args_matcher) {
        Ok(config) => config,
        Err(e) => {
            panic!("PasswordConfig Created Fail");
        }
    };

    // 根据配置生成密码本
    let passwordcreater = PasswordCreater::new(&passwordconfig);

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
