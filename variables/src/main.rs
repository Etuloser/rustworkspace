fn main() {
    // rust中, 使用let关键字声明变量, 默认是不可变的
    // 使用mut可是让它变为可变
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // const用于声明常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadowing隐藏和mut区别在于, Shadowing实际上创建了一个新的变量, 只是重名
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let mut spaces = "    ";
    // 这里会编译报错, 因为实际改变了类型
    // spaces = spaces.len()
}
