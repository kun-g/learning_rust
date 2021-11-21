// use std::io;

fn main() {
    print!("\n整数运算\n");
    integer_operations(200u16, 3u16);

    print!("\n浮点数运算\n");
    float_operations(100.5, 213.2);

    print!("\n布尔类型\n");
    bool_operations();

    println!("");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("元组");
    println!("X: {}, Y: {}, Z: {}", x, y, z);
    println!("X: {}, Y: {}, Z: {}", tup.0, tup.1, tup.2);

    println!("");
    let arr = [1, 2, 3];
    let arr3 = [3; 5];
    let [x, y, z] = arr;
    println!("数组");
    println!("长度: {}", arr.len());
    println!("X: {}, Y: {}, Z: {}", x, y, z);
    println!("X: {}, Y: {}, Z: {}", arr[0], arr[1], arr[2]);
    println!("X: {}, Y: {}, Z: {}", arr3[0], arr3[1], arr3[2]);
    println!("截取: {:?}", &arr[1..3]);


    let g: u32 = "42a".parse().expect("Not a number!");
    println!("G: {}", g);
}

fn integer_operations(a: u16, b: u16) {
    println!("{} + {} = {}", a, b, a+b);
    println!("{} - {} = {}", a, b, a-b);
    println!("{} * {} = {}", a, b, a*b);
    println!("{} / {} = {}", a, b, a/b);
    println!("{} % {} = {}", a, b, a%b);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

fn float_operations(a: f64, b: f64) {
    println!("{} + {} = {}", a, b, a+b);
    println!("{} - {} = {}", a, b, a-b);
    println!("{} * {} = {}", a, b, a*b);
    println!("{} / {} = {}", a, b, a/b);
    println!("{} % {} = {}", a, b, a%b);
}

fn bool_operations() {
    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
}
