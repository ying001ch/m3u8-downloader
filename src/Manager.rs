use crate::aes_demo;
use crate::combine;
use crate::http_util;
use crate::M3u8Item;
use crate::str_util;
use core::panic;
use std::env;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    println!("Hello this is M3u8-Downloader by rust");

    let args: Vec<String> = env::args().collect();
    let save_path = args[1].as_str();
    let m3u8_url = args[2].as_str();

    let pr = args
        .iter()
        .filter(|&e| e.contains("--proxy"))
        .map(|e| e.replace("--proxy=", ""))
        .find(|e| true);
    if pr.is_some() {
        http_util::set_proxy(pr.unwrap());
    }
    http_util::set_header(&args);

    let content;
    if args.len() >= 4 && args[3] == "--file" {
        //1. 解析m3u8文件
        if args.len() < 5 {
            panic!("--file 需要指定m3u8文件路径");
        }
        content = std::fs::read_to_string(&args[4]).unwrap();
    } else {
        //1. 解析m3u8文件
        content = http_util::query_text(m3u8_url);
    }

    let mut entity = M3u8Item::M3u8Entity::from(content);
    process(&mut entity, save_path, m3u8_url);

    download_decode(entity);

    println!("下载完毕！");

    combine::combine_clip(save_path);
}

fn process(entity: &mut M3u8Item::M3u8Entity, save_path: &str, m3u8_url: &str) {
    entity.save_path = Some(save_path.to_string());

    let mut idx1 = str_util::index_of('?', m3u8_url);
    if idx1 == -1 {
        idx1 = m3u8_url.len() as i32;
    }
    let idx2 = str_util::last_index('/', &m3u8_url[0..idx1 as usize]);
    if idx2 == -1 {
        panic!("最后一个 / 找不到");
    }
    entity.url_prefix = Some((&m3u8_url[0..idx2 as usize]).to_string() + "/");
    println!("url_prefix = {}", entity.url_prefix.as_ref().unwrap());

    entity.req_key();
}
fn download_decode(entity: M3u8Item::M3u8Entity) {
    println!("savePath={}", entity.save_path.as_ref().unwrap());

    let entity_it = Arc::new(entity);

    let mut pkg = vec![];
    let len = entity_it.clip_urls.len();
    for i in 0..len {
        pkg.push(((len - i) as i32, entity_it.clip_urls[len-1-i].clone(),0));
    }
    let pkg = Arc::new(Mutex::new(pkg));

    let mut vcs = vec![];
    for i in 0..get_thread_num() {
        let clone_entity = Arc::clone(&entity_it);
        let clone_pkg = Arc::clone(&pkg);
        let handler = thread::spawn(move || {
            sleep(Duration::from_millis(20u64+ (i as u64 *50u64)));
            loop {
                let dd = clone_entity.as_ref();
                let key = &dd.key;
                let iv = &dd.iv;
                let prefix = dd.url_prefix.as_ref().unwrap();

                let mut clip;
                let mut clip_index;
                let mut retry_num ;
                {
                    let mut pkd_ref = clone_pkg.lock().unwrap();
                    let (clip_index_, clip_, retry_num_) = match pkd_ref.pop(){
                        Some(e)=>e,
                        None=>break
                    };
                    clip = clip_;
                    clip_index = clip_index_;
                    retry_num = retry_num_;
                    if retry_num > 0{
                        println!("错误片段重新下载。retry_num={}", retry_num);
                    }
                }
                let file_ex = std::fs::File::open(format!(
                    "{}/{}.ts",
                    dd.save_path.as_ref().unwrap(),
                    make_name(clip_index)
                ));
                if file_ex.is_ok() {
                    continue;
                }

                let down_url = prefix.to_string() + clip.as_str();
                println!("--> {}", down_url);

                let result = http_util::query_bytes(&down_url,i as i32);
                if result.is_err() {
                    put_retry(&mut retry_num, &clone_pkg, clip_index, &clip);
                    println!("下载出错：{}", result.unwrap_err());
                    continue;
                }
                let mut byte_vec = Vec::with_capacity(result.as_ref().unwrap().len());
                for b in result.unwrap() {
                    byte_vec.push(b);
                }

                let result = if dd.need_decode() {
                    let res = aes_demo::decrypt(&byte_vec, key, iv);
                    if let Ok(v) = res{
                        v
                    }else{
                        put_retry(&mut retry_num, &clone_pkg, clip_index, &clip);
                        println!("下载出错：{}", res.unwrap_err());
                        continue;
                    }
                } else {
                    byte_vec
                };

                write_file(&result, &dd, make_name(clip_index));
                println!("下载成功！\n\n");
            }
        });
        vcs.push(handler);
    }
    for ha in vcs {
        ha.join().expect("线程被中断");
    }
}

fn put_retry(retry_num: &mut i32, clone_pkg: &Arc<Mutex<Vec<(i32, String, i32)>>>, 
        clip_index: i32, clip: &String) {
    if *retry_num < 3{
        let mut pkd_ref = clone_pkg.lock().unwrap();
        let len = pkd_ref.len();
        *retry_num += 1;
        pkd_ref.insert(len/2, (clip_index, clip.to_string(), *retry_num));
    }
}
fn get_thread_num()->u8{
    let num = std::env::args().filter(|e|e.contains("--worker="))
        .map(|e|e.replace("--worker=",""))
        .map(|e|->u8 {e.parse().expect("")})
        .find(|e|true)
        .unwrap_or(4);
    println!("worker num={}", num);
    num
}
fn make_name(num: i32) -> String {
    if num < 1000 {
        let s = format!("{}", num);
        let pad = "0".repeat(4 - s.len()) + &s;

        return pad;
    }
    format!("{}", num)
}

fn write_file(result: &[u8], entity: &M3u8Item::M3u8Entity, file_name: String) {
    let save_path = entity.save_path.as_ref().unwrap();
    let mut file =
        std::fs::File::create(format!("{}/{}.ts", save_path, file_name)).expect("open file failed");
    let usize = file.write(result).expect("写入文件失败");
    println!("写入成功 counter:{}, size: {}", file_name, usize);
}
