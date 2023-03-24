
pub fn length_of_last_word(s: String) -> i32 {
    let split_s: Vec<&str> = s.trim().split(&" ").collect();        
    split_s[split_s.len() - 1].len() as i32
}

#[test]
fn test1() {
    let s = "Hello World";
    assert_eq!(length_of_last_word(s.to_string()), 5);
}

#[test]
fn test2() {
    let s = "   fly me   to   the moon  ";
    assert_eq!(length_of_last_word(s.to_string()), 4);

}

#[test]
fn test3() {
    let s = "luffy is still joyboy";
    assert_eq!(length_of_last_word(s.to_string()), 6);


}
