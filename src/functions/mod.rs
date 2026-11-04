pub fn rand_numbers(num1: i32) -> i32 {
    rand::random_range(1..=num1 + 1)
}
