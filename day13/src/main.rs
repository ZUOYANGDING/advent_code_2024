use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
struct Machine {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

impl Machine {
    fn parse_machine(lines: &Vec<String>) -> Self {
        let button_a = Self::parse_from_line(lines[0].as_str());
        let button_b = Self::parse_from_line(lines[1].as_str());
        let prize = Self::parse_from_line(lines[2].as_str());
        Machine {
            button_a,
            button_b,
            prize,
        }
    }
    fn parse_from_line(line: &str) -> (u64, u64) {
        let parts: Vec<&str> = line
            .split(&['+', '=', 'X', 'Y', ',', ':', ' ', 'A', 'B'][..])
            .filter(|s| !s.is_empty())
            .collect();
        (
            parts[1].parse::<u64>().unwrap(),
            parts[2].parse::<u64>().unwrap(),
        )
    }
}

#[derive(Debug, Clone)]
struct Dataset(Vec<Machine>);

impl Dataset {
    fn load_data(filename: PathBuf) -> Dataset {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut machines = Vec::new();
        let mut lines_group = Vec::new();
        for line in reader.lines().flatten() {
            if !line.is_empty() {
                lines_group.push(line);
            }
            if lines_group.len() == 3 {
                machines.push(Machine::parse_machine(&lines_group));
                lines_group.clear();
            }
        }
        Dataset(machines)
    }

    fn adjust_prize(&mut self) {
        let mut idx = 0;
        while idx < self.0.len() {
            let machine = self.0.get_mut(idx).unwrap();
            machine.prize.0 += 10000000000000;
            machine.prize.1 += 10000000000000;
            idx += 1;
        }
    }

    fn cal_cost(&self) -> u64 {
        let mut ret = 0;
        for machine in self.0.iter() {
            if let Some((time_a, time_b)) =
                Self::solve_equation(machine.button_a, machine.button_b, machine.prize)
            {
                ret += (3 * time_a + time_b) as u64;
            }
        }
        ret
    }

    fn solve_equation(a: (u64, u64), b: (u64, u64), c: (u64, u64)) -> Option<(u64, u64)> {
        let (a1, a2) = a;
        let (b1, b2) = b;
        let (c1, c2) = c;
        let denominate = (a1 * b2) as i64 - (a2 * b1) as i64;
        if denominate == 0 {
            return None;
        }
        let x = if ((c1 * b2) as i64 - (c2 * b1) as i64) % denominate == 0 {
            ((c1 * b2) as i64 - (c2 * b1) as i64) / denominate
        } else {
            -1
        };
        let y = if ((c2 * a1) as i64 - (c1 * a2) as i64) % denominate == 0 {
            ((c2 * a1) as i64 - (c1 * a2) as i64) / denominate
        } else {
            -1
        };
        if x >= 0 && y >= 0 {
            Some((x as u64, y as u64))
        } else {
            None
        }
    }
}

fn main() {
    let mut filename = std::env::current_dir().unwrap();
    filename.push("day13/data.txt");
    let mut machines = Dataset::load_data(filename);
    println!("{:?}", machines.cal_cost()); //32041
    machines.adjust_prize();
    println!("{:?}", machines);
    println!("{:?}", machines.cal_cost()); //95843948914827
}

#[cfg(test)]
mod test {
    use crate::Machine;

    #[test]
    fn test_split() {
        let s = "Price: X+8400, Y+5400";
        println!("{:?}", Machine::parse_from_line(s));
    }
}
