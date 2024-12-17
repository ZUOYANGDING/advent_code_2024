use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::Deref,
    path::PathBuf,
};
struct DataSet {
    data: Vec<Vec<char>>,

    lines: usize,
    columns: usize,
}
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const SAMX: [char; 4] = ['S', 'A', 'M', 'X'];
impl DataSet {
    fn load_dataset(filename: PathBuf) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut data: Vec<Vec<char>> = vec![];
        let mut lines = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            data.push(line.chars().collect());
            lines += 1;
        }

        DataSet {
            columns: data[0].len(),
            data,
            lines,
        }
    }

    fn bruteforce_count_XMAS(&self) -> u32 {
        let mut ret = 0;
        // for each start position
        for idx_i in 0..self.lines {
            for idx_j in 0..self.columns {
                if self.data[idx_i][idx_j] != 'X' {
                    continue;
                }
                // do up check
                if self.up_check(idx_i, idx_j) {
                    ret += 1;
                }
                // do down check
                if self.down_check(idx_i, idx_j) {
                    ret += 1;
                }
                // do left check
                if self.left_check(idx_i, idx_j) {
                    ret += 1;
                }
                // do right check
                if self.right_check(idx_i, idx_j) {
                    ret += 1;
                }
                // do diagonal check
                ret += self.diagonal_check(idx_i, idx_j)
            }
        }
        ret
    }

    fn bruteforce_count_cross_MAS(&self) -> u32 {
        let mut ret = 0;
        for idx_i in 1..self.lines - 1 {
            for idx_j in 1..self.columns - 1 {
                if self.data[idx_i][idx_j] != 'A' {
                    continue;
                }
                // check M on top left and right
                // check M on right
                // check M on bottom left and right
                // check M on left
                if self.check_M_on_top_left_right(idx_i, idx_j)
                    || self.check_M_on_right(idx_i, idx_j)
                    || self.check_M_on_bottom_left_right(idx_i, idx_j)
                    || self.check_M_on_left(idx_i, idx_j)
                {
                    ret += 1;
                }
            }
        }
        ret
    }

    fn check_M_on_top_left_right(&self, r: usize, c: usize) -> bool {
        self.data[r - 1][c - 1] == 'M'
            && self.data[r - 1][c + 1] == 'M'
            && self.data[r + 1][c - 1] == 'S'
            && self.data[r + 1][c + 1] == 'S'
    }
    fn check_M_on_right(&self, r: usize, c: usize) -> bool {
        self.data[r - 1][c + 1] == 'M'
            && self.data[r + 1][c + 1] == 'M'
            && self.data[r - 1][c - 1] == 'S'
            && self.data[r + 1][c - 1] == 'S'
    }

    fn check_M_on_bottom_left_right(&self, r: usize, c: usize) -> bool {
        self.data[r + 1][c - 1] == 'M'
            && self.data[r + 1][c + 1] == 'M'
            && self.data[r - 1][c - 1] == 'S'
            && self.data[r - 1][c + 1] == 'S'
    }

    fn check_M_on_left(&self, r: usize, c: usize) -> bool {
        self.data[r - 1][c - 1] == 'M'
            && self.data[r + 1][c - 1] == 'M'
            && self.data[r - 1][c + 1] == 'S'
            && self.data[r + 1][c + 1] == 'S'
    }

    fn up_check(&self, r: usize, c: usize) -> bool {
        if r >= 3 {
            let slice: Vec<char> = self.data[r - 3..=r]
                .iter()
                .filter_map(|str| str.get(c))
                .cloned()
                .collect();
            slice == SAMX
        } else {
            false
        }
    }

    fn down_check(&self, r: usize, c: usize) -> bool {
        if r < self.lines - 3 {
            let slice: Vec<char> = self.data[r..=r + 3]
                .iter()
                .filter_map(|str| str.get(c))
                .cloned()
                .collect();
            slice == XMAS
        } else {
            false
        }
    }

    fn left_check(&self, r: usize, c: usize) -> bool {
        if c >= 3 {
            let slice = self
                .data
                .get(r)
                .map(|line| line.get(c - 3..=c).map(|slice| slice.to_vec()))
                .flatten()
                .unwrap();
            slice == SAMX
        } else {
            false
        }
    }

    fn right_check(&self, r: usize, c: usize) -> bool {
        if c < self.columns - 3 {
            let slice = self
                .data
                .get(r)
                .map(|line| line.get(c..=c + 3).map(|slice| slice.to_vec()))
                .flatten()
                .unwrap();
            slice == XMAS
        } else {
            false
        }
    }

    fn diagonal_check(&self, r: usize, c: usize) -> u32 {
        let mut ret = 0;
        // left up
        if c >= 3 && r >= 3 {
            let mut slice: Vec<char> = vec![];
            for step in 0..=3 {
                slice.push(self.data[r - step][c - step]);
            }
            if slice == XMAS {
                ret += 1;
            }
        }
        // left down
        if c >= 3 && r < self.lines - 3 {
            let mut slice: Vec<char> = vec![];
            for step in 0..=3 {
                slice.push(self.data[r + step][c - step]);
            }
            if slice == XMAS {
                ret += 1;
            }
        }
        // right up
        if c < self.columns - 3 && r >= 3 {
            let mut slice: Vec<char> = vec![];
            for step in 0..=3 {
                slice.push(self.data[r - step][c + step]);
            }
            if slice == XMAS {
                ret += 1;
            }
        }
        // right down
        if c < self.columns - 3 && r < self.lines - 3 {
            let mut slice: Vec<char> = vec![];
            for step in 0..=3 {
                slice.push(self.data[r + step][c + step]);
            }
            if slice == XMAS {
                ret += 1;
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
        let mut filename = std::env::current_dir().unwrap();
        filename.push("data.txt");
        let dataset = DataSet::load_dataset(filename);
        println!("{:?}", dataset.bruteforce_count_XMAS()); // should be 2434
        println!("{:?}", dataset.bruteforce_count_cross_MAS());
    }
}
