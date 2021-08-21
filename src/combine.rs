use std::{env, io::Write, process::Command};

pub fn combine_clip(save_path: &str) ->Result<(),String>{
    // 1. 检测环境变量
    let ffmpeg_dir = std::env::var("FFMPEG_PATH")
    .expect("没有配置 FFMPEG_PATH 环境变量");
    let ffmpeg = format!("{}\\ffmpeg.exe",ffmpeg_dir);
    println!("ffmpeg: {}", ffmpeg);

    // 2. 生成合并文件
    let com_file_name={
        let com_file_name = format!("{}\\combine.txt",save_path);
        let mut com_txt = std::fs::File::create(&com_file_name)
                .expect("创建合并文件失败");
        for entry in std::fs::read_dir(save_path).expect("msg") {
            let file_name = entry.unwrap().file_name().into_string()
                    .expect("获取文件名时错误");
            if !file_name.contains(".ts") {
                continue;
            }
            let line = format!("file '{}'\n", file_name);
            com_txt.write_all(line.as_bytes())
                    .expect(format!("生成合并文件时出错，file:{}", file_name).as_str());
        }
        com_txt.flush().unwrap();
        com_file_name
    };
    println!("com_file_name: {}", &com_file_name);
   

    let output_name = get_output_name();
    // 3.调用合并
    let output = 
        Command::new("cmd")
                .arg("/c")
                .arg(ffmpeg).arg("-f").arg("concat").arg("-i")
                .arg(com_file_name.as_str()).arg("-c").arg("copy")
                .arg(output_name)
                .output()
                .expect("ffmpeg exec error!");
    
    let output_str = String::from_utf8_lossy(&output.stderr);
    println!("output_str={}", output_str);

    Ok(())
}

fn get_output_name() ->String {
     env::args().filter(|e|e.contains("--output="))
            .map(|e|e.replace("--output=", ""))
            .find(|_e|true).unwrap_or("output.mp4".to_string())

}