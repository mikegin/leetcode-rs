
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let remove_symbol: i32 = -1;
    let mut total_removed = 0;

    if nums.len() == 0 {
        return 0;
    }

    for i in 0..nums.len() {
        if nums[i] == val {
            nums[i] = remove_symbol;
            total_removed += 1;
        }
    }

    for i in 0..nums.len() - 1{
        for j in i + 1..nums.len() {
            if nums[i] == remove_symbol {
                nums[i] = nums[j];
                nums[j] = remove_symbol;
            }
        }
   }

    return nums.len() as i32 - total_removed; 
}


#[test]
fn test1() {
    let mut nums = vec![3,2,2,3];
    let val = 3;

    assert_eq!(remove_element(&mut nums, val), 2);
    assert_eq!(nums, vec![2, 2, -1, -1]);
}

#[test]
fn test2() {
    let mut nums = vec![0,1,2,2,3,0,4,2];
    let val = 2;

    assert_eq!(remove_element(&mut nums, val), 5);
    assert_eq!(nums, vec![0,1,3,0,4, -1, -1, -1]);
}
