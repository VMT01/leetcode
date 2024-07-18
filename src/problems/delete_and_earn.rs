use super::Solution;

#[allow(dead_code)]
impl Solution {
    // TODO:
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        use std::{cmp::max, collections::HashMap};

        let mut nums_map: HashMap<i32, i32> = HashMap::new();

        nums.into_iter().for_each(|num| {
            nums_map.entry(num).and_modify(|v| *v += 1).or_insert(1);
        });

        let (mut curr_max, mut prev_max) = (0, 0);

        let mut keys = nums_map.keys().collect::<Vec<&i32>>();
        keys.sort_unstable();
        keys.into_iter().for_each(|key| {
            let temp = curr_max;
            curr_max = max(
                prev_max + key * nums_map.get(key).unwrap(),
                temp + (key + 1) * nums_map.get(&(key + 1)).unwrap_or(&0),
            );
            prev_max = temp;
        });

        curr_max
    }
}

#[test]
fn test_delete_and_earn_1() {
    assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
}

#[test]
fn test_delete_and_earn_2() {
    assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
}
