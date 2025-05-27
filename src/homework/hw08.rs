fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    }
    let upper_limit = (number as f64).sqrt() as i32;
    for divisor in 2..=upper_limit {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}
#[test]
fn test_is_prime() {
    let test_data = [
        (-10, false),
        (-1, false),
        (0, false),
        (1, false),
        (2, true),
        (11, true),
        (15, false),
        (17, true),
        (25, false),
        (31, true),
        (49, false),
        (97, true),
        (121, false),
        (7919, true),
        (8000, false),
    ];
    test_data
        .iter()
        .for_each(|(n, expected)| {
            assert_eq!(is_prime(*n), *expected, "Failed for number: {}", n);
        });
}
