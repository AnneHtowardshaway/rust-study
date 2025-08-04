// Rust 变量详解示例

fn main() {
    println!("=== Rust 变量详解演示 ===\n");
    
    // 1. 变量声明和不可变性
    variable_immutability();
    
    // 2. 可变变量
    mutable_variables();
    
    // 3. 常量
    constants_demo();
    
    // 4. 变量遮蔽 (Shadowing)
    variable_shadowing();
    
    // 5. 数据类型
    data_types_demo();
    
    // 6. 类型推断和显式类型
    type_inference_and_annotation();
    
    // 7. 作用域和生命周期
    scope_and_lifetime();
    
    // 8. 解构赋值
    destructuring_demo();
}

// 1. 变量不可变性演示
fn variable_immutability() {
    println!("1. 变量不可变性:");
    
    // 默认情况下，变量是不可变的
    let x = 5;
    println!("x 的值是: {}", x);
    
    // x = 6; // 错误！不能修改不可变变量
    
    // 使用 let 重新绑定（这不是修改，而是创建新变量）
    let x = x + 1;
    println!("重新绑定后 x 的值是: {}", x);
    
    println!();
}

// 2. 可变变量演示
fn mutable_variables() {
    println!("2. 可变变量:");
    
    // 使用 mut 关键字声明可变变量
    let mut y = 5;
    println!("y 的初始值: {}", y);
    
    y = 6; // 可以修改可变变量的值
    println!("修改后 y 的值: {}", y);
    
    // 可变变量的类型不能改变
    // y = "hello"; // 错误！类型不匹配
    
    // 可变变量可以重新绑定为不可变
    let y = y;
    // y = 7; // 现在 y 又变成不可变了
    println!("重新绑定为不可变后 y 的值: {}", y);
    
    println!();
}

// 3. 常量演示
const MAX_POINTS: u32 = 100_000; // 常量必须显式指定类型
const PI: f64 = 3.14159;

fn constants_demo() {
    println!("3. 常量:");
    
    println!("MAX_POINTS: {}", MAX_POINTS);
    println!("PI: {}", PI);
    
    // 常量的特点：
    // - 使用 const 关键字
    // - 必须显式指定类型
    // - 可以在任何作用域中声明，包括全局作用域
    // - 只能被设置为常量表达式，不能是函数调用的结果
    // - 在整个程序运行期间都有效
    
    const INNER_CONSTANT: i32 = 42;
    println!("内部常量: {}", INNER_CONSTANT);
    
    println!();
}

// 4. 变量遮蔽演示
fn variable_shadowing() {
    println!("4. 变量遮蔽 (Shadowing):");
    
    let x = 5;
    println!("第一个 x: {}", x);
    
    // 遮蔽：创建同名的新变量
    let x = x + 1;
    println!("遮蔽后的 x: {}", x);
    
    // 可以改变类型
    let x = "现在是字符串";
    println!("类型改变后的 x: {}", x);
    
    // 内部作用域的遮蔽
    {
        let x = x.len();
        println!("内部作用域的 x (字符串长度): {}", x);
    }
    
    println!("回到外部作用域的 x: {}", x);
    
    // 遮蔽 vs 可变变量的区别
    let spaces = "   ";
    let spaces = spaces.len(); // 遮蔽，可以改变类型
    
    let mut spaces_mut = "   ";
    // spaces_mut = spaces_mut.len(); // 错误！可变变量不能改变类型
    spaces_mut = "    "; // 但可以改变值
    
    println!("spaces: {}, spaces_mut: {}", spaces, spaces_mut);
    
    println!();
}

