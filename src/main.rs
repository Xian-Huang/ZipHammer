/*
 Project:ZipHammer
 Author:@LiDingyiii
*/

/*
  TODO 引进toko 实现多线程尝试密码
*/
use clap::{command, Parser};
use rand::Rng;
use core::panic;
use std::{fs::File, path::Path};
use zip::ZipArchive;

enum WordType {
    Number,
    Letter,
    Special,
}




impl WordType {
    fn create_wordtypes(password_length:u8,wordtypes:Vec<&WordType>)->Vec<&WordType>{
        /*
            创建密码格式
         */
        let mut wordtypes_res:Vec<&WordType> = Vec::new();
        for _ in 0..password_length{
            let rand_seek = wordtypes.len();
            let select = rand::thread_rng().gen_range(0..rand_seek);
            if let Some(wt) = wordtypes.get(select){
                wordtypes_res.push(wt);
            };
        }
        wordtypes_res
    }

    fn create_until(self:&Self)->u8{
        // 创建密码元素
        match self {
            WordType::Number => self.create_number(),
            WordType::Letter => todo!(),
            WordType::Special => todo!(),
        }
    }

    fn create_number(self:&Self)->u8{
        rand::thread_rng().gen_range(0..=9)
    }

    fn create_letter()->u8{
        todo!("创建字母元素")
    }

    fn create_special()->u8{
        todo!("创建特殊字符元素")
    }
}

struct PasswordCreater {
    password: Vec<u8>,
    types: Vec<WordType>,
}

impl PasswordCreater {
    fn new(length: u8,wordtypes:Vec<&WordType>) -> Self{
        let wt = WordType::create_wordtypes(length, wordtypes);
        let password:Vec<u8> = Vec::new();
        let mut password_wt = Vec::new();
        for i in 0..length {
            todo!("创建指定长度的密码");
            let select_wt = *wt.get(i as usize).unwrap();
            let util = select_wt.create_until();
            password_wt.push(*select_wt);
        }

        PasswordCreater { password: password, types: password_wt }
    }

    fn create_password_set() {}
}

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

fn create_pwds(length: u8) -> Result<Vec<String>, String> {
    /*
        TODO 根据参数创建密码本
    */

    let mut password_type: Vec<WordType> = Vec::new();

    let mut passwords: Vec<String> = Vec::new();

    for i in 0..length {}

    Ok(passwords)
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

    let passwords = create_pwds().unwrap();

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
