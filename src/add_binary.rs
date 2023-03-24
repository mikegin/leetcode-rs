pub fn add_binary(a: String, b: String) -> String {
    let l = std::cmp::max(a.len(), b.len());

    let mut result = "".to_owned();

    let mut carry: u8 = 0;


    fn get_val(s: &str, idx: usize) -> String {
        if s.len() as i32 - 1 - idx as i32 >= 0 as i32 {
            return std::str::from_utf8(&[s.as_bytes()[s.len() - 1 - idx]]).unwrap().to_owned();
        }
        return "0".to_owned();
    };

    let mut add = |a_val: &str, b_val: &str| -> char {
        let a_num = a_val.parse::<u8>().unwrap();
        let b_num = b_val.parse::<u8>().unwrap();

        let sum = a_num + b_num + carry;

        if sum == 3 {
            carry = 1;
            return '1';
        } else if sum == 2 {
            carry = 1;
            return '0';
        } else if sum == 1 {
            carry = 0;
            return '1';
        } else {
            carry = 0;
            return '0';
        }
    };

    for i in 0..l {
        let a_val = get_val(&a, i);
        let b_val = get_val(&b, i);

        let sum_char = add(&a_val, &b_val);

        result.insert(0, sum_char);
    }

    if carry == 1 {
        result.insert(0, '1');
    }

    result
}

#[test]
fn test1() {
    let a = "11";
    let b = "1";
    assert_eq!(add_binary(a.to_string(), b.to_string()), "100");
}

#[test]
fn test2() {
    let a = "1010";
    let b = "1011";
    assert_eq!(add_binary(a.to_string(), b.to_string()), "10101");
}