fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    // 最后一个表达式后添加分号，隐含其返回值为unit
    3.1415926;
}

fn main() {
    // Rust下函数是一等公民,可以作为参数或者返回值
    // Rust函数参数的类型和返回值的类型豆必须显式定义,如果没有返回值可以省略,返回unit
    // 函数内部如果提前返回，需要用return关键字，否则最后一个表达式就是其返回值
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));

    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
        pi();
    };
    println!(
        "is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}",
        is_pi, is_unit1, is_unit2
    );
}
