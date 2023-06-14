# Homework1
## 1. `rustup`是什么，如何用它来管理Rust版本？
* `rustup`是Rust的构建系统和包管理器。可以处理很多任务，如构建代码、下载依赖库、以及编译这些库。
* rust版本分三种: stable, nightly, beta.
```
rustup show # 检查安装的版本
rustup install $name # 安装指定版本
rustup uninstall $name # 卸载指定版本
rustup default $name # 设置默认版本
```
## 2. Rust中`&str`和`String`的区别是什么，每个应该在什么时候使用？
> * `String`底层由Vec实现，它的大小是可变的。由`String`修饰的变量存在堆上。
> * `&str`是一个字符串字面量的引用，实际存放在程序的只读数据段中。在程序运行时被加载到内存中读取。在编译时大小就确定下来。被`&str`修饰的变量是始终不可变的，即使在跨线程的场景下。

## 3. Rust中的泛型类型是什么，你可以自己写几个例子吗？
> 泛型是具体类型或其他属性的抽象替代。
```rust 
struct Pair<K, V> {
    k: K,
    v: V,
}

impl<K, V> Pair<K, V> {
    fn new(k: K, v: V) -> Self {
        Pair{k, v}
    }
}

let p1 = Pair{
    k: "K",
    v: "v",
}

println!("{:?}, {:?}", p1.k, p1.v);
```

## 4. Rust中使用泛型类型的一些常见数据结构有哪些？
> 集合、范型结构体、范型枚举
## 5. Rust中有哪三种循环结构，它们如何使用？
* for循环
```rust 
let v = vec![1,2,3];
for num in v {
    println!("{:?}", num);
}
```
* loop无限循环
```rust 
    let mut x = 0;
    loop {
        println("{:?}", x);
        if x == 10 {
            break;
        }
        x += 1;
    }
```
* while条件循环
```rust 
    let mut x = 0;
    while x < 5 {
        println!("x is {}", x);
        x += 1;
    }
```
## 6. 在Rust中，`match`、`while let`、`let`和`if let`之间的区别是什么，每个应该在什
么时候使用？
> match 主要和枚举搭配使用，以匹配不同的枚举成员；if let 可以让我们关注想要的结果；while let 只处理正确的结果；let匹配元组中的元素
## 7. Rust中有哪三种类型的注释？
> 多行注释、单行注释、文档注释