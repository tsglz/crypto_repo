use std::io;

pub fn get_num()-> (u32, u32) {
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("请输入两个整数:");

    io::stdin().read_line(&mut input1).expect("读取输入失败");
    io::stdin().read_line(&mut input2).expect("读取输入失败");

    // 将输入的字符串转换为i32类型
    let num1: u32 = input1.trim().parse().expect("请输入有效的整数");
    let num2: u32 = input2.trim().parse().expect("请输入有效的整数");
    return (num1, num2);
}

pub fn gcd_by_euclidean_algorithm(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    return gcd_by_euclidean_algorithm(b, a % b);
}

pub fn output_result(result: u32) {
    println!("最大公约数为: {}", result);
}