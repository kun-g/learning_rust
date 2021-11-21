# 所有权

## 目的
熟悉Rust的所有权

## 过程
Rust的所有权模型，是为了解决内存管理问题。

Rust所有权规则：
- Rust中所有的值都有一个所有者
- 同一时间，一个值只有一个所有者
- 当所有者离开访问范围，该值将会被释放

### 反常识: 同一时间一个值只有一个所有者
```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // 在这一行之后，s1不再有效
    takes_ownership(s2); // 在这之后，s2不再有效
    
    let v1 = 2;
    let v2 = v1; // 在这一行之后，v1仍然有效

    println!("{}, world!", s1);
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}
```
s1和v1区别在于，v1的大小是确定的，所以在`v2=v1`时，复制了一份值给v2，所以v2和v1只是两个相同的值的所有者。

而s1的大小未知，所以是赋值就是所有权转移。

函数的调用与返回，都是所有权转移的过程。

### 引用/借用
逻辑似乎很清晰，不过有些情况还是很别扭：
```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```
明明后面还是需要使用s1，但是由于所有权的问题，calculate_length还要把所有权转移回来。

引用可以使得这种情况下的代码更加简洁：
```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
这里的`&`符号表示这里是引用，而不是所有权转移。这种情况也被称为借用(Borrow)。

### 反常识：可变借用同一时间只能有一个

```
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // 不行，因为s产生了两个可变引用。

    println!("{}, {}", r1, r2);
    
    let mut w = String::from("world");

    let w1 = w;
    let w2 = &mut w; // 不行，一个不变引用和一个可变引用也不可以。
}
```

这个机制看起来很奇怪，主要目的是在编译期解决数据同步访问问题（两个线程同时读写一个数据区）


## 总结

### 收获

### 疑问


## 参考资料
- https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
