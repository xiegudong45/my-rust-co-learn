# Homework1
## 1. `rustup`是什么，如何用它来管理Rust版本？
> * `rustup`是Rust的构建系统和包管理器。可以处理很多任务，如构建代码、下载依赖库、以及编译这些库。
> * rust版本分三种: stable, nightly, beta. 设置默认版本使用`rustup toolchain default <version>`，使用非默认版本运行代码: `rustup run <version> cargo build
`, 在不同项目下使用不同的版本：`rustup override set nightly`。

## 2. Rust中`&str`和`String`的区别是什么，每个应该在什么时候使用？
> * `String`底层由Vec实现，它的大小是可变的。由`String`修饰的变量存在堆上。
> * `&str`是一个字符串字面量的引用，实际存放在程序的只读数据段中。在程序运行时被加载到内存中读取。在编译时大小就确定下来。被`&str`修饰的变量是始终不可变的，即使在跨线程的场景下。

## 3. Rust中的泛型类型是什么，你可以自己写几个例子吗？
## 4. Rust中使用泛型类型的一些常见数据结构有哪些？
## 5. Rust中有哪三种循环结构，它们如何使用？
## 6. 在Rust中，`match`、`while let`、`let`和`if let`之间的区别是什么，每个应该在什么时候使用？
## 7. Rust中有哪三种类型的注释？