//38. Product of Array Except Self
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix: Vec<i32> = vec![0; nums.len()];
    let mut suffix: Vec<i32> = vec![0; nums.len()];
    let mut ans: Vec<i32> = vec![0; nums.len()];

    prefix[0] = nums[0];
    suffix[nums.len() - 1] = nums[nums.len() - 1];
    for i in 1..nums.len() {
        prefix[i] = prefix[i - 1] * nums[i];
    }

    for j in (0..nums.len() - 1).rev() {
        suffix[j] = suffix[j + 1] * nums[j];
    }

    let mut a: i32;
    let mut b: i32;
    for i in 0..nums.len() {
        a = 1;
        b = 1;
        if (i - 1) >= 0 {
            a = prefix[i - 1];
        } else if i + 1 < nums.len() {
            b = suffix[i + 1];
        } else {
            continue;
        }

        ans[i] = a * b;
    }

    return ans;
}
