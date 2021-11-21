# 流程控制

## 目的
熟悉Rust的单元测试

## 过程
### 测试模块
测试模块用`#[cfg(test)]`属性来标注，测试过程用`#[test]`属性来标注
```
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(1, 1);
        assert_ne!(1, 2);
        assert!(true);
    }
}
```

### 异常捕获
有时候会预期测试过程抛出异常，可以使用`#[should_panic]`来指定
```
#[cfg(test)]
mod tests_panic {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}
```

### 禁用测试用例
使用`#[ignore]`禁用掉某些测试用例
```
#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }
}
```

## 总结

### 收获
- 测试模块用`#[cfg(test)]`属性来标注
- 测试过程用`#[test]`属性来标注
- 用`#[should_panic]`来捕获异常，`expected`属性来指定要捕获的异常
- 用`#[ignore]`来禁用测试用例

### 疑问


## 参考资料
- https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
