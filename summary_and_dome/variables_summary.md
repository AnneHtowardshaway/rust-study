# Rust 变量详解

## 1. 变量基础

### 变量声明
```rust
let x = 5;        // 不可变变量
let mut y = 10;   // 可变变量
const MAX: u32 = 100; // 常量
```

### 不可变性 (Immutability)
- **默认不可变**: Rust 变量默认是不可变的
- **安全性**: 防止意外修改，提高代码安全性
- **性能**: 编译器可以进行更多优化

```rust
let x = 5;
// x = 6; // ❌ 编译错误
let x = 6; // ✅ 重新绑定，创建新变量
```

## 2. 可变变量

### 使用 `mut` 关键字
```rust
let mut counter = 0;
counter += 1; // ✅ 可以修改
```

### 可变性规则
- 只有值可以改变，类型不能改变
- 可以重新绑定为不可变变量

```rust
let mut x = 5;
x = 6;        // ✅ 修改值
// x = "hello"; // ❌ 不能改变类型

let x = x;    // 重新绑定为不可变
// x = 7;     // ❌ 现在不可变了
```

## 3. 常量 (Constants)

### 特点
- 使用 `const` 关键字
- 必须显式指定类型
- 只能设置为常量表达式
- 全程序有效
- 命名约定：大写字母 + 下划线

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;
```

## 4. 变量遮蔽 (Shadowing)

### 概念
用 `let` 重新声明同名变量，创建新变量

```rust
let x = 5;
let x = x + 1;      // 遮蔽，值变为 6
let x = "hello";    // 遮蔽，类型也可以改变
```

### 遮蔽 vs 可变变量
| 特性 | 遮蔽 (Shadowing) | 可变变量 (mut) |
|------|------------------|----------------|
| 改变值 | ✅ | ✅ |
| 改变类型 | ✅ | ❌ |
| 创建新变量 | ✅ | ❌ |
| 作用域影响 | ✅ | ❌ |

## 5. 数据类型

### 标量类型 (Scalar Types)

#### 整数类型
| 长度 | 有符号 | 无符号 |
|------|--------|--------|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

```rust
let a: i32 = -42;
let b: u32 = 42;
let c = 98_222; // i32 (默认)
```

#### 浮点类型
```rust
let x: f32 = 3.14;
let y = 2.0; // f64 (默认)
```

#### 布尔类型
```rust
let t = true;
let f: bool = false;
```

#### 字符类型
```rust
let c = 'z';
let heart = '❤';
let chinese = '中';
```

### 复合类型 (Compound Types)

#### 元组 (Tuple)
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;        // 解构
let first = tup.0;          // 索引访问
```

#### 数组 (Array)
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let b = [3; 5];             // [3, 3, 3, 3, 3]
let first = a[0];
```

## 6. 类型推断和注解

### 类型推断
```rust
let x = 5;        // 推断为 i32
let y = 3.14;     // 推断为 f64
```

### 显式类型注解
```rust
let x: i64 = 42;
let guess: u32 = "42".parse().expect("Not a number!");
let guess2 = "42".parse::<i32>().expect("Not a number!"); // turbofish
```

## 7. 作用域和生命周期

### 作用域规则
```rust
{                      // s 无效
    let s = "hello";   // s 有效
    // 使用 s
}                      // s 无效，被销毁
```

### 变量遮蔽在作用域中
```rust
let x = 5;
{
    let x = 10;        // 内部遮蔽
    println!("{}", x); // 输出 10
}
println!("{}", x);     // 输出 5
```

## 8. 解构赋值

### 元组解构
```rust
let point = (3, 5);
let (x, y) = point;
let (x, _) = point;    // 忽略 y
```

### 数组解构
```rust
let arr = [1, 2, 3, 4, 5];
let [first, second, ..] = arr;
```

### 结构体解构
```rust
struct Point { x: i32, y: i32 }
let p = Point { x: 0, y: 7 };
let Point { x, y } = p;
let Point { x: a, y: b } = p; // 重命名
```

## 最佳实践

1. **默认使用不可变变量**，只在需要时使用 `mut`
2. **使用有意义的变量名**
3. **适当使用类型注解**提高代码可读性
4. **利用遮蔽**进行类型转换和值变换
5. **使用解构**简化复杂数据的访问
6. **常量使用大写命名**
7. **注意作用域**，避免不必要的长生命周期