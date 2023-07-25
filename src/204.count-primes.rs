// Category: algorithms
// Level: Medium
// Percent: 33.21181%

// Given an integer n, return the number of prime numbers that are strictly less than n.
//
//
// Example 1:
//
// Input: n = 10
// Output: 4
// Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
//
//
// Example 2:
//
// Input: n = 0
// Output: 0
//
//
// Example 3:
//
// Input: n = 1
// Output: 0
//
//
//
// Constraints:
//
//
// 	0 <= n <= 5 * 10⁶
//

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }
        if n == 3 {
            return 1;
        }

        let mut primes = vec![2];

        for number in 3..n {
            if Self::is_prime(number, &primes) {
                primes.push(number);
            }
        }

        primes.len() as i32
    }

    fn is_prime(n: i32, primes_less_than_n: &Vec<i32>) -> bool {
        if n < 2 {
            return false;
        }

        for prime in primes_less_than_n {
            if prime * prime > n {
                break;
            }
            if n % prime == 0 {
                return false;
            }
        }

        true
    }
}
