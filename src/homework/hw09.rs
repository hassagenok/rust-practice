fn rotate(s: String, shift: i32) -> String {
    let string_length = s.chars().count() as i32;
    if string_length == 0 {
        return s;
    }
    
    let normalized_shift = ((shift % string_length) + string_length) % string_length;
    
    let characters: Vec<char> = s.chars().collect();
    let split_position = string_length as usize - normalized_shift as usize;
    
    characters[split_position..]
        .iter()
        .chain(characters[..split_position].iter())
        .collect()
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
        (16, "abcdefgh"),
        (-16,"abcdefgh"),
        (3,  "fghabcde"),
    ];
    for (shift, expected) in shifts.iter() {
        assert_eq!(rotate(s.to_string(), *shift), expected.to_string());
    }
}
