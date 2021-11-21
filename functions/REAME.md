# 函数

## 目的
熟悉Rust的函数

## 过程
### 函数的声明与调用
关键字`fn`用于声明函数，声明一个接受一个u32参数的叫another_function的函数:
`fn another_function(x: u32) { }`

调用这个函数：
`another_function(12u32);`

带返回值的函数：
```
fn plus_one(x: u32) -> u32 {
    x + 1 // 注意这里是没有分号的
}
```

使用函数的返回值：
```
println!("Plus {}", plus_one(1));
println!("Plus {}", plus_one(4));
```


## 总结

### 收获
- 函数的声明、调用
- 函数的出口是表达式(expression)而不是语句(statement)，所以不要加分号

### 疑问
- 能否把函数作为参数使用？
- 能否函数作为返回值时候？


## 参考资料
- https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
