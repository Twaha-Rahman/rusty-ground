mod tests {
    use rusty_ground::fcc_inter_algos;

    #[test]
    fn it_works() {
        let result = fcc_inter_algos::pig_latin("banana");

        assert_eq!(result, "ananabay");
    }
}
