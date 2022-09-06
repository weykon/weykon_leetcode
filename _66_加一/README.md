```rust 
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        match digits.iter().enumerate().rev().skip_while(|(i,&x)|9==x).next() {
            Some((i,&a))=>
                    [digits[..i].iter().map(|x| x.clone()).collect::<Vec<i32>>().as_slice()
                        , vec![digits[i] + 1].as_slice(), vec![0; digits.len() - (i + 1)].as_slice()].concat()
            ,None=> [vec![1,].as_slice(),vec![0;digits.len()].as_slice()].concat()
        }
    }
}
```
作者：java_Lee
链接：https://leetcode.cn/problems/plus-one/solution/rust-ji-jian-5xing-ban-by-java_lee-qvyc/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
=> 100% , 10% 

```rust
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut i = digits.len() - 1;
        loop {
            if digits[i] != 9 {
                digits[i] += 1;
                return digits;
            } else {
                digits[i] = 0;
                if i > 0 {
                    i -= 1;
                } else {
                    digits.insert(0, 1);
                    return digits;
                }
            }
        }
    }
}
```
作者：S3Z09tHfsx
链接：https://leetcode.cn/problems/plus-one/solution/rustti-jie-by-s3z09thfsx-5/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

=> 100% , 36% 


在我的思维比较迟钝，这里的思维是，忘记了在遍历中
提早退出循环，沉溺于单个操作中。