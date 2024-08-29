mod aes_128;
mod choice;
mod base_func;

//use std::fs;

fn main() {
    // 显示欢迎信息
    println!("Welcome,master!");

    loop {
        // 显示选项列表
        let choice = base_func::table::show(vec![String::from("aes"), String::from("退出")]);
        println!("{}", choice);

        // 选项的处理(先输出)和匹配(结果)
        let match_encrypt_result = choice::choice_select(choice);

        match match_encrypt_result {
            1 => {
                // 进入 aes 匹配选项
                choice::match_aes();
            }
            2 => {
                println!("再见");
                return;
            }
            _ => println!("请输入有效的选项"),
        }
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
