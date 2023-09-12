use core::num;

/*
 * @lc app=leetcode.cn id=448 lang=rust
 *
 * [448] 找到所有数组中消失的数字
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let mut not_hit_vec: Vec<i32> = vec![];
        // find who no hit_back

        for start_index in 0..nums.len() {
            if not_hit_vec.contains(&(start_index as i32 + 1)) {
                not_hit_vec.remove(start_index);
            }
            let mut cur_value = nums[start_index];
            if cur_value != start_index as i32 + 1 {
                not_hit_vec.push(start_index as i32 + 1);

                // no right num hitten, go jump
                while nums[cur_value as usize - 1] != cur_value {}
                // so here is hit right num
            }
        }
        vec![]
    }
}
// @lc code=end

mod tests {
    use crate::Solution;

    #[test]
    fn my_fn() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }

    #[test]
    fn my_fn2() {
        assert_eq!(Solution::find_disappeared_numbers(vec![2, 1]), vec![]);
    }

    #[test]
    fn my_fn3() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![1, 1, 2, 2]),
            vec![3, 4]
        );
    }

    #[test]
    fn my_fn4() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 7, 2, 3, 1]),
            vec![5, 6, 8]
        );
    }
}

pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    let l = nums.len();
    let mut i = 0;
    while i < l {
        loop {
            let num = nums[i];
            let target_i = (num - 1) as usize;
            if target_i != i {
                let target_num = nums[target_i];
                if target_num != num {
                    nums.swap(i, target_i);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        i += 1;
    }
    let mut result = Vec::new();
    for i in 0..l {
        if nums[i] != i as i32 + 1 {
            result.push(i as i32 + 1);
        }
    }
    result
}

pub fn find_disappeared_numbers2(mut nums: Vec<i32>) -> Vec<i32> {
    // 43278231
    // _3248231
    // _3248271
    // _3348271
    // _2348271
    //  *3
    //     _
    // _2348271
    // _234_278
    // 1234_278
    // 1234__78
    let mut res = vec![];
    println!("测试： {:?}", nums);
    for i in 0..nums.len() {
        if i + 1 != nums[i] as usize {
            println!("此处想等于 {} 但是是 {}", i + 1, nums[i]);
            res.push(i); // 4 != 0 -> [0]
            println!("抛出数字 : {} 序数是: {} , 暂存 : {:?} ", nums[i], i, res);
            let mut value = nums[i].clone(); // 4
            nums[i] = -1;
            while value != nums[value as usize - 1] as i32 {
                println!(
                    "当前要跳转数字 : {} , 跳到index {:?} -是与{:?} 替换     暂存：{:?}",
                    value,
                    value - 1,
                    nums[value as usize - 1],
                    res
                );
                if nums[value as usize] == -1 {
                    nums[value as usize] = value;
                    break;
                }
                if res.contains(&((value - 1) as usize)) {
                    println!("如果在目前要跳的数字在暂存中已经存在，属于正确的位置，应当去掉暂存");
                    res = vec![i];
                    if res[0] == value as usize - 1 {
                        res = vec![];
                    }
                    nums[value as usize - 1] = value; // nums[3] = 4
                    break;
                }
                // 4 != nums[4-1] = 7
                let pop_out_value = nums[value as usize - 1]; // 7
                nums[value as usize - 1] = value; // nums[3] = 4
                res.push(value as usize - 1); // 7 -> [0,3]
                value = pop_out_value; // 4 -> 7
            }
            println!("one tik : {:?} , {:?}", i, res);
        }
        println!("\n 每步nums: {:?}", nums);
    }
    res.iter_mut().map(|x| (*x as i32 + 1)).collect()
}
