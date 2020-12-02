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

    for candy in candies.iter() {
        if *candy > max_candy {
            max_candy = *candy;
        }
    }

    for candy in candies.iter() {
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
            rectangle: rectangle.clone(),
        };
        return rectangle;
    }
    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..row2 + 1 {
            for j in col1..col2 + 1 {
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
    let mut result: Vec<i32> = Vec::new();
    for i in 0..n {
        result.push(nums[i as usize]);
        result.push(nums[(i + n) as usize]);
    }
    return result;
}

pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut result = s.clone().into_bytes();
    for i in 0..indices.len() {
        let c: char = s.chars().nth(i as usize).unwrap();
        result[indices[i as usize] as usize] = c as u8;
    }
    return String::from_utf8(result).expect("Found invalid UTF-8");
}

pub fn number_of_steps(num: i32) -> i32 {
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
    let mut set: HashSet<i32> = HashSet::new();
    let mut map: Vec<i32> = vec![0; 101];
    let mut result = 0;

    for i in nums {
        set.insert(i);
        map[i as usize] = map[i as usize] + 1;
    }

    for i in set {
        let n = map[i as usize];
        if n >= 2 {
            result += ((n - 1) + 1) * (n - 1) / 2;
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

// use std::iter::Iterator;
// let x_index = x.iter().find(|&&x| x == point_x);

pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut set: HashSet<i32> = HashSet::new();
    let mut sorted_set: Vec<i32> = Vec::new();
    let mut result: i32 = 0;

    for point in points {
        set.insert(point[0]);
    }
    sorted_set = set.into_iter().collect();
    sorted_set.sort();

    for i in 0..sorted_set.len() - 1 {
        if (sorted_set[i + 1] - sorted_set[i]) > result {
            result = sorted_set[i + 1] - sorted_set[i];
        }
    }

    return result;
}

use std::collections::HashMap;

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut sorted_nums = nums.clone();
    sorted_nums.sort();

    let mut last_num = sorted_nums[0];
    let mut num_size = 0;
    map.insert(last_num, num_size);

    for i in sorted_nums {
        if (i != last_num) {
            map.insert(i, num_size);
            last_num = i;
        }
        num_size = num_size + 1;
    }

    for i in nums {
        result.push(*map.get(&i).unwrap());
    }

    return result;
}

pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut digit: Vec<i32> = Vec::new();
    let mut num: i32 = n;

    while num >= 10 {
        digit.push(num % 10);
        num = num / 10;
    }
    digit.push(num);

    //     for i in &digit {
    //         print!("{} ", *i);
    //     }

    let sum: i32 = digit.iter().sum();
    let product: i32 = digit.iter().product();

    return product - sum;
}

pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for i in 0..nums.len() / 2 {
        for j in 0..nums[2 * i] {
            result.push(nums[2 * i + 1]);
        }
    }

    return result;
}

struct ParkingSystem {
    plots: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        return ParkingSystem {
            plots: vec![big, medium, small],
        };
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            _ => {
                let size = self.plots[car_type as usize - 1];
                if size >= 1 {
                    self.plots[car_type as usize - 1] = size - 1;
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut result: i32 = 0;
    let mut p = head;
    while p.clone().unwrap().next != None {
        result = (result << 1) + p.clone().unwrap().val;
        p = p.clone().unwrap().next;
        // print!("{}-{} ",p.clone().unwrap().val, result);
    }
    result = (result << 1) + p.unwrap().val;

    return result;
}

pub fn balanced_string_split(s: String) -> i32 {
    let mut result = 0;
    let mut l = 0;
    let mut r = 0;
    let mut list: Vec<char> = s.clone().chars().collect();

    for i in list {
        match i {
            'L' => {
                l = l + 1;
                if l == r {
                    l = 0;
                    r = 0;
                    result = result + 1;
                }
            }
            'R' => {
                r = r + 1;
                if l == r {
                    l = 0;
                    r = 0;
                    result = result + 1;
                }
            }
            _ => {}
        }
    }

    return result;
}


pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut result = 0;

    for i in 0..n {
        result = result ^ (start + 2 * i);
    }

    return  result ^ 0;
}

pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for i in 0..nums.len() {
        result.insert(index[i as usize] as usize, nums[i as usize]);
    }
    return result;
}

pub fn max_depth(s: String) -> i32 {
    let mut depth = 0;
    let mut result = 0;
    let mut list:Vec<char> = s.chars().collect();

    for c in list {
        match c {
            '(' => {
                depth = depth + 1;
                if depth > result {
                    result = depth;
                }
            }
            ')' => {
                depth = depth - 1;
            }
            _ => {}
        }
    }

    return result;
}


pub fn num_teams(rating: Vec<i32>) -> i32 {
    let mut result:i32 = 0;

    for i in 0..rating.len() {
        for j in i..rating.len() {
            for z in j..rating.len() {
                if rating[z] > rating[j] && rating[j] > rating[i] {
                    result= result + 1;
                }
                if rating[z] < rating[j] && rating[j] < rating[i] {
                    result= result + 1;
                } 
            }
        }
    }

    return result;
}

pub fn main336() {
    let mut counter = if false {1} else {0};

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {}", result);
}
