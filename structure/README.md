# 结构体

## 目的
熟悉Rust的结构体

## 过程
基于另一个结构体初始化新结构体
```
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
```

方法，使用impl为结构体声明方法，当方法没有self参数时，改方法为结构体的关联方法，可以直接调用
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
}

```

## 总结

### 收获
- 基于另一个结构体初始化新结构体的方法很有意思
- 有类似于Javascript中的构造语法: `{ key1, key2 }`
- 给结构体增加`#[derive(Debug)]`标签，可以使用一些调试工具

### 疑问


## 参考资料
- https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
