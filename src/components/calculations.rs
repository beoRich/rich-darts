use crate::components::domain::CurrentScore;

pub fn calculate_remaining(count: &Vec<CurrentScore>, val: u16) -> CurrentScore {
    let last = count.last().unwrap();
    let last_remaining = last.remaining;
    let new_remaining: u16;
    let possible_remaining = last_remaining - val;
    if val <= last_remaining && check_possible_remaining(possible_remaining) {
        new_remaining = possible_remaining;
    } else {
        new_remaining = last_remaining;
    }
    CurrentScore {
        remaining: new_remaining,
        thrown: val,
    }
}

fn check_possible_remaining(possible_remaning: u16) -> bool {
    match possible_remaning {
        1 => false,
        _ => true
    }
}