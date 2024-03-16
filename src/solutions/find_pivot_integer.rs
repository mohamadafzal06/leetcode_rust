//2485. Find the Pivot Integer
pub fn pivot_integer(n: i32) -> i32 {
    for i in 0..n {
        for x in 1..n + 1 {
            let sum_left = Solution::sum_to_n(x);
            let sum_right = Solution::sum_to_n(n) - Solution::sum_to_n(x - 1);
            if sum_right == sum_left {
                x
            }
        }
    }
    -1
}

fn sum_to_n(n: i32) -> i32 {
    (n * (n + 1)) / 2
}
