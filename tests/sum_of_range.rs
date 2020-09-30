mod tests {
    use rusty_ground::fcc_inter_algos;

    #[test]
    fn adds_series_with_positive_number() {
        let result = fcc_inter_algos::sum_in_range([1, 4]);

        assert_eq!(result, 10);
    }

    #[test]
    fn adds_series_with_negative_number() {
        let result = fcc_inter_algos::sum_in_range([-3, -1]);

        assert_eq!(result, -6);
    }

    #[test]
    fn adds_series_with_positive_and_negative_number() {
        let result = fcc_inter_algos::sum_in_range([-3, 2]);

        assert_eq!(result, -3);
    }

    #[test]
    fn adds_series_with_no_number() {
        let result = fcc_inter_algos::sum_in_range([0, 0]);

        assert_eq!(result, 0);
    }
}
