use super::http_util;

pub struct M3u8Entity{
    // content: String,
    pub method: String,
    pub key_url: String,
    pub key: [u8;16],
    pub iv: [u8;16],
    
    pub clip_urls: Vec<String>,
    pub url_prefix: Option<String>,
    pub savePath: Option<String>,
}
impl M3u8Entity {
    pub fn from(content: String) -> M3u8Entity {
        // let mut clip_urls = vec![];
        let method="".to_string();
        let key_url="".to_string();
        let key=[0;16];
        let iv=[0;16];
        let mut mm = M3u8Entity{
            clip_urls: vec![],
            url_prefix: None,
            method,
            key_url,
            key,
            iv,
            savePath:None
        };
        let lines  = content.lines();
        for li in lines {
            if li.contains("EXT-X-KEY"){
                // key method iv
                parse_Key(&mut mm, li);
            }else if li.contains(".ts") {
                mm.clip_urls.push(li.to_string());
            }
        }
        if mm.clip_urls.len()==0{
            panic!("M3U8 元信息解析错误，未解析到视频片段信息。content: \n{}", &content[0..200]);
        }
        if mm.key_url.len()==0 {
            println!("未发现密钥信息, 将不进行解密！");
        }
        println!("clip num: {}", mm.clip_urls.len());
        mm
    }
    pub fn req_key(&mut self) {
        if !self.need_decode(){
            return;
        }

        if !(&self.key_url).starts_with("http") {
            self.key_url = self.url_prefix.as_ref().unwrap().to_string() + &self.key_url;
        }
        println!("req_key key_url={}", &self.key_url);
        let raw_bytes = http_util::query_bytes(&self.key_url).unwrap();
        let mut key_bytes = [0u8;16];
        let len = raw_bytes.len();
        if len != 16 {
            panic!("reqKey failed");
        }
        let mut idx=0;
        for b in raw_bytes {
            key_bytes[idx] = b;
            idx += 1;
        }
        self.key = key_bytes;
        println!("key_bytes={:?}", key_bytes);
    }
    fn to_string(&self)->String{
        format!("{{method={},key_url={},\nkey={:?},\niv={:?},\nclip_urls={:?}}}",
            self.method, self.key_url, self.key,self.iv, self.clip_urls)
    }
    pub fn need_decode(&self)-> bool{
        !self.key_url.is_empty()
    }
}

fn parse_Key(mm: &mut M3u8Entity, line: &str) {
    let (k, vv) = line.split_once(":").unwrap();
    let keyStr = vv;
    let entrys = keyStr.split(",");
    for entry in entrys {
        let (x,y) = entry.split_once("=").unwrap();
        let val = y;
        if entry.starts_with("METHOD") {
            mm.method = val.to_string();
        }else if entry.starts_with("URI") {
            mm.key_url = val[1..val.len()-1].to_string();
        }else if entry.starts_with("IV") {
            mm.iv = hex2Byte(val);
        }
    }
}

fn hex2Byte(mut val: & str) -> [u8; 16] {
    if val.starts_with("0x") {
        val = &val[2..];
    }
    let nval = val.to_lowercase();
    // println!("{}", a);

    let length = val.len();
    let mut idx = 0;
    let mut bytes = [0u8; 16];
    while idx+2 <= length {
        let integer = from_hex(&nval[idx..idx+2]);
        bytes[idx/2] = integer;
        // bytes[idx/2] = integer/10*16 + integer % 10;
        idx += 2;
    }

    return bytes;
}

fn from_hex(idx: &str) -> u8 {
    let ac = idx.chars().next().unwrap();
    let ac2 = idx.chars().last().unwrap();

    let num:u8 = parse_hex_char(ac);
    let num2:u8 = parse_hex_char(ac2);

    num*16 + num2
}

fn parse_hex_char(ac: char) -> u8 {
    if !(ac >= 'a' && ac <= 'f') && !(ac >= '0' && ac <= '9'){
        panic!("解析数字错误");
    }
    let nu = ac as u8;
    if ac >= '0' && ac <= '9'{
        nu - ('0' as u8)
    }else {
        nu - ('a' as u8) + 10
    }
}