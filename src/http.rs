use reqwest::{blocking::Response, IntoUrl};
use std::{collections::HashMap, io::{Read, Write}};
use bytes::Bytes;

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
pub fn query_bytes(url: &str) ->Bytes {
    let body = reqwest::blocking
            ::get(url);
    let ar = [0];
    match body {
        Ok(res) => res.bytes().expect("query bytes failed"),
        Err(err) => {
            println!("{}", err);
            panic!("err 發生");
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
