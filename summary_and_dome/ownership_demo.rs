// Rust 所有权、引用和借用详解示例

fn main() {
    println!("=== Rust 所有权、引用和借用演示 ===\n");
    
    // 1. 所有权基础概念
    ownership_basics();
    
    // 2. 引用和借用
    references_and_borrowing();
    
    // 3. 可变引用
    mutable_references();
    
    // 4. 引用规则演示
    reference_rules();
    
    // 5. 切片类型
    slice_examples();
}

// 1. 所有权基础演示
fn ownership_basics() {
    println!("1. 所有权基础:");
    
    // 所有权转移 (move)
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权转移给 s2
    
    // println!("{}", s1); // 错误！s1 已经失效
    println!("s2: {}", s2);
    
    // 克隆数据
    let s3 = String::from("world");
    let s4 = s3.clone(); // 深拷贝，s3 仍然有效
    println!("s3: {}, s4: {}", s3, s4);
    
    // 函数调用中的所有权转移
    let s5 = String::from("函数调用");
    takes_ownership(s5); // s5 的所有权转移到函数中
    // println!("{}", s5); // 错误！s5 已经失效
    
    // 返回值转移所有权
    let s6 = gives_ownership();
    println!("从函数获得: {}", s6);
    
    println!();
}

fn takes_ownership(some_string: String) {
    println!("函数接收: {}", some_string);
} // some_string 在这里被销毁

fn gives_ownership() -> String {
    let some_string = String::from("返回的字符串");
    some_string // 返回所有权
}

// 2. 引用和借用演示
fn references_and_borrowing() {
    println!("2. 引用和借用:");
    
    let s1 = String::from("hello");
    
    // 不可变引用（借用）
    let len = calculate_length(&s1); // &s1 创建一个指向 s1 的引用
    println!("字符串 '{}' 的长度是 {}", s1, len); // s1 仍然有效
    
    // 多个不可变引用是允许的
    let r1 = &s1;
    let r2 = &s1;
    println!("r1: {}, r2: {}", r1, r2);
    
    println!();
}

fn calculate_length(s: &String) -> usize { // s 是 String 的引用
    s.len()
} // s 离开作用域，但因为它不拥有引用的值，所以什么也不会发生

// 3. 可变引用演示
fn mutable_references() {
    println!("3. 可变引用:");
    
    let mut s = String::from("hello");
    
    // 可变引用
    change(&mut s);
    println!("修改后: {}", s);
    
    // 可变引用的限制：同一时间只能有一个可变引用
    let r1 = &mut s;
    // let r2 = &mut s; // 错误！不能同时有两个可变引用
    println!("可变引用: {}", r1);
    
    println!();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 4. 引用规则演示
fn reference_rules() {
    println!("4. 引用规则演示:");
    
    let mut s = String::from("hello");
    
    // 规则1: 可以有多个不可变引用
    {
        let r1 = &s;
        let r2 = &s;
        println!("不可变引用: {}, {}", r1, r2);
        // r1 和 r2 在这里结束使用
    }
    
    // 规则2: 不可变引用和可变引用不能同时存在
    {
        let r3 = &mut s; // 可变引用
        // let r4 = &s; // 错误！不能在可变引用存在时创建不可变引用
        println!("可变引用: {}", r3);
    }
    
    // 规则3: 引用必须总是有效的（防止悬垂引用）
    // let reference_to_nothing = dangle(); // 这会导致编译错误
    
    println!();
}

// 这个函数会导致悬垂引用错误
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回字符串 s 的引用，但 s 将在函数结束时被销毁
// }

// 5. 字符串切片演示
fn slice_examples() {
    println!("5. 字符串切片:");
    
    let s = String::from("hello world");
    
    // 字符串切片
    let hello = &s[0..5];  // 或 &s[..5]
    let world = &s[6..11]; // 或 &s[6..]
    let whole = &s[..];    // 整个字符串的切片
    
    println!("原字符串: {}", s);
    println!("切片 hello: {}", hello);
    println!("切片 world: {}", world);
    println!("整个切片: {}", whole);
    
    // 使用切片找到第一个单词
    let first_word = first_word(&s);
    println!("第一个单词: {}", first_word);
    
    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("数组切片: {:?}", slice);
    
    println!();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}