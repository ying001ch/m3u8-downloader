use std::{vec};
use M3u8Item::{DownParam};

mod Manager;
mod http_util;
mod M3u8Item;
mod aes_util;
mod combine;
mod str_util;
mod config;

// #[tokio::main]
fn main() {
  //判断是否使用命令行
  if use_cmd(){
    let param:DownParam = DownParam::from_cmd();
    Manager::dispatch(param,false).unwrap();
    return;
  }
  panic!("缺少参数")
  //启动 Tauri GUI
  // command::start_tauri();
}

fn use_cmd() -> bool {
  let args:Vec<String> = std::env::args().collect();
  args.len() > 1 && (args[1].starts_with("http") || args[1].contains("--combine"))
}