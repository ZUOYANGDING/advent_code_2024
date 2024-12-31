use std::{
    borrow::BorrowMut,
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    thread,
    time::Duration,
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
        let p_col = data_vec[0].parse::<i32>().unwrap();
        let p_row = data_vec[1].parse::<i32>().unwrap();
        let v_col = data_vec[2].parse::<i32>().unwrap();
        let v_row = data_vec[3].parse::<i32>().unwrap();
        Self {
            position: (p_row, p_col),
            velocity: (v_row, v_col),
        }
    }

    fn action(&self, height: i32, width: i32, duration: u32) -> (i32, i32) {
        let (mut p_row, mut p_col) = self.position;
        let (v_row, v_col) = self.velocity;
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

    fn action_per_scecond(&mut self, height: i32, width: i32) {
        let (p_row, p_col) = self.position.borrow_mut();
        let (v_row, v_col) = self.velocity;
        *p_row = match v_row.cmp(&0) {
            Ordering::Equal | Ordering::Greater => (*p_row + v_row) % height,
            Ordering::Less => {
                if *p_row + v_row % height >= 0 {
                    *p_row + v_row % height
                } else {
                    height + (*p_row + v_row % height)
                }
            }
        };
        *p_col = match v_col.cmp(&0) {
            Ordering::Equal | Ordering::Greater => (*p_col + v_col) % width,
            Ordering::Less => {
                if *p_col + v_col % height >= 0 {
                    *p_col + v_col % width
                } else {
                    width + (*p_col + v_col % width)
                }
            }
        };
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

    fn robot_move_per_seoncd(&mut self) {
        let mut round = 0;
        loop {
            for idx in 0..self.robots.len() {
                let (row, col) = self.robots.get(idx).unwrap().position;
                self.map[row as usize][col as usize] -= 1;
                Robot::action_per_scecond(
                    self.robots.get_mut(idx).unwrap(),
                    self.height,
                    self.width,
                );
                let (row, col) = self.robots.get(idx).unwrap().position;
                self.map[row as usize][col as usize] += 1;
            }
            if self.is_majority_robots_next_to_each_other() {
                self.print_map(round);
                thread::sleep(Duration::from_secs(10));
                println!("Round work!!!!!! {:?}", round)
            } else {
                println!("Round not work {:?}", round);
            }
            round += 1;
        }
    }

    fn is_majority_robots_next_to_each_other(&self) -> bool {
        let mut grouped_robots = HashSet::new();
        let mut neighbours = HashSet::new();
        for idx in 0..self.robots.len() {
            let (p_row, p_col) = self.robots.get(idx).unwrap().position;
            if neighbours.contains(&(p_row, p_col)) {
                grouped_robots.insert((p_row, p_col));
            }
            // store all possible position of this robot
            // up
            if p_row > 0 {
                neighbours.insert((p_row - 1, p_col));
            }
            // right
            if p_col < self.width - 1 {
                neighbours.insert((p_row, p_col + 1));
            }
            // down
            if p_row < self.height - 1 {
                neighbours.insert((p_row + 1, p_col));
            }
            // left
            if p_col > 0 {
                neighbours.insert((p_row, p_col - 1));
            }
            // up-left
            if p_row > 0 && p_col > 0 {
                neighbours.insert((p_row - 1, p_col - 1));
            }
            // up-right
            if p_row > 0 && p_col < self.width - 1 {
                neighbours.insert((p_row - 1, p_col + 1));
            }
            // down-left
            if p_row < self.height - 1 && p_col > 0 {
                neighbours.insert((p_row + 1, p_col - 1));
            }
            // up-right
            if p_row < self.height - 1 && p_col < self.width - 1 {
                neighbours.insert((p_row + 1, p_col + 1));
            }
        }
        return grouped_robots.len() >= 250;
    }

    fn print_map(&self, round: u32) {
        let mut filename = std::env::current_dir().unwrap();
        let s = format!("day14/part_2_output/{:?}", round);
        filename.push(s.as_str());
        let mut file = File::create(filename).unwrap();
        for i in 0..self.height {
            let mut line = String::new();
            for j in 0..self.width {
                if self.map[i as usize][j as usize] > 0 {
                    line.push_str("* ");
                } else {
                    line.push_str(". ");
                }
            }
            let _ = writeln!(file, "{}", line);
        }
    }

    fn init_map(&mut self) {
        // reset the whole map
        for i in 0..self.height {
            for j in 0..self.width {
                self.map[i as usize][j as usize] = 0;
            }
        }
        for idx in 0..self.robots.len() {
            let (row, col) = self.robots.get(idx).unwrap().position;
            self.map[row as usize][col as usize] = 1;
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
    puzzle.init_map();
    puzzle.robot_move_per_seoncd();
}

#[cfg(test)]
mod test {
    use crate::Puzzle;

    #[test]
    fn test_part_1() {
        let mut file = std::env::current_dir().unwrap();
        file.push("data_test.txt");
        let mut puzzle = Puzzle::load_data(file, 7, 11);
        puzzle.robot_move_per_seoncd();
        println!("{:?}", puzzle);
        println!("{:?}", puzzle.cal_safety_factor());
    }

    #[test]
    fn test_part_2() {
        let mut file = std::env::current_dir().unwrap();
        file.push("data_test.txt");
        let mut puzzle = Puzzle::load_data(file, 7, 11);
        puzzle.init_map();
        puzzle.robot_move_per_seoncd();
    }
}
