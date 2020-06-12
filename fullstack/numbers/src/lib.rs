pub fn say_hello() {
    println!("Hello, world!");
}

pub fn print() {
    let vector_numbers = vec![1, 2, 3, 4, 5];
    output_sequence(&vector_numbers);
    let array_number = [1, 2, 3, 4, 5];
    output_sequence(&array_number);
}

fn output_sequence(numbers: &[u8]) {
    for n in numbers {
        println!("{}", n);
    }
}
