// Simple helper to check if a string has a pattern

pub fn has_pattern(s: &str) -> bool {
    let len = s.len();
    let mid_point =  len / 2;

    let (first_half, second_half) = s.split_at(mid_point);

    first_half == second_half
}