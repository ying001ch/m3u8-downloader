use core::panic;

mod Manager;
mod http_util;
mod M3u8Item;
mod aes_demo;
mod combine;
mod str_util;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    if args[1] == "--combine"{
        if args.len() < 3{
            panic!("合并操作还需要指定片段存放目录；m3u8-downloader --combine /clip_path");
        }
        combine::combine_clip(&args[2]).expect("combine failed");
        return;
    }
    
    Manager::run();
}
