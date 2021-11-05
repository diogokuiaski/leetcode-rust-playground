/*

https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/

Given two integers n and k, return the kth lexicographically smallest integer in the range [1, n].

Example 1:
Input: n = 13, k = 2
Output: 10
Explanation: The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], so the second smallest number is 10.

Example 2:
Input: n = 1, k = 1
Output: 1

Constraints:
1 <= k <= n <= 109

*/
use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let v = Self::l(&n, 9);
        //println!("{} {} :: {:?}",n,k,v);
        if n < 10 {
            return k;
        }
        let mut s = 0;
        for i in 0..v.len() {
            if s + v[i] >= k {
                return match Self::f(v[i] - 1, k - s - 1) {
                    None => i as i32 + 1,
                    Some(x) => {
                        let mut x = x;
                        x.insert(0, i as i32 + 1);
                        x.iter().fold(0, |acc, x| acc * 10 + x)
                    }
                };
            }
            s += v[i];
        }
        s
    }

    pub fn f(n: i32, k: i32) -> Option<Vec<i32>> {
        let v = Self::l(&n, 10);
        //println!("{} {} :: {:?}",n,k,v);
        if k == 0 {
            return None;
        }
        if n <= 10 {
            return Some(vec![k - 1]);
        }
        let mut s = 0;
        for i in 0..v.len() {
            if s + v[i] >= k {
                return match Self::f(v[i] - 1, k - s - 1) {
                    None => Some(vec![i as i32]),
                    Some(x) => {
                        let mut x = x;
                        x.insert(0, i as i32);
                        Some(x)
                    }
                };
            }
            s += v[i];
        }
        None
    }

    pub fn find_kth_number_sorted(n: i32, k: i32) -> i32 {
        let k = k as usize;
        let range = 1..n + 1;
        let range: Vec<i32> = range.collect();
        let mut range = range.iter().map(|v| Self::v(v)).collect::<Vec<Vec<i32>>>();
        range.sort_by(|a, b| {
            let mut la = 0;
            let mut lb = 0;

            loop {
                let result = a[la].cmp(&b[lb]);

                if result != Ordering::Equal {
                    return result;
                }

                la += 1;
                lb += 1;

                if la == a.len() && lb == b.len() {
                    return Ordering::Equal;
                } else if la == a.len() {
                    return Ordering::Less;
                } else if lb == b.len() {
                    return Ordering::Greater;
                }
            }
        });
        //println!("{} - {:?}",k-1, &range);
        let range = range.get(k - 1).unwrap();
        range.iter().fold(0, |acc, x| acc * 10 + x)
    }

    fn v(n: &i32) -> Vec<i32> {
        let mut n = n.clone();
        if n == 0 {
            return vec![0];
        }
        let mut a = vec![];
        while n > 0 {
            a.insert(0, n % 10);
            n = n / 10;
        }
        a
    }

    fn l(n: &i32, le: usize) -> Vec<i32> {
        let mut v = vec![0; le];
        let mut l = 1;
        let mut n = n.clone();
        while n > 0 {
            for i in 1..le + 1 {
                let j = (i - 1) as usize;
                if n < l {
                    v[j] = v[j] + n;
                    n = 0;
                    break;
                }
                v[j] = v[j] + l;
                n -= l;
            }
            l *= 10;
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use crate::kth_number::Solution;

    #[test]
    fn t() {
        assert_eq!(Solution::find_kth_number(3, 1), 1);
        assert_eq!(Solution::find_kth_number(10, 3), 2);
        assert_eq!(Solution::find_kth_number(11, 1), 1);
        assert_eq!(Solution::find_kth_number(11, 2), 10);
        assert_eq!(Solution::find_kth_number(11, 3), 11);
        assert_eq!(Solution::find_kth_number(13, 2), 10);
        assert_eq!(Solution::find_kth_number(1, 1), 1);
        assert_eq!(Solution::find_kth_number(20, 12), 2);
        assert_eq!(Solution::find_kth_number(20, 13), 20);
        assert_eq!(Solution::find_kth_number(100, 10), 17);
        assert_eq!(Solution::find_kth_number(120, 3), 100);
        assert_eq!(Solution::find_kth_number(120, 4), 101);
        assert_eq!(Solution::find_kth_number(120, 13), 11);
        assert_eq!(Solution::find_kth_number(120, 14), 110);
        assert_eq!(Solution::find_kth_number(120, 22), 118);
        assert_eq!(Solution::find_kth_number(10000, 10000), 9999);
        assert_eq!(Solution::find_kth_number(10000, 1), 1);
    }
}
