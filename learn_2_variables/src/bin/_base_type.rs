

fn main() {
    // let guess = "42".parse().expect("Not a Number!");

    // 显示处理 整型溢出问题
    // 可以限定计算后的结果不超过目标类型的最大值或低于最小值
    // assert_eq!(100u8.saturating_add(1), 101);
    // assert_eq!(u8::MAX.saturating_add(127), u8::MAX);
    // 按照补码循环溢出规则处理
    // let a: u8 = 255;
    // let b = a.wrapping_add(20);
    // println!("{}", b);

    // 浮点数陷阱
    // 示例一
    // assert!(0.1 + 0.2 == 0.3)
    // 示例二
    // let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    // let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    // println!("abc (f32)");
    // println!("  0.1 + 0.2 = {:x}", (abc.0 + abc.1).to_bits());
    // println!("        0.3 = {:x}", (abc.2).to_bits());
    // println!();
    // println!("xyz (f64)");
    // println!("  0.1 + 0.2 = {:x}", (xyz.0 + xyz.1).to_bits());
    // println!("        0.3 = {:x}", (xyz.2).to_bits());
    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // NaN类型


    // 运算符号
    // let sum = 5 + 10;
    // let diff = 99.1 - 12.8;
    // let product = 4 * 30;
    // let quotient = 56.7 / 32.2;
    // let remainder = 43 % 5;
    // println!("{} {} {} {} {}",sum, diff, product, quotient, remainder);

    // 位运算
    // let a: u8 = 0b_0000_0010;  // 2
    // let b: u8 = 0b_0000_0011;  // 3
    // println!("a value is {:08b}", a);
    // println!("b value is {:08b}", b);
    // println!("a & b value is {:08b}", a & b);  // 相同位置 均为 1 ，则为1，否则为0
    // println!("a | b value is {:08b}", a | b);  // 相同位置   有 1 ，则为1，否则为0
    // println!("a ^ b value is {:08b}", a ^ b);  // 相同位置 不同   ，则为1，否则为0
    // println!("!b value is {:08b}", !b);        // 把位中的 0 和 1 相互取反，0 置为 1，1置为 0
    // println!("a << b value is {:08b}", a << b);// 所有位向左移动指定位数，右位补0
    // println!("a >> b value is {:08b}", a >> b);// 所有位向右移动指定位数，左位补0
    //注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    // let mut c = 1;
    // c <<= 1;
    // println!("after move left，c: {}", c)
    // overflow报错
    let d: u8 = 255;
    let e = d>>7; // ok
    let e = d<<7; // ok
    let e = d>>8; // overflow
    let e = d<<8; // overflow
}