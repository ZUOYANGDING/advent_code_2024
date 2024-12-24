use std::{collections::VecDeque, fs::read_to_string, path::PathBuf};

#[derive(Debug, Clone)]
struct Solution {
    origin: Vec<i16>,
    converted: Vec<String>,
    compacted: Vec<String>,
    compacted_at_once: Vec<String>,
    check_sum: u64,
    check_sum_at_once: u64,
    file_and_empty_slot: FileAndEmptySlot,
}

#[derive(Debug, Clone)]
struct FileAndEmptySlot {
    empty_queue: VecDeque<(i32, i32)>,
    file_queue: VecDeque<(String, i32, i32)>,
}

impl Solution {
    fn load_dataset(filename: PathBuf) -> Self {
        let content = read_to_string(filename).unwrap();
        let origin: Vec<i16> = content
            .as_bytes()
            .iter()
            .map(|num| char_to_num(*num))
            .collect();

        Self {
            origin,
            converted: Vec::new(),
            compacted: Vec::new(),
            compacted_at_once: Vec::new(),
            check_sum: 0,
            check_sum_at_once: 0,
            file_and_empty_slot: FileAndEmptySlot {
                empty_queue: VecDeque::new(),
                file_queue: VecDeque::new(),
            },
        }
    }

    fn convert(&mut self) {
        let mut file_id = 0;
        let mut file_ptr = 0;
        let mut empty_ptr = 1;
        while file_ptr < self.origin.len() {
            let file_repeats = self.origin[file_ptr];
            for _ in 0..file_repeats {
                self.converted.push(file_id.to_string());
            }
            // in case there is no digit to represent empty space
            if empty_ptr < self.origin.len() {
                let empty_repeats = self.origin[empty_ptr];
                for _ in 0..empty_repeats {
                    self.converted.push(".".to_string());
                }
            }
            file_ptr += 2;
            empty_ptr += 2;
            file_id += 1;
        }
    }

    fn compact(&mut self) {
        let mut head_ptr = 0;
        let mut tail_ptr = self.converted.len() - 1;

        // the last number might not need to be move from back to head
        // like 00998111888277733364465555.66.............
        // after move the 2nd 6 in front of the 1st 6, the origin 1st 6 do not need to be move
        // that is why use head_ptr<=tail_ptr instead of head_ptr<tail_ptr
        while head_ptr <= tail_ptr {
            // move to a empty slot
            while self.converted[head_ptr] != "." && head_ptr <= tail_ptr {
                self.compacted.push(self.converted[head_ptr].clone());
                head_ptr += 1;
            }
            // move to a file slot
            while self.converted[tail_ptr] == "." && head_ptr <= tail_ptr {
                tail_ptr -= 1;
            }
            while self.converted[tail_ptr] != "."
                && self.converted[head_ptr] == "."
                && head_ptr <= tail_ptr
            {
                self.compacted.push(self.converted[tail_ptr].clone());
                tail_ptr -= 1;
                head_ptr += 1;
            }
        }
    }

    fn cal_check_sum(&mut self) {
        let mut idx = 0;
        while idx < self.compacted.len() {
            let number: u64 = self.compacted[idx].parse().unwrap();
            self.check_sum += (idx as u64) * number;
            idx += 1;
        }
    }

    // scan the converted vec from tail to head
    // build empty slot as Queue<(start_idx, length)>
    // build file slot as Queue<(String, length, start_idx)>
    fn scanner_for_file_and_empty_slot(&mut self) {
        let mut idx: i32 = (self.converted.len() as i32) - 1;
        while idx >= 0 {
            if self.converted[idx as usize] == "." {
                let mut start_idx = idx;
                while start_idx >= 0 && self.converted[start_idx as usize] == "." {
                    start_idx -= 1;
                }
                self.file_and_empty_slot
                    .empty_queue
                    .push_front(((start_idx + 1), (idx - start_idx)));
                idx = start_idx;
            } else {
                let mut start_idx = idx;
                let file_id = self.converted[idx as usize].clone();
                while start_idx >= 0
                    && self.converted[start_idx as usize] != "."
                    && self.converted[start_idx as usize] == file_id
                {
                    start_idx -= 1;
                }
                self.file_and_empty_slot.file_queue.push_back((
                    file_id,
                    (idx - start_idx),
                    start_idx + 1,
                ));
                idx = start_idx;
            }
        }
    }

