use crate::http;
use crate::M3u8Item;
use crate::aes_demo;
use std::borrow::Borrow;
use std::env;
use std::io::Write;
use std::slice::Iter;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn run(){
    println!("Hello this is M3u8-Downloader by rust");

    let args:Vec<String> = env::args().collect();
    let savePath = args[1].as_str();
    let m3u8Url = args[2].as_str();

    //1. 解析m3u8文件
    let content = http::query_text(m3u8Url);
    let mut entity = M3u8Item::M3u8Entity::from(content);
    process(&mut entity, savePath, m3u8Url);

    entity.savePath = Some(savePath.to_string());

    download_decode(entity);

    println!("下载完毕！");
}

fn process(entity: &mut M3u8Item::M3u8Entity, save_path: &str, m3u8_url: &str) {
    entity.savePath = Some(save_path.to_string());

    let mut idx1 = index_of('?', m3u8_url);
    if idx1 == -1 {
        idx1 = m3u8_url.len() as i32;
    }
    let idx2 = last_index('/', & m3u8_url[0..idx1 as usize]);
    if idx2 == -1{
        panic!("最后一个 / 找不到");
    }
    entity.url_prefix = Some((&m3u8_url[0..idx2 as usize]).to_string()+"/");
    println!("url_prefix = {}", entity.url_prefix.as_ref().unwrap());

    entity.reqKey();
}
fn index_of(ch: char, str: &str)->i32{
    let mut idx=0;
    while idx< str.len(){
        if str.chars().nth(idx).unwrap() == ch{
            return idx as i32;
        }
        idx += 1;
    }
    return -1;
}
fn last_index(ch: char, str: &str)->i32{
    let mut idx= str.len()-1;
    let chstr = ch.to_string();

    while idx >= 0{
        let c = str.get(idx..idx+1).unwrap();
        if c == chstr{
            println!("c={}",c);
            return idx as i32;
        }
        idx -= 1;
    }
    return -1;
}

fn download_decode(entity: M3u8Item::M3u8Entity) {
    println!("url_prefix={}, savePath={}", entity.url_prefix.as_ref().unwrap(),
            entity.savePath.as_ref().unwrap());

    let clip_urls = entity.clip_urls.clone();
    let it = Arc::new(Mutex::new(clip_urls));
    let entity_it = Arc::new(entity);
    let couter = Arc::new(Mutex::new(0));

    let mut vcs = vec![];
    for i in 0..8{
        let clone_counter = Arc::clone(&couter);
        let clone_it = Arc::clone(&it);
        let clone_entity = Arc::clone(&entity_it);
        let handler = thread::spawn(move ||{
            while true {
                let dd = clone_entity.as_ref();
                let key = &dd.key;
                let iv = &dd.iv;            
                let prefix = dd.url_prefix.as_ref().unwrap();
        
                let mut clip;
                {
                    let mut vec = clone_it.lock().unwrap();
                    let aa = vec.pop();
                    if aa.is_none() {
                        break;
                    }
                    clip = aa.unwrap();
                }
        
                let down_url = prefix.to_string() + clip.as_str();
                println!("--> {}", down_url);
        
                let result = http::query_bytes(&down_url);
                let mut byte_vec = vec![];
                for b in result {
                    byte_vec.push(b);
                }
                let result = aes_demo::decrypt(&byte_vec, key, iv);

                let mut co = 0;
                {
                    let mut counter = clone_counter.lock().unwrap();
                    *counter += 1;
                    co = *counter;
                }

                write_file(&result, &dd, co);
                println!("下载成功！\n\n");
            }
        });
        vcs.push(handler);
    }
    for ha in vcs {
        ha.join().expect("线程被中断");
    }
}

fn write_file(result: &[u8], entity: &M3u8Item::M3u8Entity, counter: i32) {
    // static mut counter:i32 = 0;
    let mut idx=counter;
    // unsafe{
    //     counter += 1;
    //     idx = counter;
    // }
    let save_path = entity.savePath.as_ref().unwrap();
    let mut file = std::fs::File::create(format!("{}/{}.ts", save_path, idx))
            .expect("open file failed");
    let usize = file.write(result).expect("写入文件失败");
    println!("写入成功 counter:{}, size: {}", idx, usize);
}