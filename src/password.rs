use crate::wordtype::WordType;


// 定义密码生成规则
#[derive(Default)]
pub struct PasswordConfig{
    /// 密码包含的元素类型
    pub types: Vec<WordType>,
    /// 是否包含大写字母
    pub is_contain_upper_letter:bool
}

// 密码生成器
struct PasswordCreater {
    /// 密码（btype）
    password: Vec<u8>,

    /// 密码配置
    config:PasswordConfig
}

impl PasswordCreater {
    fn new(length: u8,config:PasswordConfig) -> Self {
        let wt = WordType::create_wordtypes(length, config); // 根据给定参数设置密码格式
        let password: Vec<u8> = Vec::new();
        let mut password_wt = Vec::new();
        for i in 0..length {
            let select_wt = *wt.get(i as usize).unwrap();
            let util = select_wt.create_until();
            password_wt.push(select_wt);
        }

        PasswordCreater{
            password:Vec::new(),
            config: PasswordConfig::default(),
        }
    }

    fn create_password_set() {}
}