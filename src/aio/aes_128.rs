use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use std::io;

pub fn encrypt(input: &str) -> String {
    let shellcode_bytes: Vec<u8> = if input.contains("\\x") {
        // 解析 `\xAE\x10\x72` 这样的十六进制格式
        input
            .split("\\x")
            .skip(1) // 跳过第一个空字符串
            .filter_map(|s| u8::from_str_radix(s, 16).ok()) // 忽略错误
            .collect()
    } else {
        // 普通字符串按 UTF-8 处理
        input.as_bytes().to_vec()
    };

    // 处理 AES 加密
    let mut blocks: Vec<GenericArray<u8, _>> = shellcode_bytes
        .chunks(16)
        .map(|chunk| {
            let mut block = [0u8; 16];
            block[..chunk.len()].copy_from_slice(chunk);
            GenericArray::from(block)
        })
        .collect();

    let key: [u8; 16] = [24, 11, 15, 22, 18, 21, 13, 19, 16, 14, 23, 17, 20, 12, 10, 25];
    let key = GenericArray::from(key);
    let cipher = Aes128::new(&key);

    for block in &mut blocks {
        cipher.encrypt_block(block);
    }

    // 输出加密后的十六进制格式 `\xAE\x10\x72`
    blocks
        .iter()
        .flat_map(|block| block.iter().map(|byte| format!("\\x{:02x}", byte)))
        .collect::<Vec<String>>()
        .join("")
}

pub fn decrypt(input: &str) -> String { // 确保返回 String
    let shellcode_bytes: Vec<u8> = input
        .split("\\x")
        .skip(1)
        .filter_map(|s| u8::from_str_radix(s, 16).ok())
        .collect();

    let mut blocks: Vec<GenericArray<u8, _>> = shellcode_bytes
        .chunks(16)
        .map(|chunk| {
            let mut block = [0u8; 16];
            block[..chunk.len()].copy_from_slice(chunk);
            GenericArray::from(block)
        })
        .collect();

    let key: [u8; 16] = [24, 11, 15, 22, 18, 21, 13, 19, 16, 14, 23, 17, 20, 12, 10, 25];
    let key = GenericArray::from(key);
    let cipher = Aes128::new(&key);

    for block in &mut blocks {
        cipher.decrypt_block(block);
    }

    // **返回解密后的十六进制格式**
    let decrypted_hex: String = blocks
        .iter()
        .flat_map(|block| block.iter().map(|byte| format!("\\x{:02x}", byte)))
        .collect::<Vec<String>>()
        .join("");

    let cut_decrypted_hex = remove_trailing_zeros(&decrypted_hex);

    println!("解密后的十六进制格式: {}", cut_decrypted_hex);

    // **尝试转义成可读字符串**
    let decrypted_bytes: Vec<u8> = blocks.iter().flat_map(|block| block.iter().copied()).collect();

    if let Ok(text) = String::from_utf8(decrypted_bytes.clone()) {
        println!("解密后的字符串: {}", text.trim_end_matches('\0')); // 处理可能的填充零
        return text.trim_end_matches('\0').to_string();
    }

    println!("无法转换为可读字符串");
    cut_decrypted_hex // 返回十六进制格式
}


// 定义一个函数，用于移除十六进制字符串末尾的零字节
fn remove_trailing_zeros(hex_string: &str) -> String {
    // 将输入的十六进制字符串按照 "\\x" 分割，并跳过第一个元素（即空字符串）
    // 然后将每个分割后的字符串转换为 u8 类型，并收集到一个 Vec<u8> 中
    let bytes: Vec<u8> = hex_string
        .split("\\x")
        .skip(1)
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect();

    // 查找最后一个非零字节的索引，如果所有字节都是零，则返回 0
    // 如果找到了非零字节，则返回其索引加 1，以包含该非零字节
    let last_non_zero_index = bytes.iter().rposition(|&byte| byte != 0).map_or(0, |index| index + 1);
    // 根据最后一个非零字节的索引，截取字节数组，得到去除末尾零字节后的数组
    let trimmed_bytes = &bytes[..last_non_zero_index];

    // 将截取后的字节数组中的每个字节转换回十六进制字符串格式
    // 使用 format! 宏格式化为 "\\xXX" 的形式，其中 XX 是字节的十六进制表示
    // 最后将所有转换后的字符串连接成一个完整的字符串并返回
    trimmed_bytes
        .iter()
        .map(|byte| format!("\\x{:02x}", byte))
        .collect::<Vec<String>>()
        .join("")
}

// 定义一个公共函数 match_aes，用于处理AES加密和解密操作
pub fn match_aes() {
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
            _ => println!("无效选项，请重新输入。")
        }
    }
}
