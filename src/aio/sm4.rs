use libsm::sm4::{Cipher, Mode};
use rand::RngCore;
use std::io;

#[allow(dead_code)]
fn rand_block() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut block: [u8; 16] = [0; 16];
    rng.fill_bytes(&mut block);
    block
}

pub fn encrypt(input: &str)-> String {
    // 设置固定的密钥
    let key: [u8; 16] = [
        24, 11, 15, 22, 18, 21, 13, 19, 16, 14, 23, 17, 20, 12, 10, 25,
    ];

    // 设置固定 IV（初始化向量）
    let iv: [u8; 16] = [
        109, 238, 104, 152, 24, 10, 163, 247, 47, 1, 84, 203, 113, 8, 69, 179,
    ];

    // 使用 CBC 模式创建加密器
    let cipher_result = Cipher::new(&key, Mode::Cbc);

    // 处理 Result 类型，解包获取 Cipher 对象
    let cipher = cipher_result.expect("创建加密器失败");

    // 要加密的明文
    let plain_text = input.to_string();

    // 加密
    let cipher_text = cipher
        .encrypt(&[], plain_text.as_bytes(), &iv)
        .expect("加密失败");

    hex::encode(&cipher_text)
}

pub fn decrypt(input: &str) -> String {
    let cipher_text = hex::decode(&input.to_string()).expect("无效的十六进制字符串");

    // 设置固定的密钥
    let key: [u8; 16] = [
        24, 11, 15, 22, 18, 21, 13, 19, 16, 14, 23, 17, 20, 12, 10, 25,
    ];

    // 设置固定 IV（初始化向量）
    let iv = [
        109, 238, 104, 152, 24, 10, 163, 247, 47, 1, 84, 203, 113, 8, 69, 179,
    ];

    // 使用 CBC 模式创建加密器
    let cipher_result = Cipher::new(&key, Mode::Cbc);

    // 处理 Result 类型，解包获取 Cipher 对象
    let cipher = cipher_result.expect("创建加密器失败");

    // 解密
    let decrypted_text = cipher.decrypt(&[], &cipher_text, &iv).expect("解密失败");
    println!("解密结果: {}", String::from_utf8_lossy(&decrypted_text));

    // 将加密结果转为的字符串
    let cipher_text_string = String::from_utf8(decrypted_text.clone()).expect("转换失败");
    cipher_text_string
}

pub fn match_sm4() {
    // 无限循环，直到用户选择退出
    loop {
        // 打印提示信息，让用户选择操作
        println!("请选择操作: 1. 加密 2. 解密 3. 退出");
        // 创建一个可变字符串 choice，用于存储用户输入的选择
        let mut choice = String::new();
        // 从标准输入读取一行数据，并存储到 choice 中
        io::stdin().read_line(&mut choice).unwrap();
        // 去除输入字符串中的前后空白字符
        let choice = choice.trim();

        // 使用 match 语句根据用户的选择执行相应的操作
        match choice {
            // 如果用户选择 "1"，执行加密操作
            "1" => {
                // 打印提示信息，让用户输入要加密的Shellcode
                println!("请输入要加密的内容:");
                // 创建一个可变字符串 input，用于存储用户输入的Shellcode
                let mut input = String::new();
                // 从标准输入读取一行数据，并存储到 input 中
                io::stdin().read_line(&mut input).unwrap();
                // 调用 encrypt 函数对输入的Shellcode进行加密，并获取加密结果
                let encrypted_content = encrypt(input.trim());
                // 打印加密结果
                println!("加密结果: {}", encrypted_content);
            }
            // 如果用户选择 "2"，执行解密操作
            "2" => {
                // 打印提示信息，让用户输入要解密的Shellcode
                println!("请输入要解密的内容:");
                // 创建一个可变字符串 input，用于存储用户输入的Shellcode
                let mut input = String::new();
                // 从标准输入读取一行数据，并存储到 input 中
                io::stdin().read_line(&mut input).unwrap();
                // 调用 decrypt 函数对输入的Shellcode进行解密，并获取解密结果
                let decrypted_content = decrypt(input.trim());
                // 打印解密结果
                println!("解密结果: {}", decrypted_content);
            }
            // 如果用户选择 "3"，退出循环，结束程序
            "3" => return,
            // 如果用户输入的不是 "1"、"2" 或 "3"，打印提示信息
            _ => println!("无效选项，请重新输入。"),
        }
    }
}
