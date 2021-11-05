/*

https://leetcode.com/problems/self-crossing/

You are given an array of integers distance.

You start at point (0,0) on an X-Y plane and you move distance[0] meters to the north, then distance[1] meters to the west,
distance[2] meters to the south, distance[3] meters to the east, and so on. In other words, after each move,
your direction changes counter-clockwise.

Return true if your path crosses itself, and false if it does not.

Example 1:
Input: distance = [2,1,1,2]
Output: true

Example 2:
Input: distance = [1,2,3,4]
Output: false

Example 3:
Input: distance = [1,1,1,1]
Output: true

Constraints:

1 <= distance.length <= 105
1 <= distance[i] <= 105
*/

#[derive(PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

pub type Position = (i32,i32);

impl Direction {
    pub fn new(current_step: i32) -> Direction {
        match current_step % 4 {
            0 => Direction::North,
            1 => Direction::West,
            2 => Direction::South,
            3 => Direction::East,
            _ => unreachable!(),
        }
    }
    pub fn next_position(&self, current_position: Position, distance: i32) -> Position {
        match self {
            Direction::North => (current_position.0, current_position.1 + distance),
            Direction::West => (current_position.0 - distance, current_position.1),
            Direction::South => (current_position.0, current_position.1 - distance),
            Direction::East => (current_position.0 + distance, current_position.1),
        }
    }
}

struct Line {
    p1: Position,
    p2: Position,
}

pub struct Solution;
impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        if distance.len() < 4 {
            return false;
        }

        let direction = Direction::new(0);
        let p2 = direction.next_position((0,0), distance[0]);
        let l1 = Line {
            p1:(0, 0),
            p2,
        };
        let mut lines: Vec<Line> = vec![l1];

        for i in 1..distance.len() {
            let last = lines.last().unwrap().p2;
            let direction = Direction::new(i as i32);

            let pv1 = direction.next_position(last, distance[i]);
            lines.push(Line { p1: last, p2: pv1 });
        }
        for i in 0..lines.len() {
            for j in i+3..lines.len() {
                let li = &lines[i];
                let lj = &lines[j];

                if Solution::check_intersection(li,lj) {
                    return true;
                }
            }
        }
        false
    }

    fn check_intersection(
        l1: &Line,
        l2: &Line,
    ) -> bool {
        let is_l1_horizontal = l1.p1.0 != l1.p2.0;
        let is_l2_horizontal = l2.p1.0 != l2.p2.0;
        match (is_l1_horizontal, is_l2_horizontal) {
            (true, true) => {
                let x_min_l1 = std::cmp::min(l1.p1.0, l1.p2.0);
                let x_max_l1 = std::cmp::max(l1.p1.0, l1.p2.0);
                (l1.p1.1 == l2.p1.1) && ( (l2.p1.0 >= x_min_l1 && l2.p1.0 <= x_max_l1) || (l2.p2.0 >= x_min_l1 && l2.p2.0 <= x_max_l1) )
            },
            (true, false) => {
                //l1 horizontal = same-y / l2 vertical = same-x
                let x_min = std::cmp::min(l1.p1.0, l1.p2.0);
                let x_max = std::cmp::max(l1.p1.0, l1.p2.0);
                let y_min = std::cmp::min(l2.p1.1, l2.p2.1);
                let y_max = std::cmp::max(l2.p1.1, l2.p2.1);
                (l2.p1.0 >= x_min && l2.p1.0 <= x_max) && (l1.p1.1 >= y_min && l1.p1.1 <= y_max)
            },
            (false, true) => {
                // l1 vertical = same-x / l2 horizontal = same-y
                let x_min = std::cmp::min(l2.p1.0, l2.p2.0);
                let x_max = std::cmp::max(l2.p1.0, l2.p2.0);
                let y_min = std::cmp::min(l1.p1.1, l1.p2.1);
                let y_max = std::cmp::max(l1.p1.1, l1.p2.1);
                (l1.p1.0 >= x_min && l1.p1.0 <= x_max) && (l2.p1.1 >= y_min && l2.p1.1 <= y_max)
            },
            (false, false) => {
                let y_min_l1 = std::cmp::min(l1.p1.1, l1.p2.1);
                let y_max_l1 = std::cmp::max(l1.p1.1, l1.p2.1);
                (l1.p1.0 == l2.p1.0) && ( (l2.p1.1 >= y_min_l1 && l2.p1.1 <= y_max_l1) || (l2.p2.1 >= y_min_l1 && l2.p2.1 <= y_max_l1) )
            },
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::self_crossing::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_self_crossing(vec![2, 1, 1, 2]), true);
        assert_eq!(Solution::is_self_crossing(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::is_self_crossing(vec![1, 1, 1, 1]), true);
        assert_eq!(Solution::is_self_crossing(vec![2, 1, 1, 1]), true);

        assert_eq!(Solution::is_self_crossing(vec![1, 1, 2, 1]), false);
        assert_eq!(Solution::is_self_crossing(vec![1, 2, 1, 1]), false);

        assert_eq!(Solution::is_self_crossing(vec![1]), false);

        assert_eq!(Solution::is_self_crossing(vec![1, 2, 3, 4, 5]), false);
        assert_eq!(Solution::is_self_crossing(vec![1, 2, 1, 1, 3]), true);

        assert_eq!(Solution::is_self_crossing(vec![1, 1, 2, 1, 1]), true);
    }
}
