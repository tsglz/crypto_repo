mod aes_128;
mod table;
mod choice;

//use std::fs;
use std::io::{self, Write};

fn main() {
    loop {
        // 显示选项列表
        table::show();

        // 选择加解密的选项
        let mut choice = String::new();
        io::stdout().flush().unwrap(); //刷新标准输出缓冲区
        io::stdin().read_line(&mut choice).expect("输入错误");

        // 打印选择的选项
        println!("你选择了:{}", choice);

        // 选项的处理和匹配
        let match_encrypt_result = choice::choice_select(choice);
        println!("match_encrypt_result:{}", match_encrypt_result);
    }

    // 读取文件内容
    //let file_content = fs::read_to_string("output.txt").expect("Unable to read file");

    //对文件内容进行加密
    //let encrypted_content = aes_128::encrypt(file_content);

    // 写入加密后的内容到文件
    // 把输出的内容放进 output.txt 文件
    //fs::write("output.txt", encrypted_content).expect("Unable to write file");
    //println!("{:?}", encrypted_content);

    // 对文件内容进行解密
    //let decrypted_content = aes_128::decrypt(file_content);

    // 写入解密后的内容到文件
    // 把输出的内容放进 output.txt 文件
    //fs::write("output.txt", decrypted_content).expect("Unable to write file");
    //println!("{:?}", decrypted_content);
}
