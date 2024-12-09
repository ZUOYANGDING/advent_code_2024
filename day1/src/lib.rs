use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{anyhow, Result};
struct List {
    left: Vec<u32>,
    right: Vec<u32>,
}
impl List {
    fn init(file_path: PathBuf) -> Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut left = Vec::new();
        let mut right = Vec::new();
        for line in reader.lines() {
            let line_str = line?;
            let parts: Vec<&str> = line_str.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(num1), Ok(num2)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    left.push(num1);
                    right.push(num2);
                } else {
                    return Err(anyhow!("Read data error"));
                }
            } else {
                return Err(anyhow!("Read data error"));
            }
        }
        Ok(Self { left, right })
    }

    fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }

    fn cal_distance(&self) -> u32 {
        let mut ret = 0;
        for (num1, num2) in self.left.iter().zip(self.right.iter()) {
            match num1.cmp(num2) {
                std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => ret += num1 - num2,
                std::cmp::Ordering::Less => ret += num2 - num1,
            }
        }
        ret
    }

    fn cal_similarity(&self) -> u32 {
        let mut map_of_right = HashMap::new();
        self.right.iter().for_each(|num| {
            *map_of_right.entry(num).or_insert(0) += 1;
        });
        let mut ret = 0;
        self.left.iter().for_each(|num| {
            ret += num * map_of_right.get(num).unwrap_or(&0);
        });
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
        let mut list = List::init(file_path).unwrap();
        list.sort();
        println!("{:?}", list.cal_distance());
        println!("{:?}", list.cal_similarity());
    }
}
