use std::{
    clone,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct LabMap {
    map: Vec<Vec<char>>,
    visited: HashSet<(usize, usize)>,
    start_point: (usize, usize),
    direction: Direction,
    height: usize,
    width: usize,
}

impl LabMap {
    fn load_dataset(filename: PathBuf) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut map = Vec::new();
        let mut start_row = 0;
        let mut start_col = 0;

        for (idx, line) in reader.lines().enumerate() {
            let line: Vec<char> = line.unwrap().chars().collect();
            if let Some(col) = line.iter().position(|&c| c == '^') {
                //hard code here for my dataset is start as up
                start_row = idx;
                start_col = col;
            }
            map.push(line);
        }
        Self {
            height: map.len(),
            width: map[0].len(),
            map,
            visited: HashSet::new(),
            start_point: (start_row, start_col),
            direction: Direction::Up,
        }
    }

    fn count_unique_position(&mut self) -> u32 {
        let mut row = self.start_point.0 as i32;
        let mut col = self.start_point.1 as i32;
        let mut ret = 0;
        while row >= 0 && (row as usize) < self.height && col >= 0 && (col as usize) < self.width {
            match self.direction {
                Direction::Up => {
                    // go up until touch the map edge or match a '#'
                    while row >= 0 && self.map[row as usize][col as usize] != '#' {
                        if self.visited.insert((row as usize, col as usize)) {
                            ret += 1;
                        }
                        row -= 1;
                    }
                    if row < 0 {
                        break;
                    } else {
                        // turn right
                        self.direction = Direction::Right;
                        row += 1;
                    }
                }
                Direction::Down => {
                    while (row as usize) < self.height
                        && self.map[row as usize][col as usize] != '#'
                    {
                        if self.visited.insert((row as usize, col as usize)) {
                            ret += 1;
                        }
                        row += 1;
                    }
                    if (row as usize) >= self.height {
                        break;
                    } else {
                        // turn right
                        self.direction = Direction::Left;
                        row -= 1;
                    }
                }
                Direction::Left => {
                    while col >= 0 && self.map[row as usize][col as usize] != '#' {
                        if self.visited.insert((row as usize, col as usize)) {
                            ret += 1;
                        }
                        col -= 1;
                    }
                    if col < 0 {
                        break;
                    } else {
                        self.direction = Direction::Up;
                        col += 1
                    }
                }
                Direction::Right => {
                    while (col as usize) < self.width && self.map[row as usize][col as usize] != '#'
                    {
                        if self.visited.insert((row as usize, col as usize)) {
                            ret += 1;
                        }
                        col += 1;
                    }
                    if (col as usize) >= self.width {
                        break;
                    } else {
                        self.direction = Direction::Down;
                        col -= 1;
                    }
                }
            }
        }
        ret
    }

    fn count_possilbe_to_make_loop(&mut self) -> u32 {
        let mut ret = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.map[row][col] == '#' || self.map[row][col] == '^' {
                    continue;
                } else {
                    self.map[row][col] = '#';
                    if self.is_loop() {
                        ret += 1;
                    }
                    self.direction = Direction::Up;
                    self.map[row][col] = '.';
                }
            }
        }
        ret
    }
    // check if pass a postion 5 times, equavilent to same direction passing twice
    fn is_loop(&mut self) -> bool {
        let mut count: HashMap<(i32, i32), i32> = HashMap::new();
        let mut row = self.start_point.0 as i32;
        let mut col = self.start_point.1 as i32;

        while row >= 0 && (row as usize) < self.height && col >= 0 && (col as usize) < self.width {
            match self.direction {
                Direction::Up => {
                    // go up until touch the map edge or match a '#'
                    while row >= 0 && self.map[row as usize][col as usize] != '#' {
                        *count.entry((row, col)).or_insert(0) += 1;
                        if count.get(&(row, col)) >= Some(&5) {
                            return true;
                        }
                        row -= 1;
                    }
                    if row < 0 {
                        break;
                    } else {
                        // turn right
                        self.direction = Direction::Right;
                        // step back and deduct the times of passing this position, since next move will start from this position and add time of passing again
                        row += 1;
                        *count.get_mut(&(row, col)).unwrap() -= 1;
                    }
                }
                Direction::Down => {
                    while (row as usize) < self.height
                        && self.map[row as usize][col as usize] != '#'
                    {
                        *count.entry((row, col)).or_insert(0) += 1;
                        if count.get(&(row, col)) >= Some(&5) {
                            return true;
                        }
                        row += 1;
                    }
                    if (row as usize) >= self.height {
                        break;
                    } else {
                        // turn right
                        self.direction = Direction::Left;
                        row -= 1;
                        *count.get_mut(&(row, col)).unwrap() -= 1;
                    }
                }
                Direction::Left => {
                    while col >= 0 && self.map[row as usize][col as usize] != '#' {
                        *count.entry((row, col)).or_insert(0) += 1;
                        if count.get(&(row, col)) >= Some(&5) {
                            return true;
                        }
                        col -= 1;
                    }
                    if col < 0 {
                        break;
                    } else {
                        self.direction = Direction::Up;
                        col += 1;
                        *count.get_mut(&(row, col)).unwrap() -= 1;
                    }
                }
                Direction::Right => {
                    while (col as usize) < self.width && self.map[row as usize][col as usize] != '#'
                    {
                        *count.entry((row, col)).or_insert(0) += 1;
                        if count.get(&(row, col)) >= Some(&5) {
                            return true;
                        }
                        col += 1;
                    }
                    if (col as usize) >= self.width {
                        break;
                    } else {
                        self.direction = Direction::Down;
                        col -= 1;
                        *count.get_mut(&(row, col)).unwrap() -= 1;
                    }
                }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut filename = std::env::current_dir().unwrap();
        filename.push("data.txt");
        let mut lab_map = LabMap::load_dataset(filename);

        println!("{:?}", lab_map.count_unique_position()); //5534
        println!("{:?}", lab_map.count_possilbe_to_make_loop()); //2262
    }
}
