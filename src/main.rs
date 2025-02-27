mod aio;
mod base_func;

fn main() {
    // 显示欢迎信息
    println!("Welcome,master!");

    loop {
        // 显示选项列表
        let choice = base_func::table::show(vec![
            String::from("退出"),
            String::from("aes"),            // AES 加密，包含 shellcode 转换
            String::from("最大公约数"),     // 辗转相除法
            String::from("Fermat素数测试"), // Fermat素数测试
            String::from("对称加解密"),     // 对称加密
        ]);

        // 选项的处理(先输出)和匹配(结果)
        let match_encrypt_result = base_func::choice::choice_select(choice);

        match match_encrypt_result {
            1 => {
                println!("再见");
                return;
            }
            2 => {
                // 进入 aes 匹配选项
                aio::aes_128::match_aes("output.txt".to_string());
            }
            3 => {
                // 读取两个正整数, 并计算最大公约数
                let num = aio::gcd::get_num();
                aio::gcd::output_result(num.0, num.1);
            }
            4 => {
                let choice_format = base_func::table::show(vec![
                    String::from("退出"),
                    String::from("素数测试"),
                    String::from("最大素数"),
                ]);
                // 选项的处理(先输出)和匹配(结果)
                let match_format_result = base_func::choice::choice_select(choice_format);
                match match_format_result {
                    1 => {
                        return;
                    }
                    2 => {
                        aio::fermat::use_fermat();
                    }
                    3 => {
                        aio::fermat::get_max_prime();
                    }
                    _ => println!("请输入有效的选项"),
                };
            }
            5 => {
                let choice_format =
                    base_func::table::show(vec![
                        String::from("退出"), 
                        String::from("维吉尼亚")
                    ]);
                // 选项的处理(先输出)和匹配(结果)
                let match_format_result = base_func::choice::choice_select(choice_format);
                match match_format_result {
                    1 => {
                        return;
                    }
                    2 => {
                        let choice_format = base_func::table::show(vec![
                            String::from("退出"),
                            String::from("加密"),
                            String::from("解密"),
                        ]);
                        // 选项的处理(先输出)和匹配(结果)
                        let match_format_result = base_func::choice::choice_select(choice_format);
                        match match_format_result {
                            1 => {
                                return;
                            }
                            2 => {
                                aio::symmetric::vigenere_encrypt();
                            }
                            3 => {
                                aio::symmetric::vigenere_decrypt();
                            }
                            _ => println!("请输入有效的选项"),
                        };
                    }
                    _ => println!("请输入有效的选项"),
                }
            }

            _ => println!("请输入有效的选项"),
        }
    }
}
