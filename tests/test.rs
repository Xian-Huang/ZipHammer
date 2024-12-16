#[cfg(test)]
mod tests {
    use std::{fs::File, path::{self, Path}, sync::Arc};

    use tokio::sync::Mutex;
    use zip::ZipArchive;
    use ZipHammer::{create_archive, get_passwordconfig, hammer, password::{PasswordConfig, PasswordCreater}, try_hammer, wordtype::{self, WordType}};

    #[tokio::test]
    async fn test_hammer() {
        let path ="./test.zip";
        // 根据参数生成密码配置
        let passwordconfig = PasswordConfig{ types: vec![WordType::Letter,WordType::Number], capital: true, min_length: 6, max_length: 6 };

        // 根据配置生成密码创建期
        let passwordcreater: PasswordCreater = PasswordCreater::new(&passwordconfig);
        let mutex_pwdc = Arc::new(Mutex::new(passwordcreater));

        // 引入tokio
        let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .build()
            .unwrap();
        let passwords_arc = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::new();
        // let semaphore = Arc::new(Semaphore::new(3));
        loop {
            let mut archive: ZipArchive<File> = match create_archive(Path::new(path)) {
                Ok(f) => f,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            let config = passwordconfig.clone();
            let mutext_pwdc_clone = mutex_pwdc.clone();
            let pwda = Arc::clone(&passwords_arc);
            // let permit = semaphore.clone().acquire_owned().await.unwrap();
            let handle = runtime.spawn(async move {
                try_hammer(&mut archive, &pwda, &config, &mutext_pwdc_clone);
            });
            handles.push(handle);
        }
    }
}
