#[allow(dead_code)]
pub fn sum_in_range(range: [i8; 2]) -> i8 {
    (range[0]..=range[1]).sum()
}
