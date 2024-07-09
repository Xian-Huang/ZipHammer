#[allow(dead_code)]
pub struct ArgError(String);

impl ArgError {
    pub fn new() -> Self {
        ArgError("参数错误！".to_string())
    }
}
