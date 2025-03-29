mod aio;
mod base_func;

fn main() {
    println!("Welcome, master!");

    loop {
        let choice = base_func::table::show(vec![
            String::from("退出"),
            String::from("最大公约数"),     // 辗转相除法
            String::from("Fermat素数测试"), // Fermat素数测试
            String::from("对称加解密"),     // 对称加密
        ]);

        let match_encrypt_result = base_func::choice::choice_select(choice);

        match match_encrypt_result {
            1 => {
                println!("再见");
                return;
            }

            2 => {
                let num = aio::gcd::get_num();
                aio::gcd::output_result(num.0, num.1);
            }

            3 => {
                let choice_format = base_func::table::show(vec![
                    String::from("退出"),
                    String::from("素数测试"),
                    String::from("最大素数"),
                ]);
                let match_format_result = base_func::choice::choice_select(choice_format);
                match match_format_result {
                    1 => return,
                    2 => aio::fermat::use_fermat(),
                    3 => aio::fermat::get_max_prime(),
                    _ => println!("请输入有效的选项"),
                }
            }

            4 => {
                let choice_format = base_func::table::show(vec![
                    String::from("退出"),
                    String::from("维吉尼亚"),
                    String::from("AES"),
                ]);
                let match_format_result = base_func::choice::choice_select(choice_format);
                match match_format_result {
                    1 => return,
                    2 => {
                        let choice_format = base_func::table::show(vec![
                            String::from("退出"),
                            String::from("加密"),
                            String::from("解密"),
                        ]);
                        let match_format_result = base_func::choice::choice_select(choice_format);
                        match match_format_result {
                            1 => return,
                            2 => aio::symmetric::vigenere_encrypt(),
                            3 => aio::symmetric::vigenere_decrypt(),
                            _ => println!("请输入有效的选项"),
                        }
                    }
                    3 => {
                        // 调用不再依赖文件的 AES 交互方式
                        aio::aes_128::match_aes();
                    }
                    _ => println!("请输入有效的选项"),
                }
            }

            _ => println!("请输入有效的选项"),
        }
    }
}
