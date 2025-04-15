use crate::components::calculation;
use crate::components::calculation::double_throws_calculation::HowManyOnDouble::{
    NeedEntry, NotRelevant, OnlyOne,
};
use crate::components::calculation::recommendation_calculation::{FinishRecValue, RecValue};
use crate::domain::Score;

pub enum HowManyOnDouble {
    NotRelevant,
    OnlyOne,
    NeedEntry(Vec<u16>),
}

pub fn ask_for_double_entry(last_score: &Score, new_score: &Score) -> HowManyOnDouble {
    if calculation::common::is_finish(last_score.remaining) {
        let mut double_attempt_vector: Vec<u16> = vec![0, 1, 2, 3];
        if new_score.remaining > 50 && new_score.thrown != 0 {
            NotRelevant
        }
        else {
            if new_score.remaining == 0 {
                double_attempt_vector.retain(|&x| x != 0)
            }
            let last_remaining = last_score.remaining;
            if last_remaining > 110 {
                double_attempt_vector.retain(|&x| x != 2 && x != 3)
            }

            if last_remaining >= 99 && last_remaining < 110 {
                let value = calculation::recommendation_calculation::determine_rec(last_remaining);
                match value {
                    RecValue::IsFinish(FinishRecValue { primary_rec, secondary_rec: _ }) => {
                        match primary_rec {
                            Some(val) => { if val.len() == 3 { double_attempt_vector.retain(|&x| x != 3) } }
                            None => { panic!("Must have a primary recommendation") }
                        }
                    }
                    _ => { panic!("Missing recommendation") }
                }
            }


            if last_remaining % 2 == 1 || (last_remaining > 40 && last_remaining != 50) {
                double_attempt_vector.retain(|&x| x != 3);
            }

            if double_attempt_vector.len() == 1 {
                return OnlyOne;
            }
            NeedEntry(double_attempt_vector)
        }
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
        let finishes: Vec<u16> = (2..171)
            .into_iter()
            .map(|s| s as u16)
            .filter(|val| common::is_finish(*val))
            .collect();
        let ask_entries: Vec<(u16, HowManyOnDouble)> = finishes
            .into_iter()
            .map(|val| (val, ask_for_double_entry(&helper(20, val), &helper(0, 20))))
            .collect();
        for (test_val, entry) in ask_entries.into_iter() {
            match entry {
                NotRelevant => {panic!("wrong ask_entry for {}", test_val);},
                OnlyOne => {//good
                     },
                NeedEntry(rec_val) => {assert!(!rec_val.contains(&0));}
            }
        }
    }

    #[test]
    fn no_3_on_finish_that_are_not_doubles() {
        let finishes: Vec<u16> = (2..171)
            .into_iter()
            .map(|s| s as u16)
            .filter(|val| common::is_finish(*val))
            .filter(|val| !(val < &41 && val % 2==0 ))
            .filter(|val| !(val == &50))
            .collect();
        let ask_entries: Vec<(u16, HowManyOnDouble)> = finishes
            .into_iter()
            .map(|val| (val, ask_for_double_entry(&helper(val, val), &helper(0, 20))))
            .collect();
        for (test_val, entry) in ask_entries.into_iter() {
            match entry {
                NotRelevant => {panic!("wrong ask_entry (NotRelevant) for {}", test_val);},
                OnlyOne => {},
                NeedEntry(rec_val) => {if rec_val.contains(&0) || rec_val.contains(&3) {
                    panic!("wrong rec_vals (needentry) for {} with {:?}", test_val, rec_val);
                };}
            }
        }
    }

    #[test]
    fn no_3_on_finish_that_are_not_doubles_not_suc() {
        let finishes: Vec<u16> = (2..171)
            .into_iter()
            .map(|s| s as u16)
            .filter(|val| common::is_finish(*val))
            .filter(|val| !(val < &41 && val % 2==0 ))
            .filter(|val| !(val == &50))
            .collect();
        let ask_entries: Vec<(u16, HowManyOnDouble)> = finishes
            .into_iter()
            .map(|val| (val, ask_for_double_entry(&helper(val, val), &helper(20, 20))))
            .collect();
        for (test_val, entry) in ask_entries.into_iter() {
            match entry {
                NotRelevant => {panic!("wrong ask_entry (NotRelevant) for {}", test_val);},
                OnlyOne => {},
                NeedEntry(rec_val) => {if rec_val.contains(&3) {
                    panic!("wrong rec_vals (needentry) for {} with {:?}", test_val, rec_val);
                };}
            }
        }
    }

    #[test]
    fn not_relevant_on_non_finishes() {
        let finishes: Vec<u16> = (2..501)
            .into_iter()
            .map(|s| s as u16)
            .filter(|val| !common::is_finish(*val))
            .collect();
        let ask_entries: Vec<(u16, HowManyOnDouble)> = finishes
            .into_iter()
            .map(|val| (val, ask_for_double_entry(&helper(val, val), &helper(300, 20))))
            .collect();
        for (test_val, entry) in ask_entries.into_iter() {
            match entry {
                NotRelevant => {},
                OnlyOne => {panic!("wrong ask_entry (OnlyOne) for {}", test_val);},
                NeedEntry(rec_val) => {panic!("wrong ask_entry (needentry) for {} with {:?}", test_val, rec_val);}
            }
        }
    }

    #[test]
    fn not_relevant_on_non_0_above_50() {
        let finishes: Vec<u16> = (51..501)
            .into_iter()
            .map(|s| s as u16)
            .collect();
        let ask_entries: Vec<(u16, HowManyOnDouble)> = finishes
            .into_iter()
            .map(|val| (val, ask_for_double_entry(&helper(val+100, 20), &helper(val, 100))))
            .collect();
        for (test_val, entry) in ask_entries.into_iter() {
            match entry {
                NotRelevant => {},
                OnlyOne => {panic!("wrong ask_entry (OnlyOne) for {}", test_val);},
                NeedEntry(rec_val) => {panic!("wrong ask_entry (needentry) for {} with {:?}", test_val, rec_val);}
            }
        }
    }

    #[test]
    fn only_one_on_120_plus_finishes() {
        let finishes: Vec<u16> = (121..171)
            .into_iter()
            .map(|s| s as u16)
            .filter(|val| common::is_finish(*val))
            .collect();
        let ask_entries: Vec<(u16, HowManyOnDouble)> = finishes
            .into_iter()
            .map(|val| (val, ask_for_double_entry(&helper(val, val), &helper(0, 20))))
            .collect();
        for (test_val, entry) in ask_entries.into_iter() {
            match entry {
                NotRelevant => {panic!("wrong ask_entry (NotRelevant) for {}", test_val);},
                OnlyOne => {},
                NeedEntry(rec_val) => {panic!("wrong ask_entry (NeedEntry) for {} with {:?}", test_val, rec_val);}
            }
        }
    }

    #[test]
    fn only_one_on_120_plus_finishes_not_suc() {
        let finishes: Vec<u16> = (121..171)
            .into_iter()
            .map(|s| s as u16)
            .filter(|val| common::is_finish(*val))
            .collect();
        let ask_entries: Vec<(u16, HowManyOnDouble)> = finishes
            .into_iter()
            .map(|val| (val, ask_for_double_entry(&helper(val, val), &helper(10, 20))))
            .collect();
        for (test_val, entry) in ask_entries.into_iter() {
            match entry {
                NotRelevant => {panic!("wrong ask_entry (NotRelevant) for {}", test_val);},
                OnlyOne => {panic!("wrong ask_entry (OnlyOne) for {}", test_val);},
                NeedEntry(rec_val) => {if rec_val.contains(&2) || rec_val.contains(&3) {
                    panic!("wrong rec_vals (needentry) for {} with {:?}", test_val, rec_val);
                };}
            }
        }
    }
}

