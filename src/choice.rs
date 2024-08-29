use crate::aes_128;
use std::fs;

pub fn choice_select(choice: String) -> i32 {
    // 选项的处理
    let input: i32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    return input;
}

pub fn match_aes(file_path: String) {
    let file_path = file_path.trim().to_string();

    loop {
        let choice = crate::base_func::table::show(vec![
            String::from("加密"),
            String::from("解密"),
            String::from("返回"),
        ]);
        //匹配输入输出

        // 选项的处理(先输出)和匹配(结果)
        let match_encrypt_result = choice_select(choice);

        match match_encrypt_result {
            1 => {
                println!("正在读取 {} 内容", file_path);
                // 读取文件内容
                let file_content = fs::read_to_string(&file_path).expect("Unable to read file");
                //对文件内容进行加密
                let encrypted_content = aes_128::encrypt(file_content);

                // 写入加密后的内容到文件
                // 把输出的内容放进 output.txt 文件
                fs::write("output.txt", encrypted_content).expect("Unable to write file");
                //println!("{:?}", encrypted_content);
                println!("加密完成，加密后的内容已写入 output.txt 文件")
            }
            2 => {
                println!("正在读取 {} 内容", file_path);
                // 读取文件内容
                let file_content = fs::read_to_string(&file_path).expect("Unable to read file");
                // 对文件内容进行解密
                let decrypted_content = aes_128::decrypt(file_content);

                // 写入解密后的内容到文件
                // 把输出的内容放进 output.txt 文件
                fs::write("output.txt", decrypted_content).expect("Unable to write file");
                //println!("{:?}", decrypted_content);
                println!("解密完成，解密后的内容已写入 output.txt 文件")
            }
            3 => {
                return;
            }
            _ => println!("请输入有效的选项"),
        }
    }
}
