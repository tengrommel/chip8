pub fn say_hello() {
    println!("Hello, world!");
}

pub fn print() {
    let numbers = [1, 2, 3, 4, 5];
    output_sequence(numbers);
    output_sequence(numbers);
}

fn output_sequence(numbers: [u8; 5]) {
    for n in numbers.iter() {
        println!("{}", n);
    }
}
