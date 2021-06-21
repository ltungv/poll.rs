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

/// Determine the best option(s) using the instant-runoff voting system. This function does not
/// guarantee the winner to be the one receives the majority votes.
pub fn run_instant_runoff_voting<'a, T>(votes: &'a [&'a [T]]) -> PollResult<'a, T>
where
    T: 'a + Eq + Hash,
    // &'a VoteIter: IntoIterator<Item = &'a OptionIter>,
    // &'a OptionIter: 'a + IntoIterator<Item = &'a T>,
{
    let mut eliminated_options: HashSet<&T> = HashSet::new();
    loop {
        // Count votes
        let mut votes_count: HashMap<&T, u32> = HashMap::new();
        for &vote in votes {
            for opt in vote {
                if !eliminated_options.contains(opt) {
                    let count = votes_count.entry(opt).or_insert(0);
                    *count += 1;
                    break;
                }
            }
        }
        // There is no vote
        if votes_count.is_empty() {
            break PollResult::NoWinner;
        }

        // Get options with most number of votes and options with least number of votes
        let mut max_count = u32::MIN;
        let mut min_count = u32::MAX;
        let mut best_options = Vec::new();
        let mut worst_options = Vec::new();
        for (&k, &v) in votes_count.iter() {
            if v > max_count {
                best_options.clear();
                max_count = v;
            }
            if v >= max_count {
                best_options.push(k);
            }
            if v < min_count {
                worst_options.clear();
                min_count = v;
            }
            if v <= min_count {
                worst_options.push(k);
            }
        }

        // Only one option received the majority of votes
        if best_options.len() == 1 {
            break PollResult::Winner(best_options[0]);
        }
        // Tied when votes are evenly distributed
        if max_count == min_count {
            break PollResult::Tied(best_options);
        }

        for opt in worst_options {
            eliminated_options.insert(opt);
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
