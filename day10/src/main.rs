use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
struct RoadMap {
    map: Vec<Vec<i32>>,
    trail_heads: Vec<(i32, i32)>,
    wide: i32,
    height: i32,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Nil,
}

impl RoadMap {
    fn load_file(filename: PathBuf) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut map: Vec<Vec<i32>> = Vec::new();
        let mut trail_heads = Vec::new();
        let mut row = 0_i32;
        for line in reader.lines() {
            let line = line.unwrap();
            let mut col = 0;
            let mut line_vec = Vec::new();
            for ch in line.as_bytes() {
                let number = (ch - b'0') as i32;
                if number == 0 {
                    trail_heads.push((row, col));
                }
                line_vec.push(number);
                col += 1;
            }
            map.push(line_vec);
            row += 1;
        }

        Self {
            wide: map[0].len() as i32,
            height: map.len() as i32,
            map,
            trail_heads,
        }
    }

    fn cal_score(&self) -> i32 {
        let mut ret = 0;
        for (row, col) in &self.trail_heads {
            let mut visited_nine: HashSet<(i32, i32)> = HashSet::new();
            ret += self.dfs(*row, *col, -1, Direction::Nil, &mut visited_nine);
        }
        ret
    }

    fn cal_distinct_score(&self) -> i32 {
        let mut ret = 0;
        for (row, col) in &self.trail_heads {
            ret += self.dfs_distinct(*row, *col, -1, Direction::Nil);
        }
        ret
    }

    fn dfs(
        &self,
        row: i32,
        col: i32,
        prev_num: i32,
        pre_direction: Direction,
        visited_nine: &mut HashSet<(i32, i32)>,
    ) -> i32 {
        // should check this first before check number is 9 or not
        if self.map[row as usize][col as usize] - prev_num != 1 {
            return 0;
        } else if self.map[row as usize][col as usize] == 9 {
            // check the 9 has been visited or not
            if visited_nine.insert((row, col)) {
                return 1;
            } else {
                return 0;
            }
        } else {
            let up_ret = if row > 0 && pre_direction != Direction::Down {
                self.dfs(
                    row - 1,
                    col,
                    self.map[row as usize][col as usize],
                    Direction::Up,
                    visited_nine,
                )
            } else {
                0
            };
            let right_ret = if col < self.wide - 1 && pre_direction != Direction::Left {
                self.dfs(
                    row,
                    col + 1,
                    self.map[row as usize][col as usize],
                    Direction::Right,
                    visited_nine,
                )
            } else {
                0
            };
            let down_ret = if row < self.height - 1 && pre_direction != Direction::Up {
                self.dfs(
                    row + 1,
                    col,
                    self.map[row as usize][col as usize],
                    Direction::Down,
                    visited_nine,
                )
            } else {
                0
            };
            let left_ret = if col > 0 && pre_direction != Direction::Right {
                self.dfs(
                    row,
                    col - 1,
                    self.map[row as usize][col as usize],
                    Direction::Left,
                    visited_nine,
                )
            } else {
                0
            };
            return up_ret + right_ret + down_ret + left_ret;
        }
    }

    fn dfs_distinct(&self, row: i32, col: i32, prev_num: i32, pre_direction: Direction) -> i32 {
        if self.map[row as usize][col as usize] - prev_num != 1 {
            return 0;
        } else if self.map[row as usize][col as usize] == 9 {
            return 1;
        } else {
            let up_ret = if row > 0 && pre_direction != Direction::Down {
                self.dfs_distinct(
                    row - 1,
                    col,
                    self.map[row as usize][col as usize],
                    Direction::Up,
                )
            } else {
                0
            };

            let right_ret = if col < self.wide - 1 && pre_direction != Direction::Left {
                self.dfs_distinct(
                    row,
                    col + 1,
                    self.map[row as usize][col as usize],
                    Direction::Right,
                )
            } else {
                0
            };

            let down_ret = if row < self.height - 1 && pre_direction != Direction::Up {
                self.dfs_distinct(
                    row + 1,
                    col,
                    self.map[row as usize][col as usize],
                    Direction::Down,
                )
            } else {
                0
            };

            let left_ret = if col > 0 && pre_direction != Direction::Right {
                self.dfs_distinct(
                    row,
                    col - 1,
                    self.map[row as usize][col as usize],
                    Direction::Left,
                )
            } else {
                0
            };
            return up_ret + right_ret + down_ret + left_ret;
        }
    }
}

fn main() {
    let mut filename = std::env::current_dir().unwrap();
    filename.push("day10/data.txt");
    let road_map = RoadMap::load_file(filename);
    // println!("{:?}", road_map);
    println!("{:?}", road_map.cal_score()); //733
    println!("{:?}", road_map.cal_distinct_score());
}
