/*
 Project:ZipHammer
 Author:@LiDingyiii
*/

/*
  TODO 引进tokio 实现多线程尝试密码
*/
use clap::Parser;
use core::panic;
use std::io;
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

        println!("TRY TO APPLY PASSWORD {:?}:{:?} {:?}", password,password.as_bytes(),"123456".as_bytes());
        let file = archive.by_index_decrypt(0, password.as_bytes());
        match file {
            Ok(f) => {
                dbg!(f.enclosed_name());
                break;
            },
            Err(_) => {},
        }
        // let mut outfile = File::create("./res.md").unwrap();
        // if let Ok(mut f) = file {
        //     println!("RIGHT PASSWORD=>{}\nFIRST FILE:{}", password, f.name());
        //     io::copy(&mut f, &mut outfile).unwrap();
        //     break;
        // }
    }
 
}
