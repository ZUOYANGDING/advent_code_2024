use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
struct SignalMap {
    signal_point: HashMap<u8, Vec<(i32, i32)>>,
    antinodes: HashSet<(i32, i32)>,
    antinodes_without_limit: HashSet<(i32, i32)>,
    map_height: u32,
    map_width: u32,
}

impl SignalMap {
    fn load_data(filename: PathBuf) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut signal_point: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
        let mut row = 0;
        let mut width = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let mut col = 0;
            for char in line.as_bytes() {
                if char.is_ascii_digit() || char.is_ascii_alphabetic() {
                    match signal_point.entry(char.to_owned()) {
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().push((row, col));
                        }
                        Entry::Vacant(entry) => {
                            let mut position_vec = Vec::new();
                            position_vec.push((row, col));
                            entry.insert(position_vec);
                        }
                    }
                }
                col += 1;
            }
            width = col;
            row += 1;
        }
        Self {
            signal_point,
            antinodes: HashSet::new(),
            antinodes_without_limit: HashSet::new(),
            map_height: row as u32,
            map_width: width as u32,
        }
    }

    fn detect_antinodes(&mut self) {
        let signal_point = self.signal_point.clone();
        for (_, point_vec) in signal_point.iter() {
            self.detect_antinodes_for_one_frequency(point_vec);
        }
    }

    fn detect_antinodes_for_one_frequency(&mut self, point_vec: &[(i32, i32)]) {
        if point_vec.len() < 2 {
            return;
        }
        for slow_ptr in 0..point_vec.len() {
            for fast_ptr in slow_ptr + 1..point_vec.len() {
                let point_a = point_vec[slow_ptr];
                let point_b = point_vec[fast_ptr];
                let row_diff = (point_a.0 - point_b.0).abs();
                let col_diff = (point_a.1 - point_b.1).abs();
                if point_a.0 <= point_b.0 && point_a.1 <= point_b.1 {
                    // a is on top left of b
                    let antinode_a_row = point_a.0 - row_diff;
                    let antinode_a_col = point_a.1 - col_diff;
                    if antinode_a_row >= 0 && antinode_a_col >= 0 {
                        self.antinodes.insert((antinode_a_row, antinode_a_col));
                    }
                    let antinode_b_row = point_b.0 + row_diff;
                    let antinode_b_col = point_b.1 + col_diff;
                    if antinode_b_row < self.map_height as i32
                        && antinode_b_col < self.map_width as i32
                    {
                        self.antinodes.insert((antinode_b_row, antinode_b_col));
                    }
                } else if point_a.0 <= point_b.0 && point_a.1 >= point_b.1 {
                    // a is on top right of b
                    let antinode_a_row = point_a.0 - row_diff;
                    let antinode_a_col = point_a.1 + col_diff;
                    if antinode_a_row >= 0 && antinode_a_col < self.map_width as i32 {
                        self.antinodes.insert((antinode_a_row, antinode_a_col));
                    }
                    let antinode_b_row = point_b.0 + row_diff;
                    let antinode_b_col = point_b.1 - col_diff;
                    if antinode_b_row < self.map_height as i32 && antinode_b_col >= 0 {
                        self.antinodes.insert((antinode_b_row, antinode_b_col));
                    }
                } else if point_a.0 >= point_b.0 && point_a.1 <= point_b.1 {
                    // a is on down left of b
                    let antinode_a_row = point_a.0 + row_diff;
                    let antinode_a_col = point_a.1 - col_diff;
                    if antinode_a_row < self.map_height as i32 && antinode_a_col >= 0 {
                        self.antinodes.insert((antinode_a_row, antinode_a_col));
                    }
                    let antinode_b_row = point_b.0 - row_diff;
                    let antinode_b_col = point_b.1 + col_diff;
                    if antinode_b_row >= 0 && antinode_b_col < self.map_width as i32 {
                        self.antinodes.insert((antinode_b_row, antinode_b_col));
                    }
                } else if point_a.0 >= point_b.0 && point_b.1 >= point_b.1 {
                    // a is on down right of b
                    let antinode_a_row = point_a.0 + row_diff;
                    let antinode_a_col = point_a.1 + col_diff;
                    if antinode_a_row < self.map_height as i32
                        && antinode_a_col < self.map_width as i32
                    {
                        self.antinodes.insert((antinode_a_row, antinode_a_col));
                    }
                    let antinode_b_row = point_b.0 - row_diff;
                    let antinode_b_col = point_b.1 - col_diff;
                    if antinode_b_row >= 0 && antinode_b_col >= 0 {
                        self.antinodes.insert((antinode_b_row, antinode_b_col));
                    }
                }
            }
        }
    }

    fn detect_antinodes_without_distance_limit(&mut self) {
        let signal_points = self.signal_point.clone();
        for (_, point_vec) in signal_points.iter() {
            self.detect_antinodes_without_distance_limit_for_one_requency(point_vec);
        }
    }

    fn detect_antinodes_without_distance_limit_for_one_requency(
        &mut self,
        point_vec: &[(i32, i32)],
    ) {
        if point_vec.len() < 2 {
            return;
        }
        for slow_ptr in 0..point_vec.len() {
            for fast_ptr in slow_ptr + 1..point_vec.len() {
                let point_a = point_vec[slow_ptr];
                let point_b = point_vec[fast_ptr];
                let row_diff = (point_a.0 - point_b.0).abs();
                let col_diff = (point_a.1 - point_b.1).abs();
                self.antinodes_without_limit.insert((point_a.0, point_a.1));
                self.antinodes_without_limit.insert((point_b.0, point_b.1));
                if point_a.0 <= point_b.0 && point_a.1 <= point_b.1 {
                    // a is on top left of b
                    let mut antinode_a_row = point_a.0 - row_diff;
                    let mut antinode_a_col = point_a.1 - col_diff;
                    while antinode_a_row >= 0 && antinode_a_col >= 0 {
                        self.antinodes_without_limit
                            .insert((antinode_a_row, antinode_a_col));
                        antinode_a_row -= row_diff;
                        antinode_a_col -= col_diff;
                    }
                    let mut antinode_b_row = point_b.0 + row_diff;
                    let mut antinode_b_col = point_b.1 + col_diff;
                    while antinode_b_row < self.map_height as i32
                        && antinode_b_col < self.map_width as i32
                    {
                        self.antinodes_without_limit
                            .insert((antinode_b_row, antinode_b_col));
                        antinode_b_row += row_diff;
                        antinode_b_col += col_diff;
                    }
                } else if point_a.0 <= point_b.0 && point_a.1 >= point_b.1 {
                    // a is on top right of b
                    let mut antinode_a_row = point_a.0 - row_diff;
                    let mut antinode_a_col = point_a.1 + col_diff;
                    while antinode_a_row >= 0 && antinode_a_col < self.map_width as i32 {
                        self.antinodes_without_limit
                            .insert((antinode_a_row, antinode_a_col));
                        antinode_a_row -= row_diff;
                        antinode_a_col += col_diff;
                    }
                    let mut antinode_b_row = point_b.0 + row_diff;
                    let mut antinode_b_col = point_b.1 - col_diff;
                    while antinode_b_row < self.map_height as i32 && antinode_b_col >= 0 {
                        self.antinodes_without_limit
                            .insert((antinode_b_row, antinode_b_col));
                        antinode_b_row += row_diff;
                        antinode_b_col -= col_diff;
                    }
                } else if point_a.0 >= point_b.0 && point_a.1 <= point_b.1 {
                    // a is on down left of b
                    let mut antinode_a_row = point_a.0 + row_diff;
                    let mut antinode_a_col = point_a.1 - col_diff;
                    while antinode_a_row < self.map_height as i32 && antinode_a_col >= 0 {
                        self.antinodes_without_limit
                            .insert((antinode_a_row, antinode_a_col));
                        antinode_a_row += row_diff;
                        antinode_a_col -= col_diff;
                    }
                    let mut antinode_b_row = point_b.0 - row_diff;
                    let mut antinode_b_col = point_b.1 + col_diff;
                    while antinode_b_row >= 0 && antinode_b_col < self.map_width as i32 {
                        self.antinodes_without_limit
                            .insert((antinode_b_row, antinode_b_col));
                        antinode_b_row -= row_diff;
                        antinode_b_col += col_diff;
                    }
                } else if point_a.0 >= point_b.0 && point_b.1 >= point_b.1 {
                    // a is on down right of b
                    let mut antinode_a_row = point_a.0 + row_diff;
                    let mut antinode_a_col = point_a.1 + col_diff;
                    while antinode_a_row < self.map_height as i32
                        && antinode_a_col < self.map_width as i32
                    {
                        self.antinodes_without_limit
                            .insert((antinode_a_row, antinode_a_col));
                        antinode_a_row += row_diff;
                        antinode_a_col += col_diff;
                    }
                    let mut antinode_b_row = point_b.0 - row_diff;
                    let mut antinode_b_col = point_b.1 - col_diff;
                    while antinode_b_row >= 0 && antinode_b_col >= 0 {
                        self.antinodes_without_limit
                            .insert((antinode_b_row, antinode_b_col));
                        antinode_b_row -= row_diff;
                        antinode_b_col -= col_diff;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut filename = std::env::current_dir().unwrap();
    filename = filename.join("day8/data.txt");
    let mut singal_map = SignalMap::load_data(filename);
    singal_map.detect_antinodes();
    println!("{:?}", singal_map.antinodes.len()); //293
    singal_map.detect_antinodes_without_distance_limit();
    println!("{:?}", singal_map.antinodes_without_limit.len());
}
