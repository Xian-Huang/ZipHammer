use crate::wordtype::WordType;

struct PasswordCreater {
    password: Vec<u8>,
    types: Vec<WordType>,
}

impl PasswordCreater {
    fn new(length: u8, wordtypes: Vec<&WordType>) -> Self {
        let wt = WordType::create_wordtypes(length, wordtypes);
        let password: Vec<u8> = Vec::new();
        let mut password_wt = Vec::new();
        for i in 0..length {
            todo!("创建指定长度的密码");
            let select_wt = *wt.get(i as usize).unwrap();
            let util = select_wt.create_until();
            password_wt.push(*select_wt);
        }

        PasswordCreater {
            password: password,
            types: password_wt,
        }
    }

    fn create_password_set() {}
}