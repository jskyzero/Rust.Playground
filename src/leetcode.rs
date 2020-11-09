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

struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {

    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        let rectangle = SubrectangleQueries {
            rectangle: rectangle.clone()
        };
        return rectangle;
    }
    
    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..row2+1 {
            for j in col1..col2+1 {
                self.rectangle[i as usize][j as usize] = new_value;
                println!("{}", self.rectangle[i as usize][j as usize]);
            }
        }
    }
    
    fn get_value(&self, row: i32, col: i32) -> i32 {
        return self.rectangle[row as usize][col as usize];
    }
}


pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result:Vec<i32> = Vec::new();
    for i in 0..n {
        result.push(nums[i as usize]);
        result.push(nums[(i+n) as usize]);
    }
    return result;
}

pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut result = s.clone().into_bytes();
    for i in 0..indices.len() {
        let c:char = s.chars().nth(i as usize).unwrap();
        result[indices[i as usize] as usize] = c as u8;
    }
    return String::from_utf8(result).expect("Found invalid UTF-8");
}

pub fn number_of_steps (num: i32) -> i32 {
    let mut result = 0;
    let mut n = num;
    while n > 0 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n - 1;
        }
        result = result + 1;
    }
    return result;
}

use std::collections::HashSet;

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut set:HashSet<i32> = HashSet::new();
        let mut map:Vec<i32> = vec![0;101];
        let mut result = 0;

        for i in nums {
            set.insert(i);
            map[i as usize] = map[i as usize] + 1;
        }

        for i in set {
            let n = map[i as usize];
            if n >= 2 {
                result += ((n - 1) + 1) * (n -1) / 2;
            }
        }
        return result;
}

// fn factorial(num: i32) -> i32 {
//     match num {
//         0 => 1,
//         1 => 1,
//         _ => factorial(num - 1) * num,
//     }
// }