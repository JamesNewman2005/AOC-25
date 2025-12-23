use crate::helpers::check_pattern::has_pattern;

pub fn solve(input: &str) -> u64 {

    let rows: Vec<&str> = input.split(',').map(|s| s.trim()).collect();

    let mut total_sum: u64 = 0;
    
    for row in rows {
        let range: Vec<&str> = row.split("-").collect();

        let start_num: u64 = range[0].parse().unwrap();
        let end_num: u64 = range[1].parse().unwrap();

        for num in start_num..=end_num {
            let num_str = num.to_string();
            if num_str.len() % 2 == 0 {
                if has_pattern(&num_str) {
                    total_sum += num;
                }
            }
        }
    }

    total_sum
}