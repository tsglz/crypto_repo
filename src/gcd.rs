use std::io;

pub fn get_num()-> (i64, i64) {
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("请输入两个整数:");

    io::stdin().read_line(&mut input1).expect("读取输入失败");
    io::stdin().read_line(&mut input2).expect("读取输入失败");

    // 将输入的字符串转换为i32类型
    let num1: i64 = input1.trim().parse().expect("请输入有效的整数");
    let num2: i64 = input2.trim().parse().expect("请输入有效的整数");
    return (num1, num2);
}

pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (gcd, s1, t1) = extended_gcd(b, a % b);
    let s = t1;
    let t = s1 - (a / b) * t1;
    (gcd, s, t)
}

pub fn output_result(a: i64, b: i64) {
    let (gcd, s, t) = extended_gcd(a, b);
    println!("gcd({}, {}) = {}", a, b, gcd);
    println!("s = {}, t = {}", s, t);
    println!("{} = {} * {} + {} * {}", a * s + b * t, t, b, s, a);
}