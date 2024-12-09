use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct DataSet {
    data_frame: Vec<Vec<i32>>,
    num_of_safe: u32,
}

impl DataSet {
    fn init(file_path: PathBuf) -> Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut data_frame = Vec::new();
        let mut num_of_safe = 0;

        for line in reader.lines() {
            let line_str = line?;
            let nums: Vec<i32> = line_str
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            if DataSet::is_safe(&nums) {
                num_of_safe += 1;
            }
            data_frame.push(nums);
        }

        Ok(Self {
            data_frame,
            num_of_safe,
        })
    }

    fn is_safe(nums: &Vec<i32>) -> bool {
        if nums.len() < 2 {
            return true;
        }
        let mut idx = 0;
        let is_increase = nums[idx + 1] > nums[idx];
        if nums[idx].abs_diff(nums[idx + 1]) < 1 || nums[idx].abs_diff(nums[idx + 1]) > 3 {
            return false;
        }
        idx += 1;
        while idx < nums.len() - 1 {
            if nums[idx].abs_diff(nums[idx + 1]) < 1
                || nums[idx].abs_diff(nums[idx + 1]) > 3
                || is_increase != (nums[idx + 1] > nums[idx])
            {
                return false;
            }
            idx += 1;
        }
        return true;
    }

    fn tolerate_a_single_bad_level(&self) -> u32 {
        let mut ret = 0;
        for line in &self.data_frame {
            if DataSet::is_safe(line) {
                ret += 1;
            } else {
                for idx in 0..line.len() {
                    let mut clone_line = line.clone();
                    clone_line.remove(idx);
                    if DataSet::is_safe(&clone_line) {
                        ret += 1;
                        break;
                    }
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut file_path = std::env::current_dir().unwrap();
        file_path.push("data.txt");
        let data_set = DataSet::init(file_path).unwrap();
        println!("{:?}", data_set);

        println!("{:?}", data_set.tolerate_a_single_bad_level());
    }
}
