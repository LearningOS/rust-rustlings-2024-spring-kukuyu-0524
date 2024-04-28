// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data); // 传递对字符串的引用

    println!("Last character: {}", last_char);

    string_uppercase(data); // 传递字符串的所有权
}

// 不应该获取所有权
fn get_char(data: &String) -> char {
    data.chars().last().expect("Empty string")
}

// 应该获取所有权
fn string_uppercase(data: String) {
    let uppercased = data.to_uppercase(); // 将字符串转换为大写
    println!("{}", uppercased);
}
