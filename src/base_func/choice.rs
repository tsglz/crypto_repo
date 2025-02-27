pub fn choice_select(choice: String) -> i32 {
    // 选项的处理
    let input: i32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    return input;
}
