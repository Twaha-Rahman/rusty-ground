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

        let mut pre_vowel_chars: String = String::new();
        let mut post_vowel_chars: String = String::new();

        for character in str_to_process.chars() {
            if vowels.contains(&character) {
                vowel_encountered = true;
            }

            if vowel_encountered {
                post_vowel_chars.push(character);
            } else {
                pre_vowel_chars.push(character);
            }
        }

        format!("{}{}ay", post_vowel_chars, pre_vowel_chars)
    } else {
        let processed_strings = format!("{}way", str_to_process);
        processed_strings
    }
}
