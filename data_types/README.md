# 数据类型

## 目的
熟悉Rust的数据类型

## 过程
### 标量类型
#### 整数
i8表示8位的有符号整数，类似的还有i16、i32、i64、i128
同样的，无符号的整数有u8、u16、u32、u64、u128

在写整数常量的时候，可以后缀数据类型，比如：666u16。

为了方便阅读，可以用下划线对数字进行分割，比如：`10_000_000`等同`100000000`

#### 浮点数
有`f32`和`f64`两种，默认是`f64`，因为有差不多的性能和更高的精度。

#### 布尔数
`bool`有两种可能的值：`true`和`false`，用于流程控制。

#### 字符
同C语言一样，Rust中字符`char`是标量，使用单引号：'X'。而使用双引号的是字符串"X"

### 组合类型
#### 元组
元组的特性：
- 创建时确定了长度，将无法增加或减少元素的数量
- 可以由不同的元素组成

以下两种声明方式都是可以的：
`let tup: (i32, f64, u8) = (500, 6.4, 1);`
`let tup = (500, 6.4, 1);`
  
访问元组的元素，可以展开元组：
`let (x, y, z) = tup;`
也可以直接通过下标访问
`println("Second value is {}", tup.1);`

#### 数组
数组的特性：
- 由相同的元素组成
- 长度不可以改变

以下三种声明方式都是可以的：
`let a: [i32; 5] = [1, 2, 3, 4, 5];`
`let a = [1, 2, 3, 4, 5];`
`let a = [3; 5]; // 长度为5，元素全部为三的数组`

访问数组元素的方式也有两种：
`let (x, y, z) = a;`
`println("Second value is {}", a[1]);`

可以截取数组的一段：
`println!("截取: {:?}", &arr[1..3]);`


## 总结

### 收获
- 基础数据类型：整数、浮点数、字符、布尔数、元组、数组

### 疑问
- 截取数组时的`&`是什么意义？


## 参考资料
- https://doc.rust-lang.org/book/ch03-02-data-types.html