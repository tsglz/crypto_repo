use rand::Rng; // 引入rand库以生成随机数
use std::io::{self, Write}; // 引入标准输入输出库

// Fermat素性测试函数
fn fermat_primality_test(n: u64, k: usize) -> (bool, Vec<u64>) {
    // 如果n是2或3，返回true（是质数）及空列表
    if n == 2 || n == 3 {
        return (true, vec![]);
    }
    // 如果n是偶数且不等于2，返回false（不是质数）及空列表
    if n % 2 == 0 {
        return (false, vec![]);
    }

    // 存储在测试过程中选择的所有随机'a'值
    let mut chosen_a_values = Vec::new();
    let mut rng = rand::thread_rng(); // 创建随机数生成器

    // 执行测试k次
    for _ in 0..k {
        // 随机选择一个整数a，范围是2到n-2
        let a = rng.gen_range(2..n - 1);
        chosen_a_values.push(a);

        // 根据费马小定理：如果n是质数，则a^(n-1) % n 应该等于1
        if mod_exp(a, n - 1, n) != 1 {
            return (false, chosen_a_values); // n为合数
        }
    }

    (true, chosen_a_values) // n可能是质数
}

// 计算模幂函数 base^exponent % modulus
fn mod_exp(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus; // 奇数指数
        }
        exponent >>= 1; // 指数 = 指数 / 2
        base = (base * base) % modulus; // 基数 = 基数^2 % 模数
    }
    
    result
}

fn get_num()-> (u64) {
    let mut num_to_test = String::new();
    print!("请输入待测数:");
    io::stdout().flush().expect("刷新输出失败"); // 刷新输出缓冲
    io::stdin().read_line(&mut num_to_test).expect("读取输入失败");

    // 将输入的字符串转换为 u64 类型
    let num1: u64 = num_to_test.trim().parse().expect("请输入有效的整数");
    num1
}

pub fn use_fermat() {
    // 参数设置
    let n = get_num() as u64; // 要测试的数
    let k = 20;        // 测试的迭代次数

    // 执行Fermat素性测试
    let (is_prime, a_values) = fermat_primality_test(n, k);
    println!("测试结果：{} 使用数组：{:?} 测试次数为：{}", is_prime, a_values, a_values.len());
}