    fn compacted_at_once(&mut self) {
        self.compacted_at_once = self.converted.clone();
        while !self.file_and_empty_slot.file_queue.is_empty()
            && !self.file_and_empty_slot.empty_queue.is_empty()
        {
            // get the candidate file slot
            let file_slot = self.file_and_empty_slot.file_queue.pop_front().unwrap();
            let mut idx_of_empty_slot = 0;
            // find the potential empty slot, especially the empty slot should be on left hand of file slot
            while idx_of_empty_slot < self.file_and_empty_slot.empty_queue.len() {
                if self.file_and_empty_slot.empty_queue[idx_of_empty_slot].1 >= file_slot.1
                    && self.file_and_empty_slot.empty_queue[idx_of_empty_slot].0 < file_slot.2
                {
                    break;
                } else {
                    idx_of_empty_slot += 1;
                }
            }
            // if the potientail empty slot found
            if let Some(empty_slot) = self
                .file_and_empty_slot
                .empty_queue
                .remove(idx_of_empty_slot)
            {
                if file_slot.1 == empty_slot.1 {
                    // file data length match the empty slot exactly
                    // change the "." to file_id
                    let mut start_idx = empty_slot.0 as usize;
                    let mut end_idx = (empty_slot.0 + empty_slot.1) as usize;
                    let file_id = file_slot.0.clone();
                    for idx in start_idx..end_idx {
                        self.compacted_at_once[idx] = file_id.clone();
                    }
                    // change the file_id to "."
                    start_idx = file_slot.2 as usize;
                    end_idx = (file_slot.2 + file_slot.1) as usize;
                    for idx in start_idx..end_idx {
                        self.compacted_at_once[idx] = ".".to_string();
                    }
                } else {
                    // file data length less than empty slot length
                    // change the "." to file_id
                    let mut start_idx = empty_slot.0 as usize;
                    let mut end_idx = (empty_slot.0 + file_slot.1) as usize;
                    let file_id = file_slot.0.clone();
                    for idx in start_idx..end_idx {
                        self.compacted_at_once[idx] = file_id.clone();
                    }
                    // change the file_id to "."
                    start_idx = file_slot.2 as usize;
                    end_idx = (file_slot.2 + file_slot.1) as usize;
                    for idx in start_idx..end_idx {
                        self.compacted_at_once[idx] = ".".to_string();
                    }
                    // push the rest empty space back to empty_slot queue
                    self.file_and_empty_slot.empty_queue.insert(
                        idx_of_empty_slot,
                        (empty_slot.0 + file_slot.1, empty_slot.1 - file_slot.1),
                    );
                }
            }
            // if potential empty slot cannot be found
            // just drop the candidate file slot
        }
    }

    fn cal_check_sum_at_once(&mut self) {
        let mut idx: u64 = 0;
        while (idx as usize) < self.compacted_at_once.len() {
            if self.compacted_at_once[idx as usize] != "." {
                self.check_sum_at_once +=
                    idx * self.compacted_at_once[idx as usize].parse::<u64>().unwrap();
            }
            idx += 1;
        }
    }
}

fn main() {
    let mut filename = std::env::current_dir().unwrap();
    filename.push("day9/data.txt");
    let mut solution = Solution::load_dataset(filename);
    solution.convert();
    solution.compact();
    solution.cal_check_sum();
    println!("{:?}", solution.check_sum); //6519155389266
    solution.scanner_for_file_and_empty_slot();
    // println!("{:?}", solution.converted);
    // println!("{:?}", solution.file_and_empty_slot);
    solution.compacted_at_once();
    // println!("{:?}", solution.compacted_at_once);
    solution.cal_check_sum_at_once();
    println!("{:?}", solution.check_sum_at_once); //6547228115826
}

fn char_to_num<T>(c: u8) -> T
where
    T: From<u8>,
{
    T::from((c as u8) - b'0')
}
