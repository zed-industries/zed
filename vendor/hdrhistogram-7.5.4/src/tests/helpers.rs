use super::Histogram;

#[cfg(test)]
pub fn histo64(
    lowest_discernible_value: u64,
    highest_trackable_value: u64,
    num_significant_digits: u8,
) -> Histogram<u64> {
    Histogram::<u64>::new_with_bounds(
        lowest_discernible_value,
        highest_trackable_value,
        num_significant_digits,
    )
    .unwrap()
}
