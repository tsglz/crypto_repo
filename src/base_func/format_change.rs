use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn extract_and_write(output_file: &str, process_string: &str) -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join(output_file);

    println!("Attempting to write to: {:?}", file_path); // Debugging line

    // 检查文件是否存在
    if !Path::new(&file_path).exists() {
        // 如果文件不存在，创建一个空文件
        File::create(&file_path).map_err(|e| {
            println!("Error creating file: {}", e);
            e
        })?;
    }

    let mut outfile = File::create(file_path).map_err(|e| {
        println!("Error creating file: {}", e);
        e
    })?;
    outfile.write_all(process_string.as_bytes()).map_err(|e| {
        println!("Error writing to file: {}", e);
        e
    })?;
    Ok(())
}

fn generate_shellcode_array(shellcode_str: &str) -> String {
    // 使用 split 方法以 \x 为分隔符并忽略第一个空元素
    let parts: Vec<&str> = shellcode_str.split("\\x").filter(|s| !s.is_empty()).collect();
    
    // 将每个部分格式化为 0xXX
    let c_array: Vec<String> = parts.iter()
        .map(|part| format!("0x{:02x}", u8::from_str_radix(part, 16).unwrap_or(0)))
        .collect();
    
    c_array.join(", ") // 使用逗号和空格连接
}

pub fn write_shellcode_to_file(shellcode_str: &str) {
    let output_file = "output.txt";
    let c_array = generate_shellcode_array(shellcode_str);
    if let Err(e) = extract_and_write(output_file, &c_array) {
        println!("发生错误: {}", e);
    }
}

fn main() {
    // 输入机器码，以文本形式表示
    let shellcode_str = r"\x83\x3f"; // 请将实际的机器码文本填入这里
    write_shellcode_to_file(shellcode_str);
}
