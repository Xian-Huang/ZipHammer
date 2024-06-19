use crate::error::ArgError;
use clap::Parser;
use password::PasswordConfig;
use wordtype::WordType;

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
    #[arg(short, long)]
    pub min_length: u32,

    ///密码最大长度，设置该参数后必须设置 最大长度 且 length无效
    #[arg(short, long)]
    pub max_length: u32,

    /// 密码中是否包含数字[0-9],默认包含
    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    // 密码中是否包含字母[a-z],默认包含
    #[arg(short, long, default_value_t = true)]
    pub letter: bool,

    // 字母是否开启大小写
    #[arg(short, long, default_value_t = true)]
    pub capital: bool,

    // 密码中是否包含特殊字符 默认不包含
    #[arg(short, long, default_value_t = false)]
    pub special: bool,
}

pub fn create_pwds(length: u8) -> Result<Vec<String>, String> {
    /*
        TODO 根据参数创建密码本
    */

    let mut password_type: Vec<WordType> = Vec::new();

    let mut passwords: Vec<String> = Vec::new();

    for i in 0..length {}

    Ok(passwords)
}

pub fn get_passwordconfig(args: &Args) -> Result<PasswordConfig, ArgError> {
    // 创建PasswordConfig

    let mut wordtypes = Vec::new();

    if args.letter {
        wordtypes.push(WordType::Letter);
    }

    if args.number {
        wordtypes.push(WordType::Number);
    }

    if wordtypes.len() <= 0 {
        return Err(ArgError::new());
    }
    Ok(PasswordConfig {
        types: wordtypes,
        capital: args.capital,
    })
}

pub mod error;
pub mod password;
pub mod wordtype;
