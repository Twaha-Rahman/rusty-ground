use std::fmt::Display;

#[allow(dead_code)]
pub fn array_diff<'a, T: PartialEq + Display>(
    first_array: &'a [T],
    second_array: &'a [T],
) -> Vec<&'a T> {
    let mut diff: Vec<&T> = vec![];

    for element in first_array.iter() {
        if !second_array.contains(&element) {
            diff.push(element);
        }
    }

    for element in second_array.iter() {
        if !first_array.contains(&element) {
            diff.push(element);
        }
    }

    diff
}
