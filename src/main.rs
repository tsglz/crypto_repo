mod aes_128;
mod choice;
mod base_func;

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
                choice::match_aes("output.txt".to_string());
            }
            2 => {
                println!("再见");
                return;
            }
            _ => println!("请输入有效的选项"),
        }
    }
}
