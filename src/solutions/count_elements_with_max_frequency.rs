// 3005. Count Elements With Maximum Frequency

pub fn get_frequency(nums: &Vec<i32>) -> (HashMap<i32, i32>, i32) {
    let mut max_frequency: i32 = 0;
    let mut count = HashMap::new();

    for &v in nums.iter() {
        let counter = count.entry(v).or_insert(0);
        *counter += 1;
        if *counter > max_frequency {
            max_frequency = *counter;
        }
    }
    return (count, max_frequency);
}
pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let (count, max_frequency) = Solution::get_frequency(&nums);
    let mut result: i32 = 0;
    for (k, v) in count.into_iter() {
        if v == max_frequency {
            result = result + k;
        }
    }

    return result;
}
