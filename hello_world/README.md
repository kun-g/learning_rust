# Hello World

## 目的
熟悉Rust的项目结构，了解Rust的编译、运行和单元测试

## 过程
### 创建项目
使用以下命令创建项目
```
cargo new HelloWorld
```

目录结构如下：
```
HelloWorld
├── Cargo.toml
└── src
    └── main.rs
```

### 编译/运行
在Cargo.toml所在目录，执行
`cargo build`

以上命令会编译，并将结果放在target目录下，可以执行`target/debug/HelloWorld`来运行编译结果，也可以使用以下命令编译后直接运行结果：
`cargo run`

### 依赖
在Cargo.toml的`dependencies`节加入以下内容：
```
ferris-says = "0.2"
```


代码：
```
use ferris_says::say;

use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("世界你好！");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
```

这里值得注意的是`mut`描述符，Rust里默认变量是没有副作用的，如果要使用有副作用的方法/对象，需要显式的标注出来。

执行`cargo run`

## 总结

### 收获
- 编译: `cargo build`
- 运行：`cargo run`
- Cargo.toml中管理外部依赖
- 使用关键字`use`来引用库
- 使用描述符`mut`来标注有副作用的对象

### 疑问
- stdout.lock是做什么用的？
- say方法调用之后的unwrap()是做什么用的？


## 参考资料
- https://www.rust-lang.org/learn/get-started
