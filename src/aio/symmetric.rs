use std::collections::HashMap;
use std::io::{self, Write};

const alphabet: &str = "abcdefghijklmnopqrstuvwxyz";

// 加密函数
pub fn vigenere_encrypt(){
    let mut plaintext = get_text("请输入明文");
    plaintext = plaintext.to_lowercase().replace(" ", "");
    let mut key = get_text("请输入密钥");
    key = key.to_lowercase().replace(" ", "");
    let mut ciphertext = String::new();
    let mut key_index = 0;

    for char in plaintext.chars() {
        if alphabet.contains(char) {  // 确保字符是字母
            let char_index = alphabet.chars().position(|c| c == char).unwrap();
            let key_char_index = alphabet.chars().position(|c| c == key.chars().nth(key_index % key.len()).unwrap()).unwrap();
            // 进行加密，字母表中按位置进行循环
            let encrypted_char = alphabet.chars().nth((char_index + key_char_index) % 26).unwrap();
            ciphertext.push(encrypted_char);
            key_index += 1;  // 密钥索引递增
        } else {
            ciphertext.push(char);  // 如果是非字母字符，直接加到密文中
        }
    }

    println!("密文为：{}", ciphertext);
}

// 解密函数
pub fn vigenere_decrypt(){
    let mut ciphertext = get_text("请输入密文");
    ciphertext = ciphertext.to_lowercase().replace(" ", "");
    let mut key = get_text("请输入密钥");
    key = key.to_lowercase().replace(" ", "");
    let mut plaintext = String::new();
    let mut key_index = 0;

    for char in ciphertext.chars() {
        if alphabet.contains(char) {  // 确保字符是字母
            let char_index = alphabet.chars().position(|c| c == char).unwrap();
            let key_char_index = alphabet.chars().position(|c| c == key.chars().nth(key_index % key.len()).unwrap()).unwrap();
            // 进行解密，字母表中按位置进行循环
            let decrypted_char = alphabet.chars().nth((char_index + 26 - key_char_index) % 26).unwrap();
            plaintext.push(decrypted_char);
            key_index += 1;  // 密钥索引递增
        } else {
            plaintext.push(char);  // 如果是非字母字符，直接加到明文中
        }
    }

    println!("明文为：{}", plaintext);
}

// 统计字母频率函数
fn letter_frequency(text: &str) -> HashMap<char, usize> {
    let mut frequency = HashMap::new();
    for char in text.to_uppercase().chars() {
        if alphabet.contains(char) {
            let counter = frequency.entry(char).or_insert(0);
            *counter += 1;
        }
    }
    frequency
}

pub fn get_text(output: &str) -> String {
    print!("{}: ", output);      // 打印提示信息
    io::stdout().flush().unwrap();  // 确保提示信息能立即显示
    
    let mut input = String::new();  // 用来存储用户输入的字符串
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    input.trim().to_string()  // 去掉尾部换行符并返回 String
}