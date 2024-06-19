use crate::wordtype::WordType;


// 定义密码生成规则
#[derive(Default)]
struct PasswordConfig{
    /// 密码各元素类型
    types: Vec<WordType>,
    /// 是否包含大写字母
    is_contain_upper_letter:bool
}

// 密码生成器
struct PasswordCreater {
    /// 密码（btype）
    password: Vec<u8>,

    /// 密码配置
    config:PasswordConfig
}

impl PasswordCreater {
    fn new(length: u8, wordtypes: Vec<&WordType>) -> Self {
        let wt = WordType::create_wordtypes(length, wordtypes); // 根据给定参数设置密码格式
        let password: Vec<u8> = Vec::new();
        let mut password_wt = Vec::new();
        for i in 0..length {
            todo!("创建指定长度的密码");
            let select_wt = *wt.get(i as usize).unwrap();
            let util = select_wt.create_until();
            password_wt.push(*select_wt);
        }

        PasswordCreater{
            password:Vec::new(),
            config: PasswordConfig::default(),
        }
    }

    fn create_password_set() {}
}