pub fn str_str(haystack: String, needle: String) -> i32 {
  let haystack_bytes = haystack.as_bytes(); // can do this because all lowercase english letters
  let needle_bytes = needle.as_bytes();

  for (idx, i) in haystack_bytes.iter().enumerate() {
    if i == &needle_bytes[0] && haystack_bytes.len() - idx >= needle_bytes.len() {
      let mut found = true;
      for j in 0..needle_bytes.len() {
        if haystack_bytes[idx + j] != needle_bytes[j] {
          found = false;
          break;
        }
      }

      if found {
        return idx as i32;
      }
    }
  }
  
  -1
}

#[test]
fn test1() {
  let haystack = "leetcode";
  let needle = "leeto";
  
  assert_eq!(str_str(haystack.to_owned(), needle.to_owned()), -1);
}


#[test]
fn test2() {
  let haystack = "leetcode";
  let needle = "leet";
  
  assert_eq!(str_str(haystack.to_owned(), needle.to_owned()), 0);
}

#[test]
fn test3() {
  let haystack = "leetcode";
  let needle = "code";
  
  assert_eq!(str_str(haystack.to_owned(), needle.to_owned()), 4);
}

#[test]
fn test4() {
  let haystack = "aaa";
  let needle = "aaaa";
  
  assert_eq!(str_str(haystack.to_owned(), needle.to_owned()), -1);
}

#[test]
fn test5() {
  let haystack = "mississipi";
  let needle = "issip";
  
  assert_eq!(str_str(haystack.to_owned(), needle.to_owned()), 4);
}