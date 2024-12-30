use std::{
    cmp::{self, Ordering},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[derive(Debug, Clone)]
struct Puzzle {
    robots: Vec<Robot>,
    map: Vec<Vec<i32>>,
    height: i32,
    width: i32,
}

impl Robot {
    fn parse_line(data: &str) -> Self {
        let data_vec = data
            .split(&['p', '=', ',', 'v', ' '][..])
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let p_row = data_vec[0].parse::<i32>().unwrap();
        let p_col = data_vec[1].parse::<i32>().unwrap();
        let v_row = data_vec[2].parse::<i32>().unwrap();
        let v_col = data_vec[3].parse::<i32>().unwrap();
        Self {
            position: (p_row, p_col),
            velocity: (v_row, v_col),
        }
    }

    fn action(&self, height: i32, width: i32, duration: u32) -> (i32, i32) {
        let (mut p_col, mut p_row) = self.position;
        let (v_col, v_row) = self.velocity;
        let row_move = v_row * (duration as i32);
        let col_move = v_col * (duration as i32);
        p_row = match row_move.cmp(&0) {
            Ordering::Equal | Ordering::Greater => (p_row + row_move) % height,
            Ordering::Less => {
                if p_row + row_move % height >= 0 {
                    p_row + row_move % height
                } else {
                    height + (p_row + row_move % height)
                }
            }
        };
        p_col = match col_move.cmp(&0) {
            Ordering::Equal | Ordering::Greater => (p_col + col_move) % width,
            Ordering::Less => {
                if p_col + col_move % width >= 0 {
                    p_col + col_move % width
                } else {
                    width + (p_col + col_move % width)
                }
            }
        };
        (p_row, p_col)
    }
}

impl Puzzle {
    fn load_data(filename: PathBuf, height: i32, width: i32) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut robots = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            let robot = Robot::parse_line(line.as_str());
            robots.push(robot);
        }
        let map = vec![vec![0; width as usize]; height as usize];
        Self {
            robots,
            map,
            height,
            width,
        }
    }

    fn robot_move(&mut self, duration: u32) {
        for robot in self.robots.clone() {
            let (row, col) = robot.action(self.height, self.width, duration);
            self.map[row as usize][col as usize] += 1;
        }
    }

    fn cal_safety_factor(&self) -> i32 {
        // first quarant
        let mut first = 0;
        let mut second = 0;
        let mut third = 0;
        let mut fourth = 0;
        for row_idx in 0..self.height {
            for col_idx in 0..self.width {
                if row_idx < self.height / 2 && col_idx < self.width / 2 {
                    first += self.map[row_idx as usize][col_idx as usize];
                } else if row_idx < self.height / 2 && col_idx > self.width / 2 {
                    second += self.map[row_idx as usize][col_idx as usize];
                } else if row_idx > self.height / 2 && col_idx < self.width / 2 {
                    third += self.map[row_idx as usize][col_idx as usize];
                } else if row_idx > self.height / 2 && col_idx > self.width / 2 {
                    fourth += self.map[row_idx as usize][col_idx as usize];
                }
            }
        }
        println!("{:?}, {:?}, {:?}, {:?}", first, second, third, fourth);
        return first * second * third * fourth;
    }
}

fn main() {
    let mut filename = std::env::current_dir().unwrap();
    filename.push("day14/data.txt");
    let mut puzzle = Puzzle::load_data(filename, 103, 101);
    puzzle.robot_move(100);
    println!("{:?}", puzzle.cal_safety_factor()); //218433348
}

#[cfg(test)]
mod test {
    use crate::Puzzle;

    #[test]
    fn test_part_1() {
        let mut file = std::env::current_dir().unwrap();
        file.push("data_test.txt");
        let mut puzzle = Puzzle::load_data(file, 7, 11);
        puzzle.robot_move(100);
        println!("{:?}", puzzle);
        println!("{:?}", puzzle.cal_safety_factor());
    }
}
