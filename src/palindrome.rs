
pub fn is_palindrome(s: String) -> bool {
  let s: String = s.chars()
    .map(|x| match x {
        'A'..='Z' => x.to_lowercase().to_string().chars().next().unwrap(), 
        'a'..='z' => x,
        '0'..='9' => x,
        _ => '\0'
    }).collect();

    let s = s.replace("\0", "");

    s == s.chars().rev().collect::<String>()
}

#[test]
fn test1() {
  let s = "A man, a plan, a canal: Panama";
  assert_eq!(is_palindrome(s.to_owned()), true);
}

#[test]
fn test2() {
  let s = "race a car";
  assert_eq!(is_palindrome(s.to_owned()), false);
}

#[test]
fn test3() {
  let s = " ";
  assert_eq!(is_palindrome(s.to_owned()), true);
}

#[test]
fn test4() {
  let s = "0P";
  assert_eq!(is_palindrome(s.to_owned()), true)
}