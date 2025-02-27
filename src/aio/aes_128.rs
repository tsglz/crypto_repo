//use aes::cipher;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use crate::base_func::choice;
use std::fs;

pub fn encrypt(file_content: String) -> String {
    //密钥:这里填入16字节的密钥
    //把 shellcode 拆分成多个 16 字节的块
    //对每个块进行加密解密
    //不足 16 字节的块用 0x00 填充

    let key = file_content.trim();

    // 以逗号为分隔符，并转换为u8数组
    let shellcode_bytes: Vec<u8> = key
        .split(',') // 按逗号分割
        .map(|s| {
            // 去掉前缀 "0x"
            let hex_part = &s[2..];
            u8::from_str_radix(hex_part, 16).unwrap()
        })
        .collect();

    // 将Shellcode字符串转换为字节数组
    //let shellcode_bytes: Vec<u8> = shellcode.bytes().collect();

    // 将Shellcode字节数组拆分为16字节的块，不足16字节的块用0x00填充
    let mut blocks: Vec<GenericArray<u8, _>> = shellcode_bytes
        .chunks(16)
        .map(|chunk| {
            let mut block = [0u8; 16];
            block[..chunk.len()].copy_from_slice(chunk);
            GenericArray::from(block)
        })
        .collect();

    // key 这个可以自定义
    let key: [u8; 16] = [
        24, 11, 15, 22, 18, 21, 13, 19, 16, 14, 23, 17, 20, 12, 10, 25,
    ];

    let key = GenericArray::from(key);
    //println!("key: {:?}", key);

    // 用密钥初始化加密器
    let cipher = Aes128::new(&key);

    // 对每个块进行加密
    for block in &mut blocks {
        cipher.encrypt_block(block);
    }

    // 将加密后的字节数组转换为字符串，并在每两个字节之间插入一个逗号
    let encrypted_shellcode: String = blocks
        .iter()
        .flat_map(|block| block.iter().map(|byte| format!("\\x{:02x}", byte)))
        .take(blocks.len() * 16)
        .collect::<Vec<String>>()
        .join("");

    encrypted_shellcode
}

pub fn decrypt(file_content: String) -> String {

    //密钥:这里填入16字节的密钥
    //把 shellcode 拆分成多个 16 字节的块
    //对每个块进行加密解密
    //不足 16 字节的块用 0x00 填充

    let key = file_content.trim();

    // 以逗号为分隔符，并转换为u8数组
    let shellcode_bytes: Vec<u8> = key
        .split("\\x")
        .skip(1)    //跳过第一个空字符串
        .map(|s| {
            u8::from_str_radix(s, 16).unwrap()
        })
        .collect();

    // 将Shellcode字符串转换为字节数组
    //let shellcode_bytes: Vec<u8> = shellcode.bytes().collect();

    // 将Shellcode字节数组拆分为16字节的块，不足16字节的块用0x00填充
    let mut blocks: Vec<GenericArray<u8, _>> = shellcode_bytes
        .chunks(16)
        .map(|chunk| {
            let mut block = [0u8; 16];
            block[..chunk.len()].copy_from_slice(chunk);
            GenericArray::from(block)
        })
        .collect();

    //这个可以自定义
    let key: [u8; 16] = [
        24, 11, 15, 22, 18, 21, 13, 19, 16, 14, 23, 17, 20, 12, 10, 25,
    ];

    let key = GenericArray::from(key);

    // 用密钥初始化加密器
    let cipher = Aes128::new(&key);

    // 对每个块进行解密
    for block in &mut blocks {
        cipher.decrypt_block(block);
    }

    // 将解密后的字节数组转换为字符串，并在每两个字节之间插入一个逗号
    let decrypted_shellcode_nd: String = blocks
        .iter()
        .flat_map(|block| block.iter().map(|byte| format!("\\x{:02x}", byte)))
        .take(blocks.len() * 16)
        .collect::<Vec<String>>()
        .join("");

    let decrypted_shellcode = remove_trailing_zeros(&decrypted_shellcode_nd);

    decrypted_shellcode
}

fn remove_trailing_zeros(hex_string: &str) -> String {
    // 将十六进制字符串转换为字节数组
    let bytes: Vec<u8> = hex_string
        .split("\\x")
        .skip(1) // 跳过第一个空字符串
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect();

    // 找到最后一个非零字节的索引
    let last_non_zero_index = bytes.iter()
        .rposition(|&byte| byte != 0)
        .map_or(0, |index| index + 1);

    // 去掉末尾的零字节
    let trimmed_bytes = &bytes[..last_non_zero_index];

    // 将字节数组转换回十六进制字符串
    let trimmed_hex_string: String = trimmed_bytes
        .iter()
        .map(|byte| format!("\\x{:02x}", byte))
        .collect();

    trimmed_hex_string
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
        let match_encrypt_result = choice::choice_select(choice);

        match match_encrypt_result {
            1 => {
                println!("正在读取 {} 内容", file_path);
                // 读取文件内容
                let file_content = fs::read_to_string(&file_path).expect("Unable to read file");
                //对文件内容进行加密
                let encrypted_content = encrypt(file_content);

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
                let decrypted_content = decrypt(file_content);

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
