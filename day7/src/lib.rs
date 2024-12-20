use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
struct Solution {
    dataset: Vec<(u64, Vec<u64>)>,
    answer_part_one: u64,
    answer_part_two: u64,
}

impl Solution {
    fn load_data(filename: PathBuf) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut dataset = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let parts = line.split(":").collect::<Vec<&str>>();
            let answer = parts[0].parse::<u64>().unwrap();
            let components: Vec<u64> = parts[1]
                .to_string()
                .split_whitespace()
                .into_iter()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
            dataset.push((answer, components));
        }
        Self {
            dataset,
            answer_part_one: 0,
            answer_part_two: 0,
        }
    }

    fn part_a(&mut self) {
        for (answer, component) in self.dataset.iter() {
            let mut result_set = HashSet::new();
            self.strict_add_or_mul_from_left_to_right(component, component[0], 1, &mut result_set);
            if result_set.contains(answer) {
                self.answer_part_one += answer;
            }
        }
    }

    fn part_b(&mut self) {
        for (answer, component) in self.dataset.iter() {
            let mut result_set = HashSet::new();
            self.strict_add_or_mul_or_concat_from_left_to_right(
                component,
                component[0],
                1,
                &mut result_set,
            );
            if result_set.contains(answer) {
                self.answer_part_two += answer;
            }
        }
    }

    // this is wrong
    // because the problem need to do strict from left to right
    // for [1, 2, 3], the following will create result as
    //  1+2+3, (1*2)+3, 1*(2+3), (1+2)*3, 1+(2*3) and 1*2*3
    // but strict from left to right are:
    // 1+2+3, (1*2)+3, (1+2)*3 and 1*2*3
    fn naive_get_all_possible_answer(&self, component: &[u64]) -> HashSet<u64> {
        if component.len() == 1 {
            let mut ret = HashSet::new();
            ret.insert(component[0]);
            return ret;
        }
        let mut result = HashSet::new();
        for idx in 1..component.len() {
            let left = &component[0..idx];
            let right = &component[idx..];

            let left_results = self.naive_get_all_possible_answer(left);
            let right_results = self.naive_get_all_possible_answer(right);
            for left_result in &left_results {
                for right_result in &right_results {
                    result.insert(left_result + right_result);
                    result.insert(left_result * right_result);
                }
            }
        }
        return result;
    }

    fn strict_add_or_mul_from_left_to_right(
        &self,
        component: &[u64],
        prev_res: u64,
        cur_idx: usize,
        result_set: &mut HashSet<u64>,
    ) {
        if cur_idx == component.len() {
            result_set.insert(prev_res);
            return;
        }
        // add current number
        self.strict_add_or_mul_from_left_to_right(
            component,
            prev_res + component[cur_idx],
            cur_idx + 1,
            result_set,
        );
        // mul current number
        self.strict_add_or_mul_from_left_to_right(
            component,
            prev_res * component[cur_idx],
            cur_idx + 1,
            result_set,
        );
        // concate
    }

    fn strict_add_or_mul_or_concat_from_left_to_right(
        &self,
        component: &[u64],
        prev_res: u64,
        cur_idx: usize,
        result_set: &mut HashSet<u64>,
    ) {
        let concat_number = |num_left: u64, num_right: u64| {
            let num = format!("{}{}", num_left, num_right);
            num.parse::<u64>().unwrap()
        };
        if cur_idx == component.len() {
            result_set.insert(prev_res);
            return;
        }
        // add
        self.strict_add_or_mul_or_concat_from_left_to_right(
            component,
            prev_res + component[cur_idx],
            cur_idx + 1,
            result_set,
        );
        // mul
        self.strict_add_or_mul_or_concat_from_left_to_right(
            component,
            prev_res * component[cur_idx],
            cur_idx + 1,
            result_set,
        );
        // concat
        self.strict_add_or_mul_or_concat_from_left_to_right(
            component,
            concat_number(prev_res, component[cur_idx]),
            cur_idx + 1,
            result_set,
        );
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut filename = std::env::current_dir().unwrap();
        filename.push("data.txt");
        let mut solution = Solution::load_data(filename);
        solution.part_a();
        println!("{:?}", solution.answer_part_one); //1399219271639
        solution.part_b();
        println!("{:?}", solution.answer_part_two); //275791737999003
    }
}
