/*
 * @lc app=leetcode.cn id=925 lang=rust
 *
 * [925] 长按键入
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        // 第一点：能知晓在迭代中，区分元素中的边界
        let mut last_mark = char::MAX;
        let mut name_grups: Vec<Vec<char>> = vec![];
        for c in name.chars() {
            if c == last_mark {
                name_grups.last_mut().unwrap().push(c);
            } else {
                name_grups.push(vec![c]);
            }
            last_mark = c;
        }
        let mut last = char::MAX;
        let mut typed_grups: Vec<Vec<char>> = vec![];
        for c in typed.chars() {
            if c == last {
                typed_grups.last_mut().unwrap().push(c);
            } else {
                typed_grups.push(vec![c]);
            }
            last = c;
        }

        // 第二点：拿到分组后，匹配
        if name_grups.len() != typed_grups.len() {
            return false;
        }
        for (a, b) in name_grups.iter().zip(typed_grups.iter()) {
            //如果typed被连续 按下的字符与name 对应的序列不同
            //或者 typed连续的字符小于name中存在连续字符的长度
            if b.len() < a.len() || a.get(0) != b.get(0) {
                return false;
            }
        }
        true
    }
}
// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn one() {
        assert_eq!(
            Solution::is_long_pressed_name("alex".to_string(), "aaleex".to_string()),
            true
        )
    }

    #[test]
    fn two() {
        assert_eq!(
            Solution::is_long_pressed_name("leelee".to_string(), "lleeelee".to_string()),
            true
        )
    }

    #[test]
    fn three() {
        assert_eq!(
            Solution::is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()),
            false
        )
    }
}
