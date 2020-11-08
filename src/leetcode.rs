// Input: nums = [1,2,3,4]
// Output: [1,3,6,10]
// Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut next: i32 = 0;
    let mut result: Vec<i32> = Vec::new();
    for i in nums {
        next = next + i;
        result.push(next);
    }
    return result;
}

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut max_candy: i32 = 0;
    let mut result: Vec<bool> = Vec::new();

    for candy in &candies {
        if *candy > max_candy {
            max_candy = *candy;
        }
    }

    for candy in &candies {
        let new_candy = *candy + extra_candies;
        if new_candy >= max_candy {
            result.push(true);
        } else {
            result.push(false);
        }
    }

    return result;
}
