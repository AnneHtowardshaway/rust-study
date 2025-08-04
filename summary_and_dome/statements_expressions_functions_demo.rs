// Rust 语句、表达式和函数详解示例

fn main() {
    println!("=== Rust 语句、表达式和函数演示 ===\n");
    
    // 1. 语句 vs 表达式
    statements_vs_expressions();
    
    // 2. 函数基础
    function_basics();
    
    // 3. 函数参数
    function_parameters();
    
    // 4. 返回值
    return_values();
    
    // 5. 表达式的使用
    expression_usage();
    
    // 6. 控制流表达式
    control_flow_expressions();
    
    // 7. 闭包简介
    closure_intro();
    
    // 8. 高阶函数
    higher_order_functions();
}

// 1. 语句 vs 表达式演示
fn statements_vs_expressions() {
    println!("1. 语句 vs 表达式:");
    
    // 语句 (Statements) - 执行操作但不返回值
    let x = 5; // 这是一个语句
    let y = 6; // 这也是一个语句
    
    // let x = (let y = 6); // 错误！语句不返回值
    
    // 表达式 (Expressions) - 计算并返回值
    let z = {
        let x = 3;
        x + 1  // 这是表达式，注意没有分号
    }; // 整个块是表达式，返回 4
    
    println!("z 的值: {}", z);
    
    // 函数调用是表达式
    let result = add_one(5);
    println!("函数调用表达式结果: {}", result);
    
    // 宏调用是表达式
    let message = format!("Hello, {}!", "World");
    println!("宏调用表达式: {}", message);
    
    // 数学运算是表达式
    let calculation = 5 + 6 * 2;
    println!("数学表达式: {}", calculation);
    
    println!();
}

fn add_one(x: i32) -> i32 {
    x + 1 // 表达式，返回 x + 1
}

// 2. 函数基础演示
fn function_basics() {
    println!("2. 函数基础:");
    
    // 调用无参数函数
    greet();
    
    // 调用有参数函数
    greet_person("Alice");
    
    // 调用有返回值函数
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    // 函数可以在定义前调用（函数提升）
    let result = multiply(4, 7);
    println!("4 * 7 = {}", result);
    
    println!();
}

// 无参数无返回值函数
fn greet() {
    println!("Hello, World!");
}

// 有参数无返回值函数
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// 有参数有返回值函数
fn add(a: i32, b: i32) -> i32 {
    a + b // 表达式，隐式返回
}

// 函数可以在调用后定义
fn multiply(a: i32, b: i32) -> i32 {
    return a * b; // 显式返回
}

// 3. 函数参数演示
fn function_parameters() {
    println!("3. 函数参数:");
    
    // 基本参数传递
    let x = 5;
    print_value(x);
    println!("x 仍然有效: {}", x); // i32 实现了 Copy trait
    
    // 字符串参数（所有权转移）
    let s = String::from("hello");
    take_ownership(s);
    // println!("{}", s); // 错误！s 的所有权已转移
    
    // 使用引用避免所有权转移
    let s2 = String::from("world");
    borrow_string(&s2);
    println!("s2 仍然有效: {}", s2);
    
    // 可变引用参数
    let mut s3 = String::from("hello");
    modify_string(&mut s3);
    println!("修改后的字符串: {}", s3);
    
    // 多个参数
    let result = calculate(10, 5, '+');
    println!("计算结果: {}", result);
    
    // 元组参数
    let point = (3, 4);
    print_point(point);
    
    // 数组参数
    let numbers = [1, 2, 3, 4, 5];
    print_array(&numbers);
    
    println!();
}

fn print_value(x: i32) {
    println!("值: {}", x);
}

fn take_ownership(s: String) {
    println!("获得所有权: {}", s);
} // s 在这里被销毁

fn borrow_string(s: &String) {
    println!("借用字符串: {}", s);
}

fn modify_string(s: &mut String) {
    s.push_str(", world!");
}

fn calculate(a: i32, b: i32, op: char) -> i32 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => 0,
    }
}

fn print_point(point: (i32, i32)) {
    println!("点坐标: ({}, {})", point.0, point.1);
}

fn print_array(arr: &[i32]) {
    println!("数组: {:?}", arr);
}

// 4. 返回值演示
fn return_values() {
    println!("4. 返回值:");
    
    // 隐式返回（表达式）
    let result1 = implicit_return(5);
    println!("隐式返回: {}", result1);
    
    // 显式返回
    let result2 = explicit_return(10);
    println!("显式返回: {}", result2);
    
    // 提前返回
    let result3 = early_return(-5);
    println!("提前返回: {}", result3);
    
    // 返回元组
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} 余 {}", quotient, remainder);
    
    // 返回 Option
    let result4 = safe_divide(10, 2);
    match result4 {
        Some(value) => println!("安全除法结果: {}", value),
        None => println!("除零错误"),
    }
    
    let result5 = safe_divide(10, 0);
    match result5 {
        Some(value) => println!("安全除法结果: {}", value),
        None => println!("除零错误"),
    }
    
    // 返回 Result
    let result6 = parse_number("42");
    match result6 {
        Ok(num) => println!("解析成功: {}", num),
        Err(e) => println!("解析失败: {}", e),
    }
    
    // 无返回值函数（返回单元类型 ()）
    print_separator();
    
    println!();
}

