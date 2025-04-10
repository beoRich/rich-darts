use crate::components::calculation;
use crate::components::calculation::double_throws_calculation::HowManyOnDouble::{
    NeedEntry, NotRelevant, OnlyOne,
};
use crate::domain::Score;

pub enum HowManyOnDouble {
    NotRelevant,
    OnlyOne,
    NeedEntry(Vec<u16>),
}

pub fn ask_for_double_entry(last_score: &Score, new_score: &Score) -> HowManyOnDouble {
    if calculation::common::is_finish(last_score.remaining) {
        let mut double_attempt_vector: Vec<u16> = vec![0, 1, 2, 3];
        if new_score.remaining == 0 {
            double_attempt_vector.retain(|&x| x != 0)
        }
        let last_remaining = last_score.remaining;
        if last_remaining > 110 {
            double_attempt_vector.retain(|&x| x != 2 && x != 3)
        }
        if last_remaining % 2 == 1 || (last_remaining > 40 && last_remaining != 50) {
            double_attempt_vector.retain(|&x| x != 3);
        }

        if double_attempt_vector.len() == 1 {
            return OnlyOne;
        }
        NeedEntry(double_attempt_vector)
    } else {
        NotRelevant
    }
}

#[cfg(test)]
mod test {
    use crate::components::calculation::common;
    use crate::components::calculation::double_throws_calculation::HowManyOnDouble::{NeedEntry, NotRelevant, OnlyOne};
    use crate::components::calculation::double_throws_calculation::{
        ask_for_double_entry, HowManyOnDouble,
    };
    use crate::domain::Score;

    fn helper(remaining: u16, thrown: u16) -> Score {
        Score {
            leg_id: 1,
            remaining,
            thrown,
            throw_order: 1,
            double_attempt: None,
        }
    }

    #[test]
    fn no_0_on_suc_finish() {
        let finshes: Vec<u16> = (2..171)
            .into_iter()
            .map(|s| s as u16)
            .filter(|val| common::is_finish(*val))
            .collect();
        let ask_entries: Vec<(u16, HowManyOnDouble)> = finshes
            .into_iter()
            .map(|val| (val, ask_for_double_entry(&helper(20, val), &helper(0, 20))))
            .collect();
        for (val, entry) in ask_entries.into_iter() {
            match entry {
                NotRelevant => {panic!("wrong ask_entry for {}", val);},
                OnlyOne => {//good
                     },
                NeedEntry(val) => {assert!(!val.contains(&0));}
            }
        }
    }
}
