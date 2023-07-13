use std::sync::RwLock;

use crate::http_util;

pub struct GlobalConfig{
    work_num: usize,
    proxys: Option<String>,
    headers: Vec<(String,String)>,
}
//TODO 全局配置存储
static GLOBAL_CONFIG: RwLock<GlobalConfig> = RwLock::new(GlobalConfig{
    work_num: 2,
    proxys: None,
    headers: vec![],
});
pub const TASK_DOWN: usize = 1; //下载视频
pub const TASK_COM: usize = 2;  //合并视频

//----------------------------------------------------------------
pub fn set_work_num(work_num: usize) {
    let a = GLOBAL_CONFIG.write();
    match a {
        Ok(mut res)=>res.work_num=work_num,
        Err(e)=>{
            println!("====> err: {}",e);
        }
    }
    // .unwrap();
    // a.borrow_mut().work_num = 12;
}
pub fn get_work_num() -> usize {
    GLOBAL_CONFIG.read().unwrap().work_num
}
//----------------------------------------------------------------
pub fn set_proxys(ss: String) {
    {
        let mut a = GLOBAL_CONFIG.write().unwrap();
        a.proxys = Some(ss);
    }
    http_util::update_client();
}
pub fn get_proxys() -> String {
    GLOBAL_CONFIG.read().unwrap().proxys.clone().unwrap_or("".to_string())
}
//----------------------------------------------------------------
pub fn set_headers(v: Vec<(String,String)>) {
    let mut a = GLOBAL_CONFIG.write().unwrap();
    a.headers = v;
}
pub fn get_headers() -> Vec<(String,String)> {
     GLOBAL_CONFIG.read().unwrap().headers.clone()
}