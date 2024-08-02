use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
    path::Path,
    time::Instant,
};

use crate::error::ArgError;
use clap::Parser;
use password::{PasswordConfig, PasswordCreater};
use rand::Rng;
use tokio::sync::broadcast;
use wordtype::WordType;
use zip::ZipArchive;

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

fn create_archive(path: &Path) -> Result<ZipArchive<File>, String> {
    let file = File::open(path);
    let archive = zip::ZipArchive::new(file.unwrap()).unwrap();
    Ok(archive)
}

pub fn hammer(path: String, args: &Args) {
    let start = Instant::now();

    let mut archive = match create_archive(Path::new(path.as_str())) {
        Ok(f) => f,
        Err(e) => {
            panic!("{}", e);
        }
    };

    // 根据参数生成密码配置
    let passwordconfig: password::PasswordConfig = match get_passwordconfig(args) {
        Ok(config) => config,
        Err(_) => {
            panic!("PasswordConfig Created Fail");
        }
    };

    // 根据配置生成密码本
    let passwordcreater = &PasswordCreater::new(&passwordconfig);

    let passwords: &mut Vec<String> = &mut Vec::new();
    // 引入tokio
    let mut rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .build()
        .expect("tokio runtime start fail");

    // TODO 添加结束信号

    loop {
        let result = rt.block_on(async {
            let length =
                rand::thread_rng().gen_range(passwordconfig.min_length..=passwordconfig.max_length);

            let numbers = &passwordcreater.clone().create_password(length);
            let chars: Vec<char> = numbers.iter().map(|&b| b as char).collect();
            let password: String = chars.into_iter().collect();
            // if !passwords.contains(&password.clone()) {
            //     return;
            // } else {
            //     passwords.push(password.clone())
            // }
            println!(
                "TRY TO APPLY PASSWORD {:?}:{:?}:{}",
                password,
                password.as_bytes(),
                passwords.len()
            );
            let file = archive.by_index_decrypt(0, password.as_bytes());
            let outfile = File::create("./res.md").unwrap();
            let mut buffwriter = BufWriter::new(outfile);
            let _file_res: (String, f64) = match file {
                Ok(f) => {
                    dbg!(f.enclosed_name());
                    let mut reader = BufReader::new(f);
                    let res: (String, f64) = match io::copy(&mut reader, &mut buffwriter) {
                        Ok(_) => {
                            let duration = start.elapsed();
                            println!(
                                "RIGHT PASSWORD=>{},Time:{}",
                                password,
                                duration.as_secs_f64()
                            );
                            (password, duration.as_secs_f64())
                        }
                        Err(e) => {
                            let duration = start.elapsed();
                            (e.to_string(), duration.as_secs_f64())
                        }
                    };
                    res
                }
                Err(e) => {
                    (e.to_string(),-1.)
                }
            };
            while let Ok(_) = rx.recv().await {
                // 接收到信号，执行一些清理工作
            }
        });
    }
}
