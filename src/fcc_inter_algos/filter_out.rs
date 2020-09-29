use std::fmt::Display;

#[allow(dead_code)]
pub fn filter_out<'a, T>(
    array_to_filter: &'a mut Vec<T>,
    things_to_filter: &'a mut Vec<T>,
) -> &'a mut Vec<T>
where
    T: Display + PartialOrd,
{
    array_to_filter.retain(|element| !things_to_filter.contains(element));

    array_to_filter
}
