fn solution(s: &str) -> Vec<String> {
    let mut vec_res: Vec<String> = Vec::new();
    let mut strings_to_add: String = String::new();
    let is_string_length_even: bool = s.chars().count() % 2 == 0;
    for chr in s.chars() {
        strings_to_add.push(chr);
        if strings_to_add.chars().count() > 1 {
            vec_res.push(strings_to_add);
            strings_to_add = "".to_string();
        }
    }
    if is_string_length_even == false {
            let mut last_string: String = String::new();
            let last_char = s.chars().last().unwrap();
            last_string.push(last_char);
            last_string.push('_');
            vec_res.push(last_string);
        }
    vec_res
}