pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1 .. nums.len() {
            let sum: i32 = nums[i] + nums[j];
            if sum == target {
                return vec![i as i32, j as i32]
            }
        }
    }

    return vec![0, 0]
}

#[test]
fn test1() {
    let nums = vec![2,7,11,15];
    let target = 9 ;

    assert_eq!(two_sum(nums, target), vec![0, 1])

}

#[test]
fn test2() {
    let nums = vec![3,2,4];
    let target = 6;

    assert_eq!(two_sum(nums, target), vec![1,2])

}
