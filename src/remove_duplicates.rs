use std::collections::HashSet;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut set = HashSet::new(); 
    let mut last_slot = nums.len();
    let mut total_removed = 0;



    for i in 0..nums.len() {

        if set.contains(&nums[i]) {
                     
            nums[i] = -101;
            total_removed += 1;
            if last_slot == nums.len() {
                last_slot = i;
            } 
        } else {
            set.insert(nums[i]);
            if last_slot != nums.len() {
                nums[last_slot] = nums[i];
                nums[i] = -101;
                last_slot += 1; 
            }
        }
    }


    return nums.len() as i32 - total_removed;
}

#[test]
fn test1() {
    let mut nums = vec![1,1,2];
    let target = 2;

    assert_eq!(remove_duplicates(&mut nums), target);
    assert_eq!(nums, vec![1, 2, -101]);

}

#[test]
fn test2() {
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let target = 5;

    assert_eq!(remove_duplicates(&mut nums), target); 
    assert_eq!(nums, vec![0, 1, 2, 3, 4, -101, -101, -101, -101, -101]);


}
