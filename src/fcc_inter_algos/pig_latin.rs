#[allow(dead_code)]
pub fn pig_latin(str_to_process: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut starts_with_vowel = false;
    for &vowel in vowels.iter() {
        if str_to_process.starts_with(vowel) {
            starts_with_vowel = true;
            break;
        }
    }

    if !starts_with_vowel {
        let mut vowel_encountered = false;

        let mut pre_vowel_chars: Vec<char> = Vec::new();
        let mut post_vowelChars: Vec<char> = Vec::new();

        for character in str_to_process.chars() {
            println!("{}", character);
            if vowels.contains(&character) {
                vowel_encountered = true;
            } else {
                pre_vowel_chars.push(character);
            }

            if vowel_encountered {
                post_vowelChars.push(character);
            }
        }

        let first_part: String = post_vowelChars.iter().copied().collect();
        let second_part: String = pre_vowel_chars.iter().copied().collect();

        format!("{}{}ay", first_part, second_part)
    } else {
        let processed_strings = format!("{}way", str_to_process);
        processed_strings
    }
}
