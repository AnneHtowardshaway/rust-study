# Rust 编程词汇表

## A
- **annotation** - 注解，类型注解
- **array** - 数组
- **argument** - 参数，实参
- **assign** - 赋值
- **automatic** - 自动的

## B
- **borrow** - 借用
- **borrowing** - 借用（过程）
- **binding** - 绑定
- **block** - 块，代码块
- **boolean** - 布尔类型
- **break** - 跳出循环

## C
- **capture** - 捕获
- **character** - 字符
- **clone** - 克隆，深拷贝
- **closure** - 闭包
- **compound** - 复合的
- **constant** - 常量
- **const** - 常量关键字
- **copy** - 复制
- **control flow** - 控制流

## D
- **data type** - 数据类型
- **declare** - 声明
- **destructuring** - 解构
- **drop** - 销毁，释放

## E
- **expression** - 表达式
- **explicit** - 显式的
- **enumerate** - 枚举

## F
- **function** - 函数
- **floating point** - 浮点数
- **filter** - 过滤
- **fold** - 折叠，累积

## G
- **garbage collection** - 垃圾回收

## H
- **heap** - 堆
- **higher-order** - 高阶的

## I
- **immutable** - 不可变的
- **implicit** - 隐式的
- **inference** - 推断
- **integer** - 整数
- **iterate** - 迭代
- **iterator** - 迭代器

## L
- **lifetime** - 生命周期
- **literal** - 字面量
- **loop** - 循环

## M
- **match** - 匹配
- **move** - 移动，所有权转移
- **mutable** - 可变的
- **mut** - 可变关键字

## O
- **ownership** - 所有权
- **option** - 选项类型

## P
- **parameter** - 参数，形参
- **parse** - 解析
- **pattern** - 模式
- **pointer** - 指针

## R
- **reference** - 引用
- **return** - 返回
- **result** - 结果类型

## S
- **scalar** - 标量
- **scope** - 作用域
- **shadowing** - 遮蔽
- **slice** - 切片
- **stack** - 栈
- **statement** - 语句
- **string** - 字符串
- **struct** - 结构体

## T
- **trait** - 特征
- **tuple** - 元组
- **type** - 类型
- **turbofish** - 涡轮鱼语法

## U
- **unit type** - 单元类型
- **unsigned** - 无符号的

## V
- **variable** - 变量
- **value** - 值

## W
- **while** - while循环

---

## 重要概念术语

### 所有权相关
- **ownership** - 所有权
- **borrow** - 借用
- **reference** - 引用
- **move** - 移动
- **copy** - 复制
- **clone** - 克隆
- **drop** - 销毁
- **lifetime** - 生命周期
- **scope** - 作用域
- **dangling reference** - 悬垂引用

### 变量相关
- **immutable** - 不可变的
- **mutable** - 可变的
- **shadowing** - 遮蔽
- **binding** - 绑定
- **constant** - 常量
- **variable** - 变量

### 类型相关
- **scalar type** - 标量类型
- **compound type** - 复合类型
- **integer** - 整数
- **floating point** - 浮点数
- **boolean** - 布尔值
- **character** - 字符
- **tuple** - 元组
- **array** - 数组
- **slice** - 切片
- **string** - 字符串

### 函数相关
- **function** - 函数
- **parameter** - 参数
- **argument** - 实参
- **return value** - 返回值
- **expression** - 表达式
- **statement** - 语句
- **closure** - 闭包
- **higher-order function** - 高阶函数

### 控制流相关
- **if expression** - if表达式
- **match expression** - match表达式
- **loop** - 循环
- **while** - while循环
- **for** - for循环
- **break** - 跳出
- **continue** - 继续

### 错误处理相关
- **Option** - 选项类型
- **Result** - 结果类型
- **Some** - 有值
- **None** - 无值
- **Ok** - 成功
- **Err** - 错误
- **panic** - 恐慌

### 内存管理相关
- **stack** - 栈
- **heap** - 堆
- **memory safety** - 内存安全
- **zero-cost abstraction** - 零成本抽象
- **garbage collection** - 垃圾回收

---

