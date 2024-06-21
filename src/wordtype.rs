use rand::{self, Rng};
use crate::password::PasswordConfig;

#[derive(Clone, Copy)]
pub enum WordType {
    Number,
    Letter,
    Special,
}

impl WordType {
    // pub fn create_wordtypes(password_length: u8,pconfig:PasswordConfig) -> Vec<WordType> {
    //     /*
    //        创建密码格式
    //     */
    //     let mut wordtypes_res: Vec<WordType> = Vec::new();
    //     let wordtypes: Vec<WordType> = pconfig.types;
    //     for _ in 0..password_length {
    //         let rand_seek = wordtypes.len();
    //         let select = rand::thread_rng().gen_range(0..rand_seek);
    //         if let Some(wt) = wordtypes.get(select) {
    //             wordtypes_res.push(*wt);
    //         };
    //     }
    //     wordtypes_res
    // }

    pub fn create_until(self: &Self) -> u8 {
        // 创建密码元素
        match self {
            WordType::Number => self.create_number(),
            WordType::Letter => todo!(),
            WordType::Special => todo!(),
        }
    }

    pub fn create_number(self: &Self) -> u8 {
        rand::thread_rng().gen_range(0..=9)
    }

    pub fn create_letter(self: &Self, b:bool) -> u8 {
        /*
           p:是否包含大小写
         */
        if b {
            if rand::thread_rng().gen_range(1..10) % 2 == 0 {
                self.create_upper_letter()
            } else {
                self.create_low_letter()
            }
        } else {
            self.create_low_letter()
        }
    }

    pub fn create_low_letter(self: &Self) -> u8 {
        rand::thread_rng().gen_range(97..=122)
    }

    pub fn create_upper_letter(self: &Self) -> u8 {
        rand::thread_rng().gen_range(65..=90)
    }

    pub fn create_special(self: &Self) -> u8 {
        todo!("创建特殊字符元素")
    }
}
