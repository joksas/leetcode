// Category: algorithms
// Level: Easy
// Percent: 32.753258%

// You are given a 0-indexed integer array nums. A subarray s of length m is called alternating if:
//
//
// 	m is greater than 1.
// 	s₁ = s₀ + 1.
// 	The 0-indexed subarray s looks like [s₀, s₁, s₀, s₁,...,s(m-1) % 2]. In other words, s₁ - s₀ = 1, s₂ - s₁ = -1, s₃ - s₂ = 1, s₄ - s₃ = -1, and so on up to s[m - 1] - s[m - 2] = (-1)m.
//
//
// Return the maximum length of all alternating subarrays present in nums or -1 if no such subarray exists.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
//
// Example 1:
//
// Input: nums = [2,3,4,3,4]
// Output: 4
// Explanation: The alternating subarrays are [3,4], [3,4,3], and [3,4,3,4]. The longest of these is [3,4,3,4], which is of length 4.
//
//
// Example 2:
//
// Input: nums = [4,5,6]
// Output: 2
// Explanation: [4,5] and [5,6] are the only two alternating subarrays. They are both of length 2.
//
//
//
// Constraints:
//
//
// 	2 <= nums.length <= 100
// 	1 <= nums[i] <= 10⁴
//

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut prev = nums[0];
        let mut prev_change_positive: Option<bool> = None;
        let mut current_chain = 1;
        let mut longest_chain = 1;

        for num in nums[1..].iter() {
            let diff = num - prev;

            match (diff, prev_change_positive) {
                // Continuing alternation or starting it.
                (1, Some(false)) | (-1, Some(true)) | (1, None) => {
                    current_chain += 1;
                    prev_change_positive = Some(diff == 1);
                }
                // Increased or decreased two times in a row.
                (1, Some(true)) | (-1, Some(false)) => {
                    current_chain = 2;
                    prev_change_positive = Some(diff == 1);
                }
                // Everything else.
                (_, _) => {
                    current_chain = 1;
                    prev_change_positive = None;
                }
            }

            prev = *num;
            if current_chain > longest_chain {
                longest_chain = current_chain
            }
        }

        // No alternating sequences.
        if longest_chain == 1 {
            return -1;
        }

        longest_chain
    }
}
