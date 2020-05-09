fn main() {
    let use_closure = || {
        println!("This is a closure!");
    };
    use_closure();
    println!("Hello world!");
    // 闭包定义可以为每个参数和返回值类型推导一个具体的类型，但是不能推导两次
    let add_one_v2 = |x:u32| -> u32 { x+1 };
    let add_one_v3 = |x| {x+1};
    let add_one_v4 = |x| x+1;

    let a = add_one_v1(5);
    let b = add_one_v2(5);
    let c = add_one_v3(5);
    let d = add_one_v4(5);
    println!("a={} b={} c={} d={}", a, b, c, d);
    // 不能推导两次的例子
    let example_closure = |x|x;
    let s = example_closure(String::from("hello"));
    println!("s={}", s);
    let n = example_closure(5);
    println!("s={}", s);// 报错
}

// 语法格式
fn add_one_v1(x: u32) -> u32 {
    x+1
}
