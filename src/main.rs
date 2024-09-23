/*
 Project:ZipHammer
 Author:@LiDingyiii
*/
/*
  TODO 引进tokio 实现多线程尝试密码
*/
//slint::include_modules!();

use clap::Parser;
use ZipHammer::{hammer, Args};

fn main() {
    let args_matcher: &Args = &Args::parse();
    let path = args_matcher.path.clone();
    println!("file path: {}",path);
    println!("args: {:?}",args_matcher);
    let _ =hammer(path, args_matcher);
}



// fn main() {
    // let mainwindow = MainWindow::new().unwrap();

    // #[warn(unused_must_use)]
    // mainwindow.run();

// }
