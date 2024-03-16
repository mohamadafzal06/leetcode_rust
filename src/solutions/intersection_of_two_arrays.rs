// 349. Intersection of Two Arrays
#[warn(dead_code)]
fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut count: HashSet<i32> = HashSet::new();
    for (i, _) in nums1.iter().enumerate() {
        for (j, _) in nums2.iter().enumerate() {
            if nums1[i] == nums2[j] {
                count.insert(nums1[i]);
            }
        }
    }
    res.extend(count.into_iter());

    return res;
}
