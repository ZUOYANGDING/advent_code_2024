use std::{
    collections::{hash_map::Entry, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
struct Dataset {
    dict: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
    correct_updates: Vec<Vec<u32>>,
    incorrect_updates: Vec<Vec<u32>>,
    correct_mid_sum_up: u32,
    incorrect_mid_sum_up: u32,
}

enum Type {
    Correct,
    Incorrect,
}

impl Dataset {
    fn load_dataset(filename: PathBuf) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut dict: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut updates: Vec<Vec<u32>> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.contains('|') {
                let nums = line
                    .split('|')
                    .into_iter()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                match dict.entry(nums[0]) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(nums[1]);
                    }
                    Entry::Vacant(_) => {
                        let v = vec![nums[1]];
                        dict.insert(nums[0], v);
                    }
                };
            } else if line.contains(',') {
                let nums = line
                    .split(',')
                    .into_iter()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                updates.push(nums);
            } else {
                continue;
            }
        }

        Self {
            dict,
            updates,
            correct_updates: Vec::new(),
            incorrect_updates: Vec::new(),
            correct_mid_sum_up: 0,
            incorrect_mid_sum_up: 0,
        }
    }

    fn filter_correct_updates(&mut self) {
        for update in &mut self.updates {
            let mut correct = true;
            for i in 0..update.len() - 1 {
                for j in i + 1..update.len() {
                    if !self.dict.get(&update[i]).unwrap().contains(&update[j]) {
                        correct = false;
                        let temp = update[i];
                        update[i] = update[j];
                        update[j] = temp;
                    }
                }
            }
            if correct {
                self.correct_updates.push(update.clone());
            } else {
                self.incorrect_updates.push(update.clone());
            }
        }
    }

    fn cal_mid_sum_up(&mut self, cal_type: Type) {
        let updates = match cal_type {
            Type::Correct => &self.correct_updates,
            Type::Incorrect => &self.incorrect_updates,
        };
        for update in updates {
            let mid_idx = update.len() / 2;
            match cal_type {
                Type::Correct => self.correct_mid_sum_up += update[mid_idx],
                Type::Incorrect => self.incorrect_mid_sum_up += update[mid_idx],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut filename = std::env::current_dir().unwrap();
        filename.push("data.txt");
        let mut data_set = Dataset::load_dataset(filename);
        data_set.filter_correct_updates();
        data_set.cal_mid_sum_up(Type::Correct);
        data_set.cal_mid_sum_up(Type::Incorrect);
        println!("{:?}", data_set.correct_mid_sum_up);
        println!("{:?}", data_set.incorrect_mid_sum_up);
    }
}
