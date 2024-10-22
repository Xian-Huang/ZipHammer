#[cfg(test)]
mod tests {

    use crate::{hammer, Args};

    #[test]
    fn try_hammer() {
        let args = Args {
            path: String::from("./test.zip"),
            length: 6,
            min_length: 6,
            max_length: 6,
            number: true,
            isletter: true,
            capital: true,
            special: false,
            thread_count: 4,
        };
        hammer("./test.zip".to_string(),&args);
    }
}
