// Category: algorithms
// Level: Hard
// Percent: 29.96639%

// You are given a 0-indexed integer array nums representing the strength of some heroes. The power of a group of heroes is defined as follows:
//
//
// 	Let i₀, i₁, ... ,ik be the indices of the heroes in a group. Then, the power of this group is max(nums[i₀], nums[i₁], ... ,nums[ik])² * min(nums[i₀], nums[i₁], ... ,nums[ik]).
//
//
// Return the sum of the power of all non-empty groups of heroes possible. Since the sum could be very large, return it modulo 109 + 7.
//
//
// Example 1:
//
// Input: nums = [2,1,4]
// Output: 141
// Explanation:
// 1st group: [2] has power = 2² * 2 = 8.
// 2nd group: [1] has power = 1² * 1 = 1.
// 3rd group: [4] has power = 4² * 4 = 64.
// 4th group: [2,1] has power = 2² * 1 = 4.
// 5th group: [2,4] has power = 4² * 2 = 32.
// 6th group: [1,4] has power = 4² * 1 = 16.
// 7th group: [2,1,4] has power = 4² * 1 = 16.
// The sum of powers of all groups is 8 + 1 + 64 + 4 + 32 + 16 + 16 = 141.
//
//
//
// Example 2:
//
// Input: nums = [1,1,1]
// Output: 7
// Explanation: A total of 7 groups are possible, and the power of each group will be 1. Therefore, the sum of the powers of all groups is 7.
//
//
//
// Constraints:
//
//
// 	1 <= nums.length <= 10⁵
// 	1 <= nums[i] <= 10⁹
//

impl Solution {
    pub fn sum_of_power(nums: Vec<i32>) -> i32 {
        const MOD: u128 = 1_000_000_007;

        let mut powers = nums;
        powers.sort();

        let mut sum: u128 = 0;

        // The first case is groups with only one element.
        for power in powers.iter() {
            let power = *power as u128;
            let addition: u128 = power.wrapping_pow(3) % MOD;
            sum = sum.wrapping_add(addition) % MOD;
        }

        // The second case if groups with more than one element. The power of a group depends only
        // on the min and max value in the group, so we simply need to find out how many groups of
        // each combination there are. For min `a` and max `b`, we can think of how many elements
        // in-between might be visited. Suppose the number of such in-between elements is `n`. We
        // could visit 0 elements (meaning the group is of size 0), and there would be (n choose 0)
        // = 1 such path. We could visit 1 element, and there would be (n choose 1) = n such paths.
        // Thus to compute, the total number of paths involving `a` and `b` as the endpoints, we
        // need to sum (n choose 0) + (n choose 1) + ... + (n choose n), which is equal to 2^n.
        for i in 0..powers.len() {
            let min = (powers[i] as u128) % MOD;
            for j in (i + 1)..powers.len() {
                let max = (powers[j] as u128) % MOD;

                let mut num_in_between = j - i - 1;

                let addition = min.wrapping_mul(max) % MOD;
                let addition = addition.wrapping_mul(max) % MOD;
                let addition = addition.wrapping_shl(num_in_between as u32) % MOD;
                sum = sum.wrapping_add(addition) % MOD;
            }
        }

        sum as i32
    }
}
