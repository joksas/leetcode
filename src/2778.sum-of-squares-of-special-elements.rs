impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let mut sum_of_squares = 0;

        for (i, num) in nums.iter().enumerate() {
            if nums.len() % (i + 1) == 0 {
                sum_of_squares += num * num;
            }
        }

        sum_of_squares
    }
}
