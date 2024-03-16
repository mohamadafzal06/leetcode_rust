#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_subarrays_with_sum() {
        assert_eq!(4, Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2));
    }

    #[test]
    fn test_find_pivot_integer() {
        assert_eq!(4, pivot_integer(110));
    }
}
