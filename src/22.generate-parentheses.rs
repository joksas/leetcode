// Category: algorithms
// Level: Medium
// Percent: 73.02078%

// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
//
//
// Example 1:
// Input: n = 3
// Output: ["((()))","(()())","(())()","()(())","()()()"]
// Example 2:
// Input: n = 1
// Output: ["()"]
//
//
// Constraints:
//
//
// 	1 <= n <= 8
//
//
impl Solution {
    fn generate(
        combinations: &mut Vec<String>,
        current_combination: &mut String,
        num_open_left: i32,
        num_close_left: i32,
    ) {
        if num_open_left == 0 && num_close_left == 0 {
            combinations.push(current_combination.clone());
            return;
        }

        if num_open_left > 0 {
            current_combination.push('(');
            Self::generate(
                combinations,
                current_combination,
                num_open_left - 1,
                num_close_left,
            );
            current_combination.pop();
        }

        if num_close_left > num_open_left {
            current_combination.push(')');
            Self::generate(
                combinations,
                current_combination,
                num_open_left,
                num_close_left - 1,
            );
            current_combination.pop();
        }
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut current_combination = String::new();
        let mut combinations = Vec::new();
        Self::generate(&mut combinations, &mut current_combination, n, n);
        combinations
    }
}
