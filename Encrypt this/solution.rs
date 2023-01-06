fn encrypt_this(text: &str) -> String {
    let split = text.split_whitespace();
    let mut string_to_return = String::new();
    for mut word in split {
        let mut letters: Vec<char> = word.chars().collect();
        if word.len() > 1 {
            let letter_a = letters[1];
            let letter_b = letters[letters.len() - 1];
            letters[1] = letter_b;
            letters.pop();
            letters.push(letter_a);
        }
        let letters_string: String = letters.into_iter().collect();
        word = letters_string.as_str();
        let mut new_string = String::from(word);
        let first_char = (get_char(&new_string) as u32).to_string();
        new_string.remove(0);
        new_string.insert_str(0, first_char.as_str());
        string_to_return.push_str(new_string.as_str());
        string_to_return.push(' ');
    }
    string_to_return.pop();
    string_to_return
}

fn get_char(data: &String) -> char {
    data.chars().nth(0).unwrap()
}
