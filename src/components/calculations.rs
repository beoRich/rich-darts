use crate::domain::CurrentScore;
use itertools::{iproduct};
use std::collections::HashSet;

pub fn valid_thrown(val: u16) -> bool {
    let all_possible_values: HashSet<u16> = possible_values();
    match val {
        val if val > 180 => false,
        val if all_possible_values.contains(&val) => true,
        _ => false,
    }
}

//todo store results as constant
fn possible_values() -> HashSet<u16> {
    let mut singles: Vec<u16> = (0..21).collect();
    singles.push(25);
    singles.push(50);
    let double: Vec<u16> = (0..21).map(|val| val * 2).collect();
    let triples: Vec<u16> = (0..21).map(|val| val * 3).collect();

    let all: Vec<u16> = <HashSet<u16> as IntoIterator>::into_iter(HashSet::from_iter(
        vec![singles, double, triples].concat(),
    ))
    .into_iter()
    .collect();

    let mut sums: HashSet<u16> = HashSet::new();
    for (a, b, c) in iproduct!(all.clone(), all.clone(), all.clone()) {
        sums.insert(a + b + c);
    }
    sums
}

pub fn calculate_remaining(last: CurrentScore, val: u16, next_throw_order: u16) -> CurrentScore {
    let last_remaining = last.remaining;
    let new_remaining: u16;
    if val <= last_remaining && check_possible_remaining(last_remaining - val, val) {
        new_remaining = last_remaining - val;
    } else {
        new_remaining = last_remaining;
    }
    CurrentScore {
        remaining: new_remaining,
        thrown: val,
        throw_order: next_throw_order
    }
}

fn check_possible_remaining(possible_remaining: u16, val: u16) -> bool {
    match possible_remaining {
        1 => false,
        0 => {
            let boogey_nr: Vec<u16> = vec![169, 168, 166, 165, 163, 162, 159];
            match val {
                val if boogey_nr.contains(&val) => false,
                0..170 => true,
                _ => false,
            }
        }
        _ => true,
    }
}

#[cfg(test)]
mod test {
    use crate::components::calculations::{
        calculate_remaining, check_possible_remaining, valid_thrown,
    };
    use crate::domain::CurrentScore;

    #[test]
    fn invalid_throw_bigger_then_180() {
        assert!(!valid_thrown(181));
        assert!(!valid_thrown(340));
    }

    #[test]
    fn invalid_throw_edge() {
        assert!(!valid_thrown(172));
        assert!(!valid_thrown(179));
    }

    #[test]
    fn valid_throw_edge() {
        assert!(valid_thrown(171));
        assert!(valid_thrown(180));
        assert!(valid_thrown(170));
        assert!(valid_thrown(2));
    }

    #[test]
    fn valid_throw_arbitrary() {
        assert!(valid_thrown(120));
        assert!(valid_thrown(100));
        assert!(valid_thrown(63));
    }

    #[test]
    fn should_not_end_on_1() {
        let last = CurrentScore {
            remaining: 141,
            thrown: 180,
            throw_order: 0};
        let thrown = 140;
        let result = calculate_remaining(last, thrown, 0);
        assert_eq!(
            result,
            CurrentScore {
                remaining: 141,
                throw_order: 0,
                thrown
            }
        )
    }

    #[test]
    fn should_not_end_on_negative() {
        let last = CurrentScore {
                remaining: 141,
                thrown: 180,
                throw_order: 0};
        let thrown = 150;
        let result = calculate_remaining(last, thrown, 0);
        assert_eq!(
            result,
            CurrentScore {
                remaining: 141,
                throw_order: 0,
                thrown
            }
        )
    }

    #[test]
    fn should_end_on_0() {
        let thrown = 141;
        let last = CurrentScore {
                remaining: 141,
                thrown: 180,
                throw_order: 0,
            };
        let result = calculate_remaining(last, thrown, 0);
        assert_eq!(
            result,
            CurrentScore {
                remaining: 0,
                throw_order: 0,
                thrown
            }
        )
    }

    #[test]
    fn should_not_end_on_0_if_impossible() {
        assert!(!check_possible_remaining(0, 171));
        assert!(!check_possible_remaining(0, 163))
    }
}
