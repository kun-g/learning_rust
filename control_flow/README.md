# 流程控制

## 目的
熟悉Rust的流程控制

## 过程
### if
```
fn test_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

```

因为if是个表达式，所以可以有以下用法：
```
println!("number is even: {}", if number % 2 == 0 { true } else { false });
```

### 循环
循环的方法有loop，while，for

```
loop {
    println!("again!");
}
```
loop制造一个死循环，只能用break跳出。Rust中的loop可以命名，break可以指定要break的loop:
```
let mut count = 0;
'counting_up: loop {
    println!("count = {}", count);
    let mut remaining = 10;

    loop {
        println!("remaining = {}", remaining);
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }

    count += 1;
}
println!("End count = {}", count);
```

loop还可以有返回值，通过break来返回
```
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};

println!("The result is {}", result);
```

while就是带结束条件的loop:
```
fn test_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

for:
```
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {}", element);
}
```

## 总结

### 收获
- 命名loop很有意思，某些比较绕的逻辑实现起来会更清晰一些

### 疑问


## 参考资料
- https://doc.rust-lang.org/book/ch03-05-control-flow.html

