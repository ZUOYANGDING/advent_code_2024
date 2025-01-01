use std::{
    fmt::Display,
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
struct ParseDirectionErr;
impl Display for ParseDirectionErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot parse")
    }
}

impl TryFrom<char> for Direction {
    type Error = ParseDirectionErr;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            _ => Err(ParseDirectionErr),
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    map: Vec<Vec<char>>,
    moves: Vec<Direction>,
    position: (i32, i32),
    height: i32,
    width: i32,
}

impl Problem {
    fn load_data(filename: PathBuf) -> Self {
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        // read map in
        let mut map = Vec::new();
        let mut position = (0, 0);
        let mut row = 0;
        while reader.read_line(&mut buf).unwrap() > 0 {
            if buf.len() == 1 && buf == "\n" {
                buf.clear();
                break;
            }
            let mut line = vec![];
            for (idx, element) in buf.as_bytes().iter().enumerate() {
                let charactor = *element as char;
                if charactor == '\n' {
                    break;
                }
                if charactor == '@' {
                    position = (row, idx as i32);
                }
                line.push(*element as char)
            }
            map.push(line);
            row += 1;
            buf.clear();
        }
        // load moves
        let mut moves = Vec::new();
        while reader.read_line(&mut buf).unwrap() > 0 {
            for element in buf.as_bytes() {
                if *element as char == '\n' {
                    break;
                }
                match Direction::try_from(*element as char) {
                    Ok(direction) => moves.push(direction),
                    Err(e) => println!("{:?}", e),
                }
            }
            buf.clear();
        }
        Self {
            height: map.len() as i32,
            width: map.get(0).unwrap().len() as i32,
            map,
            moves,
            position,
        }
    }

    fn moving(&mut self) {
        let mut row = self.position.0;
        let mut col = self.position.1;
        for movement in self.moves.iter() {
            match movement {
                &Direction::Up => {
                    if Self::can_move(&self.map, row, col, Direction::Up) {
                        self.map[row as usize][col as usize] = '.';
                        Self::change_map(&mut self.map, row - 1, col, Direction::Up);
                        row -= 1;
                    }
                }
                &Direction::Right => {
                    if Self::can_move(&self.map, row, col, Direction::Right) {
                        self.map[row as usize][col as usize] = '.';
                        Self::change_map(&mut self.map, row, col + 1, Direction::Right);
                        col += 1;
                    }
                }
                &Direction::Down => {
                    if Self::can_move(&self.map, row, col, Direction::Down) {
                        self.map[row as usize][col as usize] = '.';
                        Self::change_map(&mut self.map, row + 1, col, Direction::Down);
                        row += 1;
                    }
                }
                &Direction::Left => {
                    if Self::can_move(&self.map, row, col, Direction::Left) {
                        self.map[row as usize][col as usize] = '.';
                        Self::change_map(&mut self.map, row, col - 1, Direction::Left);
                        col -= 1;
                    }
                }
            }
        }
    }

    fn can_move(map: &Vec<Vec<char>>, mut row: i32, mut col: i32, direction: Direction) -> bool {
        while map[row as usize][col as usize] != '#' {
            if map[row as usize][col as usize] == '.' {
                return true;
            }
            match direction {
                Direction::Up => row -= 1,
                Direction::Right => col += 1,
                Direction::Down => row += 1,
                Direction::Left => col -= 1,
            }
        }
        return false;
    }

    fn change_map(map: &mut Vec<Vec<char>>, mut row: i32, mut col: i32, direction: Direction) {
        let mut prev_char = '@';
        while map[row as usize][col as usize] != '#' {
            if map[row as usize][col as usize] == '.' {
                map[row as usize][col as usize] = prev_char;
                break;
            } else {
                let cur_char = map[row as usize][col as usize];
                map[row as usize][col as usize] = prev_char;
                prev_char = cur_char;
                match direction {
                    Direction::Up => row -= 1,
                    Direction::Right => col += 1,
                    Direction::Down => row += 1,
                    Direction::Left => col -= 1,
                }
            }
        }
    }

    fn cal_coordinates(&self) -> u64 {
        let mut ret = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.map[row as usize][col as usize] == 'O' {
                    ret += (row * 100 + col) as u64;
                }
            }
        }
        ret
    }
}

fn main() {
    let mut filename = std::env::current_dir().unwrap();
    filename.push("day15/data.txt");
    let mut problem = Problem::load_data(filename);
    problem.moving();
    println!("{:?}", problem.cal_coordinates());
}

#[cfg(test)]
mod test {
    use crate::Problem;

    #[test]
    fn test_load() {
        let mut filename = std::env::current_dir().unwrap();
        filename.push("data_test.txt");
        let mut problem = Problem::load_data(filename);
        problem.moving();
        println!("{:?}", problem.map);
        println!("{:?}", problem.moves);
        println!("{:?}", problem.cal_coordinates());
    }
}
