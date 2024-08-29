use std::io::{self, Write};

pub fn show(options: Vec<String>) -> String {
    for i in 0..options.len() {
        println!("{}.{}", i+1, options[i])
    }
    print!("请输入选项:");

    // 选择加解密的选项
    let mut choice = String::new();
    io::stdout().flush().unwrap(); //刷新标准输出缓冲区
    io::stdin().read_line(&mut choice).expect("输入错误");

    return choice.trim().to_string();
}
