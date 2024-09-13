mod aes_128;
mod base_func;
mod choice;
mod gcd;

fn main() {
    // 显示欢迎信息
    println!("Welcome,master!");

    loop {
        // 显示选项列表
        let choice = base_func::table::show(vec![
            String::from("aes"),          // AES 加密
            String::from("求最大公约数"), // 辗转相除法
            String::from("退出"),
        ]);

        // 选项的处理(先输出)和匹配(结果)
        let match_encrypt_result = choice::choice_select(choice);

        match match_encrypt_result {
            1 => {
                // 进入 aes 匹配选项
                choice::match_aes("output.txt".to_string());
            }
            2 => {
                // 读取两个正整数, 并计算最大公约数
                let num = gcd::get_num();
                gcd::output_result(gcd::gcd_by_euclidean_algorithm(num.0, num.1));
            }
            3 => {
                println!("再见");
                return;
            }
            _ => println!("请输入有效的选项"),
        }
    }
}
