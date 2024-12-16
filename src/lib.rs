use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
    path::Path,
    process::exit,
    sync::{Arc, Mutex},
    thread,
};

use crate::error::ArgError;
use clap::Parser;
use password::{PasswordConfig, PasswordCreater};
use rand::Rng;
use wordtype::WordType;
use zip::ZipArchive;
pub mod tests;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 文件路径
    #[arg(short, long)]
    pub path: String,

    ///密码长度
    #[arg(short, long)]
    pub length: u32,

    ///密码最大长度，设置该参数后必须设置 最小长度 且 length无效
    #[arg(long, required = false, default_value_t = 0)]
    pub min_length: u32,

    ///密码最大长度，设置该参数后必须设置 最大长度 且 length无效
    #[arg(long, required = false, default_value_t = 0)]
    pub max_length: u32,

    /// 密码中是否包含数字[0-9],默认包含
    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    /// 密码中是否包含字母[a-z],默认包含
    #[arg(long, default_value_t = false)]
    pub isletter: bool,

    /// 字母是否开启大小写
    #[arg(short, long, default_value_t = false)]
    pub capital: bool,

    /// 密码中是否包含特殊字符 默认不包含
    #[arg(short, long, default_value_t = false)]
    pub special: bool,

    /// 密码中是否包含特殊字符 默认不包含
    #[arg(short, long, default_value_t = 0)]
    pub thread_count: i32,
}

pub fn get_passwordconfig(args: &Args) -> Result<PasswordConfig, ArgError> {
    // 创建PasswordConfig

    let mut wordtypes = Vec::new();

    if args.isletter {
        wordtypes.push(WordType::Letter);
    }

    if args.number {
        wordtypes.push(WordType::Number);
    }

    if args.special {
        wordtypes.push(WordType::Special);
    }

    if wordtypes.len() <= 0 {
        return Err(ArgError::new());
    }

    let maxl;
    let minl;
    if args.min_length == 0 || args.max_length == 0 {
        maxl = args.length;
        minl = args.length;
    } else {
        maxl = args.max_length;
        minl = args.min_length;
    }

    Ok(PasswordConfig {
        types: wordtypes,
        capital: args.capital,
        min_length: minl,
        max_length: maxl,
    })
}

pub mod error;
pub mod password;
pub mod wordtype;

pub fn create_archive(path: &Path) -> Result<ZipArchive<File>, String> {
    let file = File::open(path);
    let archive = zip::ZipArchive::new(file.unwrap()).unwrap();
    Ok(archive)
}

pub fn try_hammer(
    archive: &mut ZipArchive<File>,
    passwords: &Arc<Mutex<Vec<String>>>,
    passwordconfig: &password::PasswordConfig,
    passwordcreater: &Arc<Mutex<PasswordCreater>>,
) {
    let mut pwds = passwords.lock().unwrap();

    let length =
        rand::thread_rng().gen_range(passwordconfig.min_length..=passwordconfig.max_length);

    let binding = Arc::clone(passwordcreater);
    let pwdc = binding.lock().unwrap();

    let numbers = pwdc.clone().create_password(length);
    let chars: Vec<char> = numbers.iter().map(|&b| b as char).collect();
    let password: String = chars.into_iter().collect();
    if pwds.contains(&password) {
        return;
    } else {
        pwds.push(password.clone());
    }

    let current_thread_id = thread::current().id();
    println!(
        "TRY TO APPLY PASSWORD {:?}:{:?}::{:?},try password count:{}",
        password,
        password.as_bytes(),
        current_thread_id,
        pwds.len()
    );
    let file = archive.by_index_decrypt(0, password.as_bytes());
    let outfile = File::create("./tmp").unwrap();
    let mut buffwriter = BufWriter::new(outfile);
    match file {
        Ok(f) => {
            dbg!(f.enclosed_name());
            let mut reader = BufReader::new(f);
            match io::copy(&mut reader, &mut buffwriter) {
                Ok(_) => {
                    println!("RIGHT PASSWORD=>{}", password);
                    exit(0);
                }
                Err(_) => {}
            };
        }
        Err(_) => {}
    };
}

pub fn hammer(path: String, args: &Args) {
    // 根据参数生成密码配置
    let passwordconfig = &match get_passwordconfig(args) {
        Ok(config) => config,
        Err(_) => {
            panic!("PasswordConfig Created Fail");
        }
    };

    // 根据配置生成密码创建期
    let passwordcreater: PasswordCreater = PasswordCreater::new(passwordconfig);
    let mutex_pwdc = Arc::new(Mutex::new(passwordcreater));

    // 引入tokio
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .build()
        .unwrap();
    let passwords_arc = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    // let semaphore = Arc::new(Semaphore::new(3));
    loop {
        let mut archive: ZipArchive<File> = match create_archive(Path::new(path.as_str())) {
            Ok(f) => f,
            Err(e) => {
                panic!("{}", e);
            }
        };
        let config = passwordconfig.clone();
        let mutext_pwdc_clone = mutex_pwdc.clone();
        let pwda = Arc::clone(&passwords_arc);
        // let permit = semaphore.clone().acquire_owned().await.unwrap();
        let handle = runtime.spawn(async move {
            try_hammer(&mut archive, &pwda, &config, &mutext_pwdc_clone);
        });
        handles.push(handle);
    }
}
