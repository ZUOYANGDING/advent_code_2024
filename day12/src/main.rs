use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

struct AreaMap {
    regions: HashMap<u8, Vec<(i32, i32)>>,
    origin_map: Vec<Vec<u8>>,
    areas: Vec<Vec<(i32, i32)>>,
    width: i32,
    height: i32,
}

impl AreaMap {
    fn load_data(filename: PathBuf) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut regions: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
        let mut origin_map = Vec::new();
        let mut row = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let mut origin_line = Vec::new();
            for (col, c) in line.as_bytes().iter().enumerate() {
                origin_line.push(*c);
                match regions.entry(*c) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push((row, col as i32));
                    }
                    Entry::Vacant(vacant) => {
                        let mut points = Vec::new();
                        points.push((row, col as i32));
                        vacant.insert(points);
                    }
                }
            }
            origin_map.push(origin_line);
            row += 1;
        }
        Self {
            width: origin_map.get(0).unwrap().len() as i32,
            height: origin_map.len() as i32,
            regions,
            origin_map,
            areas: Vec::new(),
        }
    }

    fn group_region_points_into_areas(&mut self) {
        for (c, mut points) in self.regions.clone() {
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            // sort the points by row
            points.sort_by(|left, right| left.0.partial_cmp(&right.0).unwrap());
            for idx in 0..points.len() {
                if visited.contains(&points[idx]) {
                    continue;
                }
                let mut group = Vec::new();
                let mut neighbour_queue = vec![points[idx]];
                while let Some(point) = neighbour_queue.pop() {
                    if visited.insert(point) {
                        group.push(point);
                        // push neighbours of the point to neighbour_queue
                        // neignbour defined as row/col equal to point's row/col
                        for neignbour in points.iter() {
                            if !visited.contains(neignbour)
                                && (point.0 == neignbour.0 || point.1 == neignbour.1)
                                && ((point.0 - neignbour.0).pow(2) + (point.1 - neignbour.1).pow(2)
                                    == 1)
                            {
                                neighbour_queue.push(neignbour.to_owned());
                            }
                        }
                    }
                }
                println!("char: {:?}, group: {:?}", c as char, group);
                self.areas.push(group);
            }
        }
    }

    fn cal_perimeter_of_each_group(&self, group: &Vec<(i32, i32)>) -> u64 {
        if group.len() < 2 {
            return 4;
        }
        let mut ret = 0;
        let group_char =
            self.origin_map[group.get(0).unwrap().0 as usize][group.get(0).unwrap().1 as usize];
        for point in group.iter() {
            // check it has neighbour on 4 direction,
            // then the perimeter contributed by this point is 4 - neignbours
            let up = if point.0 > 0
                && self.origin_map[(point.0 - 1) as usize][point.1 as usize] == group_char
            {
                1
            } else {
                0
            };
            let right = if point.1 < self.width - 1
                && self.origin_map[point.0 as usize][(point.1 + 1) as usize] == group_char
            {
                1
            } else {
                0
            };
            let down = if point.0 < self.height - 1
                && self.origin_map[(point.0 + 1) as usize][point.1 as usize] == group_char
            {
                1
            } else {
                0
            };
            let left = if point.1 > 0
                && self.origin_map[point.0 as usize][(point.1 - 1) as usize] == group_char
            {
                1
            } else {
                0
            };
            ret += 4 - (up + right + down + left);
        }
        ret
    }

    fn cal_price(&self) -> u64 {
        let mut ret = 0;
        for group in self.areas.iter() {
            let area_size = group.len();
            let area_perimeter = self.cal_perimeter_of_each_group(group);
            ret += area_perimeter * (area_size as u64);
        }
        ret
    }

    /// one side has 2 corners, and one corner will connect 2 sides; Which means we can count corners instead of count sides
    /// Identify corners:
    ///      There are 2 kind of corners, external and internal.
    ///      The external corner is neighboured by different region on at least 2 direction
    ///          ```
    ///          Example with 4 corners, each a is a corner
    ///              aa
    ///              aa
    ///              
    ///          Example with 4 corners, each a are 2 corners
    ///              a
    ///              a
    ///          ```
    ///      The interal corner is neighboured by same region on at least 2 direction, but different region on one diagonal
    ///          ```
    ///          Example with 4 interal corners, and 8 external corners
    ///             baab
    ///            aaaaaa
    ///            aaaaaa
    ///             baab
    ///              
    ///          ```
    fn cal_corners_of_each_group(&self, group: &Vec<(i32, i32)>) -> u64 {
        let mut external_corner = 0;
        let mut internal_cornoer = 0;
        let group_char =
            self.origin_map[group.get(0).unwrap().0 as usize][group.get(0).unwrap().1 as usize];
        for point in group {
            let external = self.is_external_corner(point, group_char);
            if external > 0 {
                println!("external {:?}", point);
                external_corner += external;
            }
            let internal = self.is_internal_corner(point, group_char);
            if internal > 0 {
                println!("internal {:?}", point);
                internal_cornoer += internal;
            }
        }
        println!("{:?}, {:?}", external_corner, internal_cornoer);
        external_corner + internal_cornoer
    }

    fn is_external_corner(&self, point: &(i32, i32), region: u8) -> u64 {
        let (row, col) = point;
        let up = *row == 0
            || self
                .origin_map
                .get((row - 1) as usize)
                .map_or(true, |row_vec| row_vec[*col as usize] != region);

        let right = *col == self.width - 1
            || self.origin_map[*row as usize]
                .get((col + 1) as usize)
                .map_or(true, |c| *c != region);

        let down = *row == self.height - 1
            || self
                .origin_map
                .get((row + 1) as usize)
                .map_or(true, |row_vec| row_vec[*col as usize] != region);

        let left = *col == 0
            || self.origin_map[*row as usize]
                .get((col - 1) as usize)
                .map_or(true, |c| *c != region);
        let mut ret = 0;
        if right && up {
            ret += 1;
        }
        if left && up {
            ret += 1;
        }
        if right && down {
            ret += 1;
        }
        if left && down {
            ret += 1;
        }
        ret
    }

    fn is_internal_corner(&self, point: &(i32, i32), region: u8) -> u64 {
        let (row, col) = point;
        let up = self
            .origin_map
            .get((row - 1) as usize)
            .map_or(false, |row_vec| row_vec[*col as usize] == region);

        let right = self.origin_map[*row as usize]
            .get((*col + 1) as usize)
            .map_or(false, |c| *c == region);

        let down = self
            .origin_map
            .get((row + 1) as usize)
            .map_or(false, |row_vec| row_vec[*col as usize] == region);

        let left = self.origin_map[*row as usize]
            .get((*col - 1) as usize)
            .map_or(false, |c| *c == region);

        let up_left = self
            .origin_map
            .get((row - 1) as usize)
            .map_or(false, |row_vec| {
                row_vec
                    .get((col - 1) as usize)
                    .map_or(false, |c| *c != region)
            });

        let up_right = self
            .origin_map
            .get((row - 1) as usize)
            .map_or(false, |row_vec| {
                row_vec
                    .get((col + 1) as usize)
                    .map_or(false, |c| *c != region)
            });

        let down_left = self
            .origin_map
            .get((row + 1) as usize)
            .map_or(false, |row_vec| {
                row_vec
                    .get((col - 1) as usize)
                    .map_or(false, |c| *c != region)
            });

        let down_right = self
            .origin_map
            .get((row + 1) as usize)
            .map_or(false, |row_vec| {
                row_vec
                    .get((col + 1) as usize)
                    .map_or(false, |c| *c != region)
            });
        let mut ret = 0;
        if right && up && up_right {
            ret += 1;
        }
        if left && up && up_left {
            ret += 1;
        }
        if right && down && down_right {
            ret += 1;
        }
        if left && down && down_left {
            ret += 1;
        }
        ret
    }

    fn cal_price_alter(&self) -> u64 {
        let mut ret = 0;
        for group in self.areas.iter() {
            let area_size = group.len();
            let area_corners = self.cal_corners_of_each_group(group);
            ret += area_corners * (area_size as u64);
        }
        ret
    }
}

fn main() {
    let mut filename = std::env::current_dir().unwrap();
    filename.push("day12/data.txt");
    let mut area_map = AreaMap::load_data(filename);
    area_map.group_region_points_into_areas();
    println!("{:?}", area_map.cal_price()); //1361494
    println!("{:?}", area_map.cal_price_alter()); //830516
}
