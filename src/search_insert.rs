
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;

    if nums[0] > target {
        return 0;
    }

    if nums[nums.len() - 1] < target {
        return nums.len() as i32;
    }

    while l <= r {
        let m = (r + l) / 2;
        if target == nums[m] {
            return m as i32;
        } else if target < nums[m] {
            r = m - 1;
        }   else {
            l = m + 1;
        }
    }

    return l as i32 as i32;
}


#[test]
fn test1() {
    let nums = vec![1,3,5,6];
    let target = 5;

    assert_eq!(search_insert(nums, target), 2);
}

#[test]
fn test2() {
    let nums = vec![1, 3, 5, 6];
    let target = 2;

    assert_eq!(search_insert(nums, target), 1);

}

#[test]
fn test3() {
    let nums = vec![1, 3, 5, 6];
    let target = 7;

    assert_eq!(search_insert(nums, target), 4);

}
