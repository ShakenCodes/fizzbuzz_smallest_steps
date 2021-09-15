pub fn fizzbuzz_to_thirty() -> String {
    let mut result = String::new();
    for i in 1..=30 {
        result.push_str(&fizzbuzz(i));
    };
    result
}
fn fizzbuzz(n: u32) -> String {
    match (n % 5, n % 3) {
        (0, 0) => "fizzbuzz\n".to_string(),
        (_, 0) => "fizz\n".to_string(),
        (0, _) => "buzz\n".to_string(),
        (_, _) => format!("{}\n", n),
    }
}