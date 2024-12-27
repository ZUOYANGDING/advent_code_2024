use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
};

struct StoneVec {
    origin: Vec<String>,
}

impl StoneVec {
    fn load_file(filepath: PathBuf) -> Self {
        let stones = fs::read_to_string(filepath).unwrap();
        let stone_vec = stones
            .split_whitespace()
            .into_iter()
            .map(|num| num.to_string())
            .collect();
        StoneVec { origin: stone_vec }
    }

    fn blink(&self, blink_time: u16) -> usize {
        let mut ret = 0;
        for num in &self.origin {
            let mut final_vec: Vec<String> = Vec::new();
            Self::dfs(num.to_owned(), 0, blink_time, &mut final_vec);
            ret += final_vec.len();
            // println!("{:?} finished", num);
        }

        ret
    }

    fn blink_alter(&self, blink_time: u16) -> u64 {
        let mut count = 0;
        let mut memory_dict: HashMap<(String, u16), u64> = HashMap::new();
        for num in &self.origin {
            count += Self::dfs_with_memory(num.to_owned(), 0, blink_time, &mut memory_dict);
        }
        count
    }

    fn dfs(num: String, depth: u16, blink_time: u16, final_vec: &mut Vec<String>) {
        if depth == blink_time {
            final_vec.push(num);
            return;
        }
        let num_len = num.len();
        if num_len % 2 == 0 {
            let (left_num, right_num) = seperate_number(&num, num_len as i32);
            Self::dfs(left_num, depth + 1, blink_time, final_vec);
            Self::dfs(right_num, depth + 1, blink_time, final_vec);
        } else if num_len == 1 && num.parse::<i32>().unwrap() == 0 {
            Self::dfs(String::from("1"), depth + 1, blink_time, final_vec);
        } else {
            let mut num: u64 = num.parse::<u64>().unwrap();
            num *= 2024;
            Self::dfs(num.to_string(), depth + 1, blink_time, final_vec);
        }
    }

    /// Use a HashMap<(number, depth), length> to memorize how many numbers will be created at this depth and deeper
    /// To avoid repeated calucation of same number at same level
    fn dfs_with_memory(
        num: String,
        depth: u16,
        blink_time: u16,
        memory_dict: &mut HashMap<(String, u16), u64>,
    ) -> u64 {
        // if the dict has record of the num at the same level
        if memory_dict.contains_key(&(num.clone(), depth)) {
            return *memory_dict.get(&(num.clone(), depth)).unwrap();
        }
        if depth == blink_time {
            return 1;
        }
        let num_len = num.len();
        let ret = if num_len % 2 == 0 {
            let (left_num, right_num) = seperate_number(&num, num_len as i32);
            let left_result = Self::dfs_with_memory(left_num, depth + 1, blink_time, memory_dict);
            let right_result = Self::dfs_with_memory(right_num, depth + 1, blink_time, memory_dict);
            left_result + right_result
        } else if num_len == 1 && num.parse::<i32>().unwrap() == 0 {
            Self::dfs_with_memory(String::from("1"), depth + 1, blink_time, memory_dict)
        } else {
            let mut num: u64 = num.parse::<u64>().unwrap();
            num *= 2024;
            Self::dfs_with_memory(num.to_string(), depth + 1, blink_time, memory_dict)
        };
        memory_dict.insert((num.clone(), depth), ret);
        ret
    }
}

fn seperate_number(num: &String, num_len: i32) -> (String, String) {
    let half = (num_len as usize) / 2;
    let left_num = &num[..half].parse::<u64>().unwrap();
    let mut right_num = num[half..(num_len as usize)].as_bytes();
    let mut idx = 0;
    while idx < right_num.len() && right_num[idx] - b'0' == 0 {
        idx += 1;
    }
    if idx >= right_num.len() {
        right_num = &[b'0'];
    } else {
        right_num = &right_num[idx..right_num.len()];
    }
    (
        left_num.to_string(),
        String::from_utf8(right_num.to_vec()).unwrap(),
    )
}

fn main() {
    let mut filename = std::env::current_dir().unwrap();
    filename.push("day11/data.txt");
    let stone_vec = StoneVec::load_file(filename);
    println!("{:?}", stone_vec.blink(25)); //172484
    println!("{:?}", stone_vec.blink_alter(75))
}
