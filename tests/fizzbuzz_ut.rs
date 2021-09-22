#[cfg(test)]
mod tests {
    use fizzbuzz::fizzbuzz_to_thirty;

    #[test]
    fn test_fizzbuzz_to_thirty() {
        assert_eq!(
            "1\n\
             2\n\
             fizz\n\
             4\n\
             buzz\n\
             fizz\n\
             7\n\
             8\n\
             fizz\n\
             buzz\n\
             11\n\
             fizz\n\
             13\n\
             14\n\
             fizzbuzz\n\
             16\n\
             17\n\
             fizz\n\
             19\n\
             buzz\n\
             fizz\n\
             22\n\
             23\n\
             fizz\n\
             buzz\n\
             26\n\
             fizz\n\
             28\n\
             29\n\
             fizzbuzz\n\
             ",
            fizzbuzz_to_thirty());
    }
}