## 常用关键字

- `let` - 变量绑定
- `mut` - 可变性
- `const` - 常量
- `fn` - 函数
- `if` - 条件判断
- `else` - 否则
- `match` - 模式匹配
- `loop` - 无限循环
- `while` - 条件循环
- `for` - 迭代循环
- `break` - 跳出循环
- `continue` - 继续循环
- `return` - 返回
- `struct` - 结构体
- `enum` - 枚举
- `impl` - 实现
- `trait` - 特征
- `use` - 使用
- `mod` - 模块
- `pub` - 公开
- `crate` - 包
- `self` - 自身
- `Self` - 类型自身
- `super` - 父模块
- `where` - 约束条件

---

## 符号和操作符

- `&` - 引用
- `&mut` - 可变引用
- `*` - 解引用
- `->` - 函数返回类型
- `::` - 路径分隔符
- `..` - 范围（不包含结尾）
- `..=` - 范围（包含结尾）
- `_` - 通配符
- `|` - 闭包参数分隔符
- `?` - 错误传播操作符
- `!` - 宏调用或never类型
---


## 从代码文件中提取的具体术语

### 函数名 (Function Names)
- `main` - 主函数
- `ownership_basics` - 所有权基础
- `references_and_borrowing` - 引用和借用
- `mutable_references` - 可变引用
- `reference_rules` - 引用规则
- `slice_examples` - 切片示例
- `calculate_length` - 计算长度
- `takes_ownership` - 获取所有权
- `gives_ownership` - 给出所有权
- `change` - 改变
- `first_word` - 第一个单词
- `statements_vs_expressions` - 语句vs表达式
- `function_basics` - 函数基础
- `function_parameters` - 函数参数
- `return_values` - 返回值
- `expression_usage` - 表达式使用
- `control_flow_expressions` - 控制流表达式
- `closure_intro` - 闭包介绍
- `higher_order_functions` - 高阶函数
- `variable_immutability` - 变量不可变性
- `mutable_variables` - 可变变量
- `constants_demo` - 常量演示
- `variable_shadowing` - 变量遮蔽
- `data_types_demo` - 数据类型演示
- `type_inference_and_annotation` - 类型推断和注解
- `scope_and_lifetime` - 作用域和生命周期
- `destructuring_demo` - 解构演示

### 变量名 (Variable Names)
- `s1`, `s2`, `s3`, `s4`, `s5`, `s6` - 字符串变量
- `r1`, `r2`, `r3`, `r4` - 引用变量
- `hello`, `world`, `whole` - 切片变量
- `first_word` - 第一个单词
- `bytes` - 字节数组
- `slice` - 切片
- `result` - 结果
- `message` - 消息
- `calculation` - 计算
- `len` - 长度
- `some_string` - 某个字符串
- `reference_to_nothing` - 指向空的引用

### 常量名 (Constant Names)
- `MAX_POINTS` - 最大点数
- `PI` - 圆周率
- `INNER_CONSTANT` - 内部常量

### 方法名 (Method Names)
- `from` - 从...创建
- `clone` - 克隆
- `len` - 长度
- `push_str` - 推入字符串
- `as_bytes` - 转为字节
- `iter` - 迭代器
- `enumerate` - 枚举
- `parse` - 解析
- `expect` - 期望
- `format` - 格式化
- `collect` - 收集
- `filter` - 过滤
- `map` - 映射
- `fold` - 折叠
- `sum` - 求和
- `pop` - 弹出

### 类型名 (Type Names)
- `String` - 字符串类型
- `&str` - 字符串切片类型
- `usize` - 无符号大小类型
- `i32` - 32位有符号整数
- `u32` - 32位无符号整数
- `f64` - 64位浮点数
- `bool` - 布尔类型
- `char` - 字符类型
- `Option` - 选项类型
- `Result` - 结果类型
- `Vec` - 向量类型

