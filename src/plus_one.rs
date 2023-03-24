pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut remainder = 1;
    let mut result = digits.clone();
    println!("{:?}", digits);
    for x in (0..result.len()).rev() {
        let sum = result[x] + remainder;
        result[x] = sum % 10;
        println!("x: {:?}, remainder: {:?}, sum: {:?}", x, remainder, sum);
        remainder = if sum >= 10 { 1 } else { 0 };
        if remainder == 0 {
            break;
        }
    }
    
    if remainder == 1 {
        result.splice(0..0, [1]);

    }
    
    result
}


#[test]
fn test1() {
    let digits = vec![1,2,3];
    assert_eq!(plus_one(digits), vec![1,2,4]);

}

#[test]
fn test2() {
    let digits = vec![4,3,2,1];
    assert_eq!(plus_one(digits), vec![4,3,2,2]);

}

#[test]
fn test3() {
   let digits = vec![9];
    assert_eq!(plus_one(digits), vec![1,0]);


}

#[test]
fn test4() {
   let digits = vec![9, 9, 9, 9];
    assert_eq!(plus_one(digits), vec![1,0,0,0,0]);


}