// 5. 数据类型演示
fn data_types_demo() {
    println!("5. 数据类型:");
    
    // 标量类型
    println!("--- 标量类型 ---");
    
    // 整数类型
    let a: i8 = -128;
    let b: u8 = 255;
    let c: i32 = -2_147_483_648;
    let d: u32 = 4_294_967_295;
    let e: isize = -9223372036854775808; // 依赖架构
    let f: usize = 18446744073709551615; // 依赖架构
    
    println!("整数: i8={}, u8={}, i32={}, u32={}", a, b, c, d);
    println!("架构相关: isize={}, usize={}", e, f);
    
    // 浮点类型
    let x: f32 = 3.14;
    let y: f64 = 2.718281828; // 默认类型
    println!("浮点: f32={}, f64={}", x, y);
    
    // 布尔类型
    let t: bool = true;
    let f: bool = false;
    println!("布尔: t={}, f={}", t, f);
    
    // 字符类型
    let c: char = 'z';
    let heart_eyed_cat: char = '😻';
    let chinese: char = '中';
    println!("字符: c={}, emoji={}, 中文={}", c, heart_eyed_cat, chinese);
    
    // 复合类型
    println!("--- 复合类型 ---");
    
    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 解构
    println!("元组解构: x={}, y={}, z={}", x, y, z);
    println!("元组索引: tup.0={}, tup.1={}, tup.2={}", tup.0, tup.1, tup.2);
    
    // 数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [3; 5]; // [3, 3, 3, 3, 3]
    println!("数组: {:?}", arr);
    println!("重复数组: {:?}", arr2);
    println!("数组元素: arr[0]={}, arr[4]={}", arr[0], arr[4]);
    
    println!();
}

// 6. 类型推断和显式类型注解
fn type_inference_and_annotation() {
    println!("6. 类型推断和显式类型:");
    
    // 类型推断
    let guess = "42".parse().expect("不是数字!"); // 错误！类型不明确
    // 需要类型注解
    let guess: u32 = "42".parse().expect("不是数字!");
    println!("解析的数字: {}", guess);
    
    // 或者使用 turbofish 语法
    let guess2 = "42".parse::<i32>().expect("不是数字!");
    println!("turbofish 语法: {}", guess2);
    
    // 编译器可以推断的情况
    let x = 5; // i32
    let y = 3.14; // f64
    let z = true; // bool
    let s = "hello"; // &str
    
    println!("推断类型: x={}, y={}, z={}, s={}", x, y, z, s);
    
    // 显式类型注解
    let explicit_int: i64 = 42;
    let explicit_float: f32 = 3.14;
    let explicit_string: String = String::from("Hello");
    
    println!("显式类型: {}, {}, {}", explicit_int, explicit_float, explicit_string);
    
    println!();
}

// 7. 作用域和生命周期
fn scope_and_lifetime() {
    println!("7. 作用域和生命周期:");
    
    let outer = "外部变量";
    println!("外部作用域: {}", outer);
    
    {
        let inner = "内部变量";
        println!("内部作用域: outer={}, inner={}", outer, inner);
        
        // 内部遮蔽外部变量
        let outer = "遮蔽的外部变量";
        println!("遮蔽后: {}", outer);
    } // inner 在这里被销毁
    
    println!("回到外部作用域: {}", outer);
    // println!("{}", inner); // 错误！inner 已经超出作用域
    
    // 变量的生命周期
    let r;
    {
        let x = 5;
        r = &x; // 错误！x 的生命周期比 r 短
    }
    // println!("{}", r); // 悬垂引用
    
    println!();
}

// 8. 解构赋值演示
fn destructuring_demo() {
    println!("8. 解构赋值:");
    
    // 元组解构
    let point = (3, 5);
    let (x, y) = point;
    println!("点坐标: x={}, y={}", x, y);
    
    // 忽略某些值
    let (x, _) = point;
    println!("只要 x 坐标: {}", x);
    
    // 数组解构
    let arr = [1, 2, 3, 4, 5];
    let [first, second, ..] = arr;
    println!("数组前两个元素: {}, {}", first, second);
    
    // 结构体解构（需要先定义结构体）
    struct Person {
        name: String,
        age: u32,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    let Person { name, age } = person;
    println!("人员信息: 姓名={}, 年龄={}", name, age);
    
    // 重命名字段
    let Person { name: person_name, age: person_age } = Person {
        name: String::from("Bob"),
        age: 25,
    };
    println!("重命名后: 姓名={}, 年龄={}", person_name, person_age);
    
    println!();
}