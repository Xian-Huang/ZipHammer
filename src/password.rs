use std::{collections::btree_map::Range, fmt::format, fs::{DirBuilder, File}, io::{self, BufWriter, Write}, path::Path, vec};

use rand::{distributions::Alphanumeric, Rng};

use crate::wordtype::{self, WordType};

// 定义密码生成规则
#[derive(Default, Clone)]
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

impl PasswordConfig {
    pub fn get_pwd_len(self: &Self) -> u8 {
        (self.max_length - self.min_length).try_into().unwrap()
    }
}

#[derive(Clone)]
struct Password {
    value: Vec<u8>,
}

impl Password {
    pub fn get_bpwd(self: &Self) -> Vec<u8> {
        self.value.clone()
    }

    pub fn get_pwd(self: &Self) -> String {
        todo!("将btype转换为String");
    }
}

// 密码生成器
#[derive(Clone)]
pub struct PasswordCreater {
    /// 密码（btype）
    passwords: Vec<Password>,

    /// 密码配置
    config: PasswordConfig,
}

impl PasswordCreater {
    pub fn new(config: &PasswordConfig) -> Self {
        PasswordCreater {
            passwords: Vec::new(),
            config: config.clone(),
        }
    }

    fn create_rand_filename(self:&Self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    }


    fn create_password_file(self:&Self)->Result<File,io::Error>{

        let tmp_path = Path::new("./tmp");
        if !tmp_path.exists(){
            match  DirBuilder::new().create(tmp_path){
                Ok(_) => {
                    println!("{} created",tmp_path.to_str().unwrap());
                },
                Err(_) => {
                    panic!("tmp dir created fail");
                },
            }
        }

        let mut filenmame = self.create_rand_filename();

        // 检查文件是否存在 存在则重新生成文件名称
        while tmp_path.join(filenmame.clone()).exists(){
            filenmame = self.create_rand_filename();
        };

        let file_path = tmp_path.join(filenmame);
       

        let file = match File::open(file_path.clone()){
            Ok(f) => {
                println!("{} created",file_path.to_str().unwrap());
                f
            },
            Err(_) => {
                panic!("{} created fail",file_path.to_str().unwrap());
            },
        };

        Ok(file)
    }

    fn create_password(length:u32,wts:&Vec<WordType>)->Vec<u8>{
        let mut password = Vec::new();
        for index in 0..length{
            let wt = wts.get(index as usize).unwrap();
            password.push(wt.create_until());
        }
        password
    }

    fn create_pwdtypes(length:u32,wts:&Vec<WordType>)->Vec<WordType>{
        // 生成指定长度 密码格式Vec
        let mut new_wrodtypes = Vec::new();
        for i in 0..length{
            let select = rand::thread_rng().gen_range(0..wts.len());
            new_wrodtypes.push(*wts.get(select).unwrap());
        }
        new_wrodtypes
    }

    

    fn create_password_set(self: &Self) {
        /// 生成密码集合
        /// ! 存在问题，不能直接生成到内存中，否则占用内存过高,暂时写入中间文件中
        
        // 生成文件
        let file = self.create_password_file().unwrap();
        let mut buf = BufWriter::new(file);

        for length in self.config.min_length..=self.config.max_length{
            let wts = self.config.types.clone();
            // 生成长度为length的密码
            
        }

        
    }

    fn get_password_set(self: &Self) -> &Vec<Password> {
        /// 获取密码集合
        &self.passwords
    }
}
