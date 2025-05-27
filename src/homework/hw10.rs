fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

#[test]
fn test() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
        (0, true),
        (7, true),
        (1001, true),
        (12321, true),
        (12345, false),
        (1000, false),
    ];
    data
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
}
