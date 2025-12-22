pub fn solve(input: &str) -> i32 {
    let mut current_pos: i32 = 50;
    let mut number_of_hits: i32 = 0;

    for line in input.lines() {
        if line.starts_with("R") {
            let num_str: &str = &line[1..];
            let num: i32 = num_str.parse::<i32>().unwrap_or(0);

            current_pos = ((current_pos + num) % 100 + 100) % 100;

            if current_pos == 0 {
                number_of_hits += 1;
            }
        }
        if line.starts_with("L") {
            let num_str = &line[1..];
            let num = num_str.parse::<i32>().unwrap_or(0);

            current_pos = ((current_pos - num) % 100 + 100) % 100;

            if current_pos == 0 {
                number_of_hits += 1;
            }
        }
    }

    number_of_hits
}

pub fn solve_p2(input: &str) -> i32 {
    let mut current_pos: i32 = 50;
    let mut count = 0;

    for line in input.lines() {
        if line.starts_with("R") {
            let num = line[1..].parse::<i32>().unwrap();
            // let end_pos = (current_pos + num) % 100;
            let mut temp = current_pos;

            for _ in 0..num {
                temp = (temp + 1) % 100;
                if temp == 0 {
                    count += 1;
                }
            }
            current_pos = temp;
        }
        if line.starts_with("L") {
            let num = line[1..].parse::<i32>().unwrap();
            let mut temp = current_pos;
            for _ in 0..num {
                temp = (temp - 1 + 100) % 100;
                if temp == 0 {
                    count += 1;
                }
            }
            current_pos = temp;
        }
    }

    count
}