### 特殊术语 (Special Terms)
- `move` - 移动语义
- `copy` - 复制语义
- `drop` - 销毁
- `dangling reference` - 悬垂引用
- `memory safety` - 内存安全
- `zero-cost abstraction` - 零成本抽象
- `pattern matching` - 模式匹配
- `type inference` - 类型推断
- `lifetime` - 生命周期
- `ownership transfer` - 所有权转移
- `borrowing rules` - 借用规则
- `mutable borrow` - 可变借用
- `immutable borrow` - 不可变借用

### 编程概念 (Programming Concepts)
- `scalar types` - 标量类型
- `compound types` - 复合类型
- `stack allocation` - 栈分配
- `heap allocation` - 堆分配
- `function signature` - 函数签名
- `return type` - 返回类型
- `parameter list` - 参数列表
- `function body` - 函数体
- `expression evaluation` - 表达式求值
- `statement execution` - 语句执行
- `control flow` - 控制流
- `pattern destructuring` - 模式解构
- `variable binding` - 变量绑定
- `type annotation` - 类型注解

### 错误相关 (Error Related)
- `compile error` - 编译错误
- `runtime error` - 运行时错误
- `borrow checker` - 借用检查器
- `lifetime error` - 生命周期错误
- `type mismatch` - 类型不匹配
- `use after move` - 移动后使用
- `double free` - 双重释放
- `memory leak` - 内存泄漏
- `null pointer` - 空指针
- `buffer overflow` - 缓冲区溢出--
-

## 学习建议和记忆技巧

### 核心概念记忆
1. **Ownership (所有权)** - Rust 最重要的概念
2. **Borrowing (借用)** - 通过引用访问数据
3. **Lifetime (生命周期)** - 引用的有效期
4. **Move (移动)** - 所有权转移
5. **Copy (复制)** - 值的复制

### 常用短语
- `let mut` - 声明可变变量
- `&mut` - 可变引用
- `-> Type` - 函数返回类型
- `impl Trait` - 实现特征
- `match expression` - 匹配表达式
- `if let` - 条件绑定
- `while let` - 循环绑定
- `for in` - 迭代循环

### 内存管理术语
- **Stack** - 栈内存（快速，自动管理）
- **Heap** - 堆内存（灵活，需要管理）
- **RAII** - 资源获取即初始化
- **Drop trait** - 析构特征
- **Memory safety** - 内存安全
- **Data race** - 数据竞争

### 函数式编程术语
- **Closure** - 闭包（匿名函数）
- **Iterator** - 迭代器
- **Higher-order function** - 高阶函数
- **Functional programming** - 函数式编程
- **Immutability** - 不可变性
- **Pure function** - 纯函数

### 类型系统术语
- **Type safety** - 类型安全
- **Static typing** - 静态类型
- **Type inference** - 类型推断
- **Generic** - 泛型
- **Trait bound** - 特征约束
- **Associated type** - 关联类型

### 并发相关术语
- **Thread** - 线程
- **Concurrency** - 并发
- **Parallelism** - 并行
- **Send trait** - 发送特征
- **Sync trait** - 同步特征
- **Mutex** - 互斥锁
- **Arc** - 原子引用计数

### 错误处理术语
- **Panic** - 恐慌（程序崩溃）
- **Unwrap** - 解包（可能恐慌）
- **Expect** - 期望（带消息的解包）
- **Error propagation** - 错误传播
- **? operator** - 问号操作符
- **Recoverable error** - 可恢复错误
- **Unrecoverable error** - 不可恢复错误

---

## 学习路径建议

### 初学者必掌握
1. **变量和可变性** - `let`, `mut`, `const`
2. **数据类型** - 标量和复合类型
3. **函数** - 定义、参数、返回值
4. **所有权** - 移动、借用、引用
5. **控制流** - `if`, `loop`, `match`

### 进阶概念
1. **结构体和枚举** - 自定义数据类型
2. **模式匹配** - `match`, `if let`
3. **错误处理** - `Option`, `Result`
4. **泛型和特征** - 代码复用
5. **生命周期** - 引用的有效期

### 高级特性
1. **智能指针** - `Box`, `Rc`, `RefCell`
2. **并发编程** - 线程、消息传递
3. **宏系统** - 代码生成
4. **异步编程** - `async`/`await`
5. **不安全代码** - `unsafe` 块