mod tests {
    use rusty_ground::fcc_inter_algos;

    #[test]
    fn word_starts_with_consonant_followed_by_vowel() {
        let result = fcc_inter_algos::pig_latin("banana");

        let correct_ans = "ananabay";

        assert!(
            result == correct_ans,
            "`Banana` should be processed to `{}`",
            correct_ans
        );
    }

    #[test]
    fn word_starts_with_consonant_followed_by_a_few_consonant() {
        let result = fcc_inter_algos::pig_latin("The");

        let correct_ans = "eThay";

        assert!(
            result == correct_ans,
            "`The` should be processed to `{}`",
            correct_ans
        );
    }

    #[test]
    fn word_starts_with_vowel() {
        let result = fcc_inter_algos::pig_latin("age");

        let correct_ans = "ageway";

        assert_eq!(result, correct_ans);
    }

    #[test]
    fn word_starts_with_vowel_followed_by_more_vowel() {
        let result = fcc_inter_algos::pig_latin("Aeroplane");

        let correct_ans = "Aeroplaneway";

        assert_eq!(result, correct_ans);
    }
}
