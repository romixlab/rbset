#![no_main]

use libfuzzer_sys::fuzz_target;
use num_traits::Num;
use rbset::RBSet;
use std::{collections::HashSet, ops::Add};

#[derive(Clone, Debug, PartialEq, arbitrary::Arbitrary)]
// #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Action {
    Insert(u8),
    Remove(u8),
    Check(u8),
    CheckOrder,
}

fuzz_target!(|actions: Vec<Action>| {
    let mut set = RBSet::new();
    let mut hash_set = HashSet::new();
    let mut actions = actions;
    actions.dedup_by(|a, b| *a == Action::CheckOrder && a == b);
    for (idx, action) in actions.iter().enumerate() {
        match action {
            Action::Insert(value) => {
                set.insert(*value);
                // print!("{action} ");
                // print_ranges(&set.ranges);
                hash_set.insert(*value);
            }
            Action::Remove(value) => {
                set.remove(value);
                // print!("{action} ");
                // print_ranges(&set.ranges);
                hash_set.remove(value);
            }
            Action::Check(value) => {
                // println!("{action}");
                if set.contains(value) != hash_set.contains(value) {
                    print_actions(&actions);
                    panic!("Check at step: {idx} for {value} failed");
                }
                assert_eq!(set.len(), hash_set.len());
            }
            Action::CheckOrder => {
                // println!("{action}");
                let mut arr = Vec::new();
                for value in &hash_set {
                    arr.push(*value);
                }
                arr.sort();
                let expected_ranges = consecutive_slices(&arr);
                if expected_ranges.len() != set.ranges().len() {
                    print!("got ranges: ");
                    print_ranges(set.ranges());
                    print!("expected  : ");
                    print_ranges(&expected_ranges);
                    panic!(
                        "expected {} ranges, got {}",
                        expected_ranges.len(),
                        set.ranges().len()
                    );
                }
                for (expected_range, actual_range) in
                    expected_ranges.iter().zip(set.ranges().iter())
                {
                    let start_ok = expected_range.0 == actual_range.0;
                    let end_ok = expected_range.1 == actual_range.1;
                    if !start_ok || !end_ok {
                        print!("got ranges: ");
                        print_ranges(set.ranges());
                        print!("expected  : ");
                        print_ranges(&expected_ranges);
                        panic!("ranges don't match");
                    }
                }
            }
        }
    }
});

// adapted from: https://stackoverflow.com/questions/50380352/how-can-i-group-consecutive-integers-in-a-vector-in-rust
fn consecutive_slices<T: Num + Add + Copy>(data: &[T]) -> Vec<(T, T)> {
    let mut slice_start = 0;
    let mut result = Vec::new();
    for i in 1..data.len() {
        if data[i - 1] + T::one() != data[i] {
            result.push((data[slice_start], data[i - 1]));
            slice_start = i;
        }
    }
    if !data.is_empty() {
        result.push((data[slice_start], data[data.len() - 1]));
    }
    result
}

fn print_actions(actions: &[Action]) {
    for (idx, action) in actions.iter().enumerate() {
        println!("{idx}: {action}");
    }
}

impl core::fmt::Display for Action {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Action::Insert(value) => write!(f, "I{value}"),
            Action::Remove(value) => write!(f, "R{value}"),
            Action::Check(value) => write!(f, "C{value}"),
            Action::CheckOrder => write!(f, "COrd"),
        }
    }
}

fn print_ranges<T: core::fmt::Display>(ranges: &[(T, T)]) {
    for range in ranges.iter() {
        print!("{}..={} ", range.0, range.1);
    }
    println!();
}
