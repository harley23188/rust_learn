// struct Struct {
//     e: i32
// }

fn main() {
    // 1. 可变不可变
    // let x = 5;
    // 使用 mut 变得可变
    // let mut x = 5;
    // println!("the value of x is: {}",x);
    // x = 6;
    // println!("the value of x is: {}",x);

    // 2. 使用下划线开头忽略未使用的变量
    // 告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头
    // let _a = 100;
    // 这里会被警告，但是能编译
    // let y = 10; 
    
    // 3. 变量解构
    // let (a, mut b): (bool, bool) = (true, false);
    // println!("{} and {}",a ,b);
    // b = true;
    // assert_eq!(a, b);
    
    // 4. 解构式赋值
    // let (a, b, c, d, e);
    // (a, b) = (1, 2);  // 元组 
    // [c, .., d, _] = [1, 2, 3, 4, 5];  // 切片
    // Struct {e, ..} = Struct { e: 5 };  // 结构体
    // assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e])

    // 5. 变量和常量之间的差异
    // 常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
    // 常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注
    // const MAX_POINT: u32 = 100_000;

    // 6. 变量遮蔽(shadowing)
    //变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字
    // let x = 5;
    // let x = 9;  // 覆盖 
    // println!(" x1 的值为：{}",x);
    // {
    //    let x = x * 2;  // 覆盖
    //     println!(" x2 的值为：{}",x)
    // }
    // println!(" x3 的值为：{}",x);
    // 6.2 let覆盖可以替换类型，相当于重新分配内存
    // 字符串类型
    // let spaces = "   ";
    // usize数值类型
    // let spaces = spaces.len();
    // println!("spaces：{}", spaces)
    // 6.3 这里会报错，因为spaces已经定义为字符串了
    let mut spaces = "   ";
    spaces = spaces.len();
}
