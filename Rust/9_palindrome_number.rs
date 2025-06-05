struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let str_x = x.to_string();

        str_x == str_x.chars().rev().collect::<String>()
    }
}

fn main() {
    let x = 121;
    let result = Solution::is_palindrome(x);
    println!("{}", result); // Output: true
}
