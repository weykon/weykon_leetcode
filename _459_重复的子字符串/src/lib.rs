use std::{borrow::BorrowMut, collections::HashMap};

/*
 * @lc app=leetcode.cn id=459 lang=rust
 *
 * [459] 重复的子字符串
 */
struct Solution;
// @lc code=start
impl Solution {

    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        let mut i = 1;

        while i * 2 <= n {
            if n % i == 0 && s == s[0..i].repeat(n / i) {
                return true;
            }

            i += 1;
        }

        false
    }
}
// @lc code=end


fn head_tail (s: String) -> bool {

    // 去头尾
    let mut t = String::from(&s);
    // abcabc  bcabc-abcab
    // abc bc-ab
    t.push_str(&s);
    let l = t.len();
    if let Some(v) = t.get(1..(l-1)) {
        //println!("{}",v);
        return v.contains(&s);
    }
    false
}

fn mark (s: String) -> bool {
    // 下面的做法类似于一个递进的场景，
    // 是缺乏场景意识的模式
    // abcabc
        // abaaba c ()
        // a,
        // |(a) b?| (a,b),
        // |(a,b) a?| mark1 |
        // |(a,b,a) a?| mark2 |
        // |(a,b,a,b) b?| mark3 |
        // |(a,b,a,a,b) a?|
        // mark4 if end .
        // but c? none ,. temp full -> false

        let mut save: Vec<u8> = vec![];
        let mut mark = 0;
        for b in s.bytes() {
            if save.len() == 0 {
                save.push(b);
            } else {
                for (i, s) in save.iter_mut().enumerate().map(|(i, s)| (i, s)) {
                    {
                        // https://course.rs/compiler/fight-with-compiler/borrowing/ref-exist-in-out-fn.html
                        if *s != b {
                            save.push(b);
                        } else {
                            save.push(b);
                            mark = i;
                        }
                    }
                }
            }
        }
        mark == save.len()
}


fn mirror (s:String) -> bool{ 

    // 这种类似于镜像的想法，和头尾计算差不多，
    // 毕竟这个场景确实符合这种想法
    let n = s.len();
    let mut i = 1;

    while i * 2 <= n {
        if n % i == 0 && s == s[0..i].repeat(n / i) {
            return true;
        }

        i += 1;
    }

    false
}

fn one_line(s:String) -> bool { 
    (s.clone() + &s)[1..s.len()*2-1].contains(&s)
}