use std::collections::btree_map::Range;

use crate::wordtype::WordType;

// 定义密码生成规则
#[derive(Default)]
pub struct PasswordConfig {
    /// 密码包含的元素类型
    pub types: Vec<WordType>,

    /// 是否包含大写字母
    pub capital: bool,

    /// 密码最小长度
    pub min_length: u32,

    /// 密码最大长度
    pub max_length: u32,
}


impl PasswordConfig{
    pub fn get_pwd_len(self:&Self)->u8{
        (self.max_length-self.min_length).try_into().unwrap()
    }
}

struct Password{
    value:Vec<u8>
}

// 密码生成器
pub struct PasswordCreater {
    /// 密码（btype）
    password: Vec<Password>,

    /// 密码配置
    config: PasswordConfig,
}

impl PasswordCreater {
    pub fn new(config: PasswordConfig) -> Self {
        let length = config.get_pwd_len();
        let wt = WordType::create_wordtypes(length, config); // 根据给定参数设置密码格式
        let password: Vec<u8> = Vec::new();
        let mut password_wt = Vec::new();
        for i in 0..length {
            let select_wt = *wt.get(i as usize).unwrap();
            let util = select_wt.create_until();
            password_wt.push(select_wt);
        }

        PasswordCreater {
            password: Vec::new(),
            config: PasswordConfig::default(),
        }
    }

    fn create_password_set() {}
}
