struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![] 
    }
}

fn main() {
    let param_1 = vec![2, 7, 11, 15];
    let param_2 = 9;

    let ret = Solution::two_sum(param_1, param_2);
    println!("{:?}", ret); // Output: [0, 1]
}