fn implicit_return(x: i32) -> i32 {
    x * 2 // 没有分号，这是表达式
}

fn explicit_return(x: i32) -> i32 {
    return x * 3; // 显式 return
}

fn early_return(x: i32) -> i32 {
    if x < 0 {
        return 0; // 提前返回
    }
    x * 2
}

fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn parse_number(s: &str) -> Result<i32, &'static str> {
    match s.parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("无法解析为数字"),
    }
}

fn print_separator() {
    println!("--- 分隔线 ---");
}

// 5. 表达式的使用
fn expression_usage() {
    println!("5. 表达式的使用:");
    
    // 块表达式
    let result = {
        let x = 3;
        let y = 4;
        x * x + y * y // 返回 25
    };
    println!("块表达式结果: {}", result);
    
    // if 表达式
    let number = 6;
    let description = if number % 2 == 0 {
        "偶数"
    } else {
        "奇数"
    };
    println!("{} 是 {}", number, description);
    
    // match 表达式
    let grade = 85;
    let letter_grade = match grade {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("分数 {} 对应等级 {}", grade, letter_grade);
    
    // loop 表达式
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // loop 可以返回值
        }
    };
    println!("loop 表达式结果: {}", result);
    
    // 函数调用表达式
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum(); // 方法链调用
    println!("数组求和: {}", sum);
    
    println!();
}

// 6. 控制流表达式
fn control_flow_expressions() {
    println!("6. 控制流表达式:");
    
    // if let 表达式
    let some_value = Some(3);
    if let Some(x) = some_value {
        println!("if let 匹配到: {}", x);
    }
    
    // while let 表达式
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
    
    // for 循环表达式
    let numbers = [1, 2, 3, 4, 5];
    for (index, value) in numbers.iter().enumerate() {
        println!("索引 {}: 值 {}", index, value);
    }
    
    // 嵌套控制流
    for i in 1..=3 {
        let result = match i {
            1 => "第一",
            2 => "第二",
            3 => "第三",
            _ => "其他",
        };
        println!("数字 {} 是 {}", i, result);
    }
    
    println!();
}

// 7. 闭包简介
fn closure_intro() {
    println!("7. 闭包简介:");
    
    // 基本闭包
    let add_one = |x| x + 1;
    println!("闭包调用: {}", add_one(5));
    
    // 带类型注解的闭包
    let multiply = |x: i32, y: i32| -> i32 { x * y };
    println!("带类型闭包: {}", multiply(3, 4));
    
    // 捕获环境变量
    let factor = 10;
    let scale = |x| x * factor; // 捕获 factor
    println!("捕获环境变量: {}", scale(5));
    
    // 闭包作为参数
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("使用闭包映射: {:?}", doubled);
    
    // 闭包作为返回值
    let make_adder = |n| move |x| x + n;
    let add_five = make_adder(5);
    println!("闭包作为返回值: {}", add_five(10));
    
    println!();
}

// 8. 高阶函数
fn higher_order_functions() {
    println!("8. 高阶函数:");
    
    // 函数作为参数
    let result1 = apply_operation(5, 3, add);
    let result2 = apply_operation(5, 3, multiply);
    println!("函数作为参数: {} + {} = {}", 5, 3, result1);
    println!("函数作为参数: {} * {} = {}", 5, 3, result2);
    
    // 使用闭包作为参数
    let result3 = apply_operation(10, 2, |a, b| a - b);
    println!("闭包作为参数: {} - {} = {}", 10, 2, result3);
    
    // 返回函数
    let operation = get_operation('+');
    println!("返回函数: {}", operation(8, 3));
    
    // 函数组合
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)  // 过滤偶数
        .map(|&x| x * x)           // 平方
        .collect();
    println!("函数组合 (偶数平方): {:?}", result);
    
    // 使用 fold 进行累积
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("fold 累积求和: {}", sum);
    
    println!();
}

// 接受函数作为参数的高阶函数
fn apply_operation<F>(a: i32, b: i32, op: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    op(a, b)
}

// 返回函数的函数
fn get_operation(op: char) -> fn(i32, i32) -> i32 {
    match op {
        '+' => add,
        '*' => multiply,
        _ => add, // 默认返回加法
    }
}