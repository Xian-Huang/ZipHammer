
/*
 Project:ZipHammer
 Author:@LiDingyiii
*/
/*
  TODO 引进tokio 实现多线程尝试密码
*/
use clap::Parser;
use ZipHammer::{hammer, Args};



fn main() {
    let args_matcher: &Args = &Args::parse();
    let path = args_matcher.path.clone();
    hammer(path, args_matcher); 
}
