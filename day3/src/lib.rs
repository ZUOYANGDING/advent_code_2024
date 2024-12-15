use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
struct Solution;

impl Solution {
    fn get_answer(filename: PathBuf) -> u64 {
        let mut file = File::open(filename).expect("File does not exists");
        let mut buffer = Vec::new();
        let _ = file.read_to_end(&mut buffer);
        let mut index = 0;
        let mut ret = 0_u64;
        while index < buffer.len() {
            if buffer[index].is_ascii() && buffer[index] == 'm' as u8 && index + 4 < buffer.len() {
                // check "mul("
                if Self::check_mul(&buffer, index) {
                    index += 4;
                    // get first number
                    let num1 = Self::get_number(&buffer, &mut index);
                    // check if format as first_number,
                    if index < buffer.len()
                        && buffer[index].is_ascii()
                        && (buffer[index] == ',' as u8)
                    {
                        // skip the ,
                        index += 1;
                        // get second number
                        let num2 = Self::get_number(&buffer, &mut index);
                        // check if format as second_number)
                        if index < buffer.len()
                            && buffer[index].is_ascii()
                            && buffer[index] == ')' as u8
                        {
                            ret += num1 * num2;
                        }
                        // skip the ) or just step forward
                        index += 1;
                    } else {
                        // just step forword
                        index += 1;
                    }
                } else {
                    // just step forward
                    index += 1;
                }
            } else {
                // just step forward
                index += 1;
            }
        }
        ret
    }

    fn get_answer_with_ops(filename: PathBuf) -> u64 {
        let mut file = File::open(filename).expect("File does not exists");
        let mut buffer = Vec::new();
        let _ = file.read_to_end(&mut buffer);
        let mut index = 0;
        let mut ret = 0_u64;
        let mut is_do = true;
        while index < buffer.len() {
            if buffer[index].is_ascii() && buffer[index] == 'd' as u8 {
                if index + 7 < buffer.len() && Self::check_dont(&mut buffer, index) {
                    index += 7;
                    is_do = false;
                } else if index + 4 < buffer.len() && Self::check_do(&mut buffer, index) {
                    index += 4;
                    is_do = true;
                }
            }
            if index >= buffer.len() {
                break;
            }
            if buffer[index].is_ascii()
                && buffer[index] == 'm' as u8
                && index + 4 < buffer.len()
                && is_do
            {
                // check "mul("
                if Self::check_mul(&buffer, index) {
                    index += 4;
                    // get first number
                    let num1 = Self::get_number(&buffer, &mut index);
                    // check if format as first_number,
                    if index < buffer.len()
                        && buffer[index].is_ascii()
                        && (buffer[index] == ',' as u8)
                    {
                        // skip the ,
                        index += 1;
                        // get second number
                        let num2 = Self::get_number(&buffer, &mut index);
                        // check if format as second_number)
                        if index < buffer.len()
                            && buffer[index].is_ascii()
                            && buffer[index] == ')' as u8
                        {
                            ret += num1 * num2;
                        }
                        // skip the ) or just step forward
                        index += 1;
                    } else {
                        // just step forword
                        index += 1;
                    }
                } else {
                    // just step forward
                    index += 1;
                }
            } else {
                // just step forward
                index += 1;
            }
        }
        ret
    }

    /// check the "mul("
    fn check_mul(file: &[u8], idx: usize) -> bool {
        file.get(idx..idx + 4) == Some(b"mul(")
    }

    /// check the "don't()"
    fn check_dont(file: &[u8], idx: usize) -> bool {
        file.get(idx..idx + 7) == Some(b"don't()")
    }

    fn check_do(file: &[u8], idx: usize) -> bool {
        file.get(idx..idx + 4) == Some(b"do()")
    }

    fn get_number(file: &[u8], idx: &mut usize) -> u64 {
        let mut ret_num = 0;
        while *idx < file.len() && file[*idx].is_ascii_digit() {
            ret_num = ret_num * 10 + (file[*idx] - b'0') as u64;
            *idx += 1;
        }
        ret_num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Answer 174336360, 88802350
        let mut filename = std::env::current_dir().unwrap();
        filename.push("data.txt");
        println!("{:?}", Solution::get_answer(filename.clone()));
        println!("{:?}", Solution::get_answer_with_ops(filename));
    }
}
