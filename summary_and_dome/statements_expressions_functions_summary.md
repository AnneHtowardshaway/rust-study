# Rust 语句、表达式和函数详解

## 1. 语句 vs 表达式

### 核心区别
- **语句 (Statements)**: 执行操作但不返回值
- **表达式 (Expressions)**: 计算并返回值

```rust
// 语句
let x = 5;           // 变量绑定语句
let y = 6;           // 变量绑定语句

// 表达式
let z = {
    let x = 3;
    x + 1            // 表达式，返回 4（注意没有分号）
};

// 错误示例
// let x = (let y = 6); // ❌ 语句不返回值
```

### 常见表达式类型
- 数学运算: `5 + 6`
- 函数调用: `add(1, 2)`
- 宏调用: `println!("hello")`
- 块: `{ let x = 1; x + 1 }`
- 控制流: `if`, `match`, `loop`

## 2. 函数基础

### 函数定义语法
```rust
fn function_name(parameter: Type) -> ReturnType {
    // 函数体
    expression // 隐式返回
}
```

### 基本示例
```rust
// 无参数无返回值
fn greet() {
    println!("Hello!");
}

// 有参数无返回值
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// 有参数有返回值
fn add(a: i32, b: i32) -> i32 {
    a + b  // 隐式返回
}

// 显式返回
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // 显式返回
}
```

### 函数特性
- **函数提升**: 可以在定义前调用
- **snake_case 命名**: 函数名使用下划线分隔
- **类型必须明确**: 参数和返回值类型必须显式声明

## 3. 函数参数

### 参数传递方式

#### 值传递 (Copy)
```rust
fn print_value(x: i32) {  // i32 实现了 Copy
    println!("{}", x);
}

let num = 5;
print_value(num);  // num 仍然有效
```

#### 所有权转移 (Move)
```rust
fn take_ownership(s: String) {
    println!("{}", s);
}  // s 被销毁

let s = String::from("hello");
take_ownership(s);  // s 的所有权转移
// println!("{}", s);  // ❌ s 已失效
```

#### 不可变引用
```rust
fn borrow_string(s: &String) {
    println!("{}", s);
}

let s = String::from("hello");
borrow_string(&s);  // 借用
println!("{}", s);  // s 仍然有效
```

#### 可变引用
```rust
fn modify_string(s: &mut String) {
    s.push_str(", world!");
}

let mut s = String::from("hello");
modify_string(&mut s);
```

### 复杂参数类型
```rust
// 元组参数
fn print_point(point: (i32, i32)) {
    println!("({}, {})", point.0, point.1);
}

// 数组切片参数
fn print_array(arr: &[i32]) {
    println!("{:?}", arr);
}

// 多个参数
fn calculate(a: i32, b: i32, op: char) -> i32 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => 0,
    }
}
```

## 4. 返回值

### 隐式返回 vs 显式返回
```rust
// 隐式返回（推荐）
fn add(a: i32, b: i32) -> i32 {
    a + b  // 表达式，没有分号
}

// 显式返回
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // 使用 return 关键字
}
```

### 提前返回
```rust
fn process_number(x: i32) -> i32 {
    if x < 0 {
        return 0;  // 提前返回
    }
    x * 2
}
```

### 多值返回（元组）
```rust
fn divide_with_remainder(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

let (quotient, remainder) = divide_with_remainder(17, 5);
```

### Option 和 Result 返回类型
```rust
// Option - 可能没有值
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

// Result - 可能出错
fn parse_number(s: &str) -> Result<i32, &'static str> {
    match s.parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("解析失败"),
    }
}
```

### 单元类型 ()
```rust
fn print_hello() {  // 返回类型是 ()
    println!("Hello!");
}  // 等价于 fn print_hello() -> () { ... }
```

## 5. 表达式详解

### 块表达式
```rust
let result = {
    let x = 3;
    let y = 4;
    x * x + y * y  // 返回 25
};
```

### 控制流表达式

#### if 表达式
```rust
let number = 6;
let description = if number % 2 == 0 {
    "偶数"
} else {
    "奇数"
};
```

#### match 表达式
```rust
let grade = 85;
let letter = match grade {
    90..=100 => "A",
    80..=89 => "B",
    70..=79 => "C",
    _ => "F",
};
```

#### loop 表达式
```rust
let result = loop {
    // 一些逻辑
    if condition {
        break value;  // loop 可以返回值
    }
};
```

## 6. 闭包 (Closures)

### 基本语法
```rust
let closure = |param| expression;
let closure = |param: Type| -> ReturnType { body };
```

### 闭包示例
```rust
// 简单闭包
let add_one = |x| x + 1;

// 带类型注解
let multiply = |x: i32, y: i32| -> i32 { x * y };

// 捕获环境变量
let factor = 10;
let scale = |x| x * factor;

// 多行闭包
let complex = |x| {
    let temp = x * 2;
    temp + 1
};
```

### 闭包特性
- **类型推断**: 编译器可以推断参数和返回类型
- **环境捕获**: 可以捕获作用域中的变量
- **灵活语法**: 可以省略括号和大括号

## 7. 高阶函数

### 函数作为参数
```rust
fn apply_operation<F>(a: i32, b: i32, op: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    op(a, b)
}

// 使用
let result = apply_operation(5, 3, |a, b| a + b);
```

### 函数作为返回值
```rust
fn get_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

let add_five = get_adder(5);
let result = add_five(10);  // 15
```

### 迭代器和函数式编程
```rust
let numbers = vec![1, 2, 3, 4, 5];

let result: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)  // 过滤偶数
    .map(|&x| x * x)           // 平方
    .collect();                // 收集结果
```

## 8. 最佳实践

### 函数设计
1. **单一职责**: 每个函数只做一件事
2. **有意义的命名**: 使用描述性的函数名
3. **适当的参数数量**: 避免过多参数
4. **返回类型明确**: 使用 Option/Result 处理错误情况

### 表达式使用
1. **优先使用表达式**: 比语句更函数式
2. **避免不必要的 return**: 使用隐式返回
3. **合理使用块表达式**: 组织复杂逻辑

### 参数传递
1. **优先使用引用**: 避免不必要的所有权转移
2. **使用切片**: `&[T]` 比 `&Vec<T>` 更灵活
3. **字符串参数使用 &str**: 比 &String 更通用

### 错误处理
1. **使用 Result**: 处理可能失败的操作
2. **使用 Option**: 处理可能为空的值
3. **避免 panic**: 在库代码中使用 Result 而不是 panic