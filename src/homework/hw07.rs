fn invert_the_case(string: String) -> String {
    let mut inverted_string = String::with_capacity(string.len());

    for ch in string.chars() {
        if ch.is_uppercase() {
            inverted_string.extend(ch.to_lowercase());
        } else if ch.is_lowercase() {
            inverted_string.extend(ch.to_uppercase());
        } else {
            inverted_string.push(ch);
        }
    }

    inverted_string
}

#[test]
fn test() {
    let data = [
        ("Hello", "hELLO"),
        ("Привіт", "пРИВІТ"),  
    ];

    for &(input, expected) in data.iter() {
        assert_eq!(invert_the_case(input.to_string()), expected.to_string());
        assert_eq!(invert_the_case(expected.to_string()), input.to_string());
    }
}
