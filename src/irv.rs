//! Simple implementation of an instant-runoff voting system

#![warn(missing_docs)]

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

/// Result of a poll
pub enum PollResult<'a, T> {
    /// Could not finish the poll
    NoWinner,
    /// The poll results in multiple winners
    Tied(Vec<&'a T>),
    /// The poll results in one winner
    Winner(&'a T),
}

/// Determine the best item(s) using the instant-runoff voting system. This function does not
/// guarantee the winner to be the one receives the majority votes.
pub fn run_instant_runoff_voting<'a, T>(ballots: &'a [&'a [T]]) -> PollResult<'a, T>
where
    T: 'a + Eq + Hash,
{
    let mut eliminated_items: HashSet<&T> = HashSet::new();
    loop {
        // Count ballots
        let mut ballots_count: HashMap<&T, u32> = HashMap::new();
        for &vote in ballots {
            for opt in vote {
                if !eliminated_items.contains(opt) {
                    let count = ballots_count.entry(opt).or_insert(0);
                    *count += 1;
                    break;
                }
            }
        }
        // There is no vote
        if ballots_count.is_empty() {
            break PollResult::NoWinner;
        }

        // Get items with most number of ballots and items with least number of ballots
        let mut max_count = u32::MIN;
        let mut min_count = u32::MAX;
        let mut best_items = Vec::new();
        let mut worst_items = Vec::new();
        for (&k, &v) in ballots_count.iter() {
            if v > max_count {
                best_items.clear();
                max_count = v;
            }
            if v >= max_count {
                best_items.push(k);
            }
            if v < min_count {
                worst_items.clear();
                min_count = v;
            }
            if v <= min_count {
                worst_items.push(k);
            }
        }

        // Only one item received the majority of ballots
        if best_items.len() == 1 {
            break PollResult::Winner(best_items[0]);
        }
        // Tied when ballots are evenly distributed
        if max_count == min_count {
            break PollResult::Tied(best_items);
        }

        for opt in worst_items {
            eliminated_items.insert(opt);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn irv_basic_poll() {
        let vote_a = vec!["bob", "bill", "sue"];
        let vote_b = vec!["sue", "bob", "bill"];
        let vote_c = vec!["bill", "sue", "bob"];
        let vote_d = vec!["bob", "bill", "sue"];
        let vote_e = vec!["sue", "bob", "bill"];

        let votes = vec![
            vote_a.as_slice(),
            vote_b.as_slice(),
            vote_c.as_slice(),
            vote_d.as_slice(),
            vote_e.as_slice(),
        ];
        match run_instant_runoff_voting(&votes) {
            PollResult::NoWinner => unreachable!(),
            PollResult::Tied(_) => unreachable!(),
            PollResult::Winner(winner) => assert_eq!(winner, &"sue"),
        };
    }

    #[test]
    fn irv_tied_01() {
        let vote_a = vec!["bob"];
        let vote_b = vec!["sue"];

        let votes = vec![vote_a.as_slice(), vote_b.as_slice()];
        match run_instant_runoff_voting(&votes) {
            PollResult::NoWinner => unreachable!(),
            PollResult::Winner(_) => unreachable!(),
            PollResult::Tied(options) => {
                assert!(options.contains(&&"sue"));
                assert!(options.contains(&&"bob"));
            }
        };
    }

    #[test]
    fn irv_tied_02() {
        let vote_a = vec!["bob", "sue"];
        let vote_b = vec!["sue", "bob"];

        let votes = vec![vote_a.as_slice(), vote_b.as_slice()];
        match run_instant_runoff_voting(&votes) {
            PollResult::NoWinner => unreachable!(),
            PollResult::Winner(_) => unreachable!(),
            PollResult::Tied(options) => {
                assert!(options.contains(&&"sue"));
                assert!(options.contains(&&"bob"));
            }
        };
    }

    #[test]
    fn irv_no_vote() {
        let votes: Vec<&[&str]> = vec![];
        match run_instant_runoff_voting(&votes) {
            PollResult::Tied(_) => unreachable!(),
            PollResult::Winner(_) => unreachable!(),
            PollResult::NoWinner => {}
        };
    }
}
