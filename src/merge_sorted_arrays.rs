pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
  //start with pointers for each vector
  //while pointer counter is less than m/n, compare and swap if needed, increment counter accordingly
  //once m is reached, stop swapping and just insert into nums1 replacing the zeroes

  let mut n1: usize = 0;
  let mut n2: usize = 0;

  let mut res: Vec<i32> = vec![];

  while (n1 as i32) < m || (n2 as i32) < n {
    if (n1 as i32) < m && (n2 as i32) < n {
      if nums2[n2] < nums1[n1] {
       res.push(nums2[n2]);
        
        n2 += 1;
      } else {
        res.push(nums1[n1]);

        n1 += 1;
      }
    } else if (n1 as i32) < m {
      res.push(nums1[n1]);

      n1 += 1;
    } else {
      res.push(nums2[n2]);

      n2 += 1;
    }
  }

  *nums1 = res;
}

#[test]
fn test1() {
  let mut nums1 = vec![1,2,3,0,0,0];
  let m = 3;
  let mut nums2 = vec![2,5,6];
  let n = 3;

  merge(&mut nums1, m, &mut nums2, n);

  assert_eq!(nums1, vec![1,2,2,3,5,6]);
}

#[test]
fn test2() {
  let mut nums1 = vec![1];
  let m = 1;
  let mut nums2 = vec![];
  let n = 0;

  merge(&mut nums1, m, &mut nums2, n);

  assert_eq!(nums1, vec![1]);
}

#[test]
fn test3() {
  let mut nums1 = vec![0];
  let m = 0;
  let mut nums2 = vec![1];
  let n = 1;

  merge(&mut nums1, m, &mut nums2, n);

  assert_eq!(nums1, vec![1]);
}

#[test]
fn test4() {
  let mut nums1 = vec![4,5,6,0,0,0];
  let m = 3;
  let mut nums2 = vec![1,2,3];
  let n = 3;

  merge(&mut nums1, m, &mut nums2, n);

  assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
}