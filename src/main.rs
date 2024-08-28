mod aes_128;
use std::fs;

fn main() {
    // 读取文件内容
    let file_content = fs::read_to_string("output.txt").expect("Unable to read file");

    //对文件内容进行加密
    let encrypted_content = aes_128::encrypt(file_content);

    // 写入加密后的内容到文件
    // 把输出的内容放进 output.txt 文件
    fs::write("output.txt", encrypted_content).expect("Unable to write file");
}
