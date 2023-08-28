/*
 * @lc app=leetcode.cn id=67 lang=rust
 *
 * [67] 二进制求和
 */

struct Solution;
// @lc code=start
use std::cmp;
impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        let (upper, lowwer) = if a.len() > b.len() { (a, b) } else { (b, a) };
        let (b, mut s) = upper
            .chars()
            .rev()
            .zip(lowwer.chars().rev().chain(std::iter::repeat('0')))
            .fold((false, String::new()), |mut acc, (u, l)| {
                match (acc.0, u, l) {
                    (false, '1', '1') => {
                        acc.0 = true;
                        acc.1.push('0');
                        (acc.0, acc.1)
                    }
                    (false, '1', '0') => {
                        acc.0 = false;
                        acc.1.push('1');
                        (acc.0, acc.1)
                    }
                    (false, '0', '1') => {
                        acc.0 = false;
                        acc.1.push('1');
                        (acc.0, acc.1)
                    }
                    (false, '0', '0') => {
                        acc.0 = false;
                        acc.1.push('0');
                        (acc.0, acc.1)
                    }
                    (true, '1', '1') => {
                        acc.0 = true;
                        acc.1.push('1');
                        (acc.0, acc.1)
                    }
                    (true, '1', '0') => {
                        acc.0 = true;
                        acc.1.push('0');
                        (acc.0, acc.1)
                    }
                    (true, '0', '1') => {
                        acc.0 = true;
                        acc.1.push('0');
                        (acc.0, acc.1)
                    }
                    (true, '0', '0') => {
                        acc.0 = false;
                        acc.1.push('1');
                        (acc.0, acc.1)
                    }
                    _ => panic!(),
                }
            });
        if b {
            s.push('1');
            s.chars().rev().collect()
        } else {
            s.chars().rev().collect()
        }
    }
}
// @lc code=end

fn std_at_i128(a: String, b: String) -> String {
    // 此方测试的样本中无效了
    let a_num = i128::from_str_radix(&a, 2).unwrap();
    let b_num = i128::from_str_radix(&b, 2).unwrap();

    format!("{:b}", (a_num + b_num)).to_string()
}

mod tests {
    use crate::Solution;

    #[test]
    fn tt() {
        assert_eq!(
            "10101",
            Solution::add_binary("1010".to_owned(), "1011".to_owned())
        );
    }
}

fn fast(mut a: String, mut b: String) -> String {
    // 确保 a <= b
    if a.len() > b.len() {
        let temp = a;
        a = b;
        b = temp;
    }
    let mut vec = Vec::new();
    let mut carry = 0;
    for (x, y) in a
        .chars()
        .rev()
        .chain(std::iter::repeat('0').take(b.len() - a.len()))
        .zip(b.chars().rev())
    {
        let x = x as u8 - b'0';
        let y = y as u8 - b'0';
        vec.push((x ^ y ^ carry + b'0') as char);
        carry = if x + y + carry > 1 { 1 } else { 0 };
    }
    // 最后的进位
    if carry > 0 {
        vec.push('1');
    }
    vec.iter().rev().collect()
}

pub fn mid(a: String, b: String) -> String {
    let first = cmp::max_by_key(&a, &b, |key| key.len());
    let second = cmp::min_by_key(&a, &b, |key| key.len());
    let mut r_vec: Vec<char> = Vec::new();
    let mut carry: u8 = 0;
    let mut first_chars = first.chars().rev();
    let mut second_chars = second.chars().rev();
    while let Some(x) = first_chars.next() {
        let y = second_chars.next().unwrap_or('0');
        let mut r = match (x, y) {
            ('0', '1') => 1,
            ('1', '0') => 1,
            ('1', '1') => 2,
            ('0', '0') => 0,
            _ => 0,
        } + carry;
        carry = 0;
        if r >= 2 {
            r -= 2;
            carry = 1
        }
        r_vec.push(r.to_string().chars().next().unwrap_or('0'));
    }
    if carry == 1 {
        //循环完毕后,检查进位标志,如果是1, 就多插入一位
        r_vec.push('1');
    }
    r_vec.iter().rev().map(ToString::to_string).collect()
}
