fn main22(operation: char, value1: i32, value2: i32) -> i32 {
    match operation {
        '+' => value1 + value2,
        '-' => value1 - value2,
        '*' => value1 * value2,
        '/' => value1 / value2,
        _ => panic!("Invalid operation"),
    }
}

