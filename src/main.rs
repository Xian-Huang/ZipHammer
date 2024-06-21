/*
 Project:ZipHammer
 Author:@LiDingyiii
*/

/*
  TODO 引进tokio 实现多线程尝试密码
*/
use clap::Parser;
use core::panic;
use std::io::{self, BufWriter, Read, Write};
use rand::Rng;
use std::{fs::File, path::Path};
use zip::ZipArchive;
use ZipHammer::Args;
use ZipHammer::{get_passwordconfig, password::PasswordCreater};

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
    let passwordconfig: ZipHammer::password::PasswordConfig = match get_passwordconfig(args_matcher)
    {
        Ok(config) => config,
        Err(_) => {
            panic!("PasswordConfig Created Fail");
        }
    };

    // 根据配置生成密码本
    let passwordcreater = &PasswordCreater::new(&passwordconfig);

    loop {
        let length =
            rand::thread_rng().gen_range(passwordconfig.min_length..=passwordconfig.max_length);

        let numbers = &passwordcreater.clone().create_password(length);
        let chars: Vec<char> = numbers.iter().map(|&b| b as char).collect();
        let password: String = chars.into_iter().collect();

        println!("TRY TO APPLY PASSWORD {:?}", password);
        let file = archive.by_index_decrypt(0, numbers);
        
        let mut outfile = File::create("./res.md").unwrap();
        if let Ok(mut f) = file {
            println!("RIGHT PASSWORD=>{}:{}", password, f.name());
            io::copy(&mut f, &mut outfile);
            break;
        }
    }
    // for length in self.config.min_length..=self.config.max_length{
    //     let wts = self.config.types.clone();
    //     // 生成长度为length的密码
    //     let pwd_counts = length.pow(2);
    //     for _ in 0..pwd_counts{
    //     }
    // }

    // let passwords = create_pwds(10).unwrap();

    // let mut progress_sum = passwords.len();
    // let mut current_progress = 0;

    // for password in passwords {
    //     println!("TRY TO APPLY PASSWORD {password:20} progress:{current_progress}/{progress_sum}");
    //     let file = archive.by_index_decrypt(0, password.as_bytes());
    //     if let Ok(_) = file {
    //         println!("RIGHT PASSWORD=>{}", password);
    //     }
    //     current_progress += 1;
    // }
}
