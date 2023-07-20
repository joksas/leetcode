// Category: algorithms
// Level: Medium
// Percent: 57.64887%

// An n-bit gray code sequence is a sequence of 2n integers where:
//
//
// 	Every integer is in the inclusive range [0, 2n - 1],
// 	The first integer is 0,
// 	An integer appears no more than once in the sequence,
// 	The binary representation of every pair of adjacent integers differs by exactly one bit, and
// 	The binary representation of the first and last integers differs by exactly one bit.
//
//
// Given an integer n, return any valid n-bit gray code sequence.
//
//
// Example 1:
//
// Input: n = 2
// Output: [0,1,3,2]
// Explanation:
// The binary representation of [0,1,3,2] is [00,01,11,10].
// - 00 and 01 differ by one bit
// - 01 and 11 differ by one bit
// - 11 and 10 differ by one bit
// - 10 and 00 differ by one bit
// [0,2,3,1] is also a valid gray code sequence, whose binary representation is [00,10,11,01].
// - 00 and 10 differ by one bit
// - 10 and 11 differ by one bit
// - 11 and 01 differ by one bit
// - 01 and 00 differ by one bit
//
//
// Example 2:
//
// Input: n = 1
// Output: [0,1]
//
//
//
// Constraints:
//
//
// 	1 <= n <= 16
//

impl Solution {
    // BRGC algorithm.
    pub fn gray_code(n: i32) -> Vec<i32> {
        let num_states = 2_i32.pow(n as u32);

        // `num_states x n` vector.
        let mut binary_states = vec![vec![false; n as usize]; num_states as usize];

        for digit_idx in 0..n {
            let offset = 2_i32.pow(digit_idx as u32);
            let alternation_width = if digit_idx == n - 1 {
                offset
            } else {
                2 * offset
            };

            let mut counter = 0;
            let mut on = true;
            for state_idx in 0..num_states {
                if counter == alternation_width {
                    on = !on;
                    counter = 0;
                }
                let idx = (state_idx + offset) % num_states;
                binary_states[idx as usize][digit_idx as usize] = on;
                counter += 1;
            }
        }

        let mut decimal_numbers = vec![];

        for state_idx in 0..num_states {
            let mut decimal_number = 0;
            // Order of iteration in constructing the numbers doesn't matter.
            for digit_idx in 0..n {
                let one = binary_states[state_idx as usize][digit_idx as usize];
                if !one {
                    continue;
                }

                decimal_number += 2_i32.pow(digit_idx as u32);
            }
            decimal_numbers.push(decimal_number);
        }

        decimal_numbers
    }
}
