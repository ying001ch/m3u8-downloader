use reqwest::{IntoUrl, blocking::{Response, get}, header::HeaderMap};
use std::{collections::HashMap, io::{Read, Write}};
use bytes::Bytes;
use reqwest::Proxy;

static mut proxys:String = String::new();
pub fn main() {
    let body = reqwest::blocking
            ::get("https://address");
    let ar = [0];
    let resp = match body {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            panic!("err 發生");
            return;
        }
    };
    write_file(resp);
    println!("读取完毕！");
}
pub fn query_bytes(url: &str) ->std::result::Result<Bytes, reqwest::Error> {
    let client;
    if get_proxy().len()>0{
        let proxy = reqwest::Proxy::all(get_proxy())
                .expect("socks proxy should be there");
        client = reqwest::blocking::Client::builder().proxy(proxy).build()
        .expect("should be able to build reqwest client");
    }else{
        client = reqwest::blocking::Client::builder()
                .build()
                .expect("should be able to build reqwest client");
    }

    let body = client.get(url).send();
    match body {
        Ok(res) => Ok(res.bytes().unwrap()),
        Err(err) => {
            println!("{}", err);
            Err(err)
        }
    }
}
pub fn query_text(url: &str) ->String {
    let body = reqwest::blocking
            ::get(url);
    let ar = [0];
    match body {
        Ok(res) => res.text().expect("query bytes failed"),
        Err(err) => {
            println!("{}", err);
            panic!("query text failed!");
        }
    }
}

fn write_file(mut reader: Response) {
    let mut buf = [0u8; 1024 * 500];

    let mut file = std::fs::File::create("v.f56150——1.ts").expect("open file failed");
    loop {
        let res = reader.read(&mut buf);
        if let Ok(size) = res {
            println!("size is {}", size);
            if size <= 0 {
                break;
            }
            let handler = file.write(&buf[0..size]);
            handler.expect("写入失败");
            file.flush().expect("flush 失败");
        } else {
            panic!("读取失败");
        }
    }
}
fn get_proxy()->&'static str{
    unsafe {
        &proxys
    }
}
pub fn set_proxy(proxy_s: String){
    println!("proxy={}", &proxy_s);
    unsafe {
        proxys = proxy_s;
    }
}