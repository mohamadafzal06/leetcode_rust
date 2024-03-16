//930. Binary Subarrays With Sum
pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let mut freq = HashMap::new();
    freq.insert(0, 1);
    let mut result: i32 = 0;
    let mut current_sum: i32 = 0;
    for i in 0..nums.len() {
        current_sum += nums[i];
        let remaining: i32 = current_sum - goal;
        match freq.get(&remaining) {
            Some(rem) => result += rem,
            None => result += 0,
        }
        match freq.get(&current_sum) {
            Some(count) => {
                freq.insert(current_sum, count + 1);
            }
            None => {
                freq.insert(current_sum, 1);
            }
        }
    }
    result
}
