use std::{
    fmt::Display,
    ops::{AddAssign, SubAssign},
};

use num_traits::Num;

pub struct RBSet<T> {
    // [start, end]
    ranges: Vec<(T, T)>,
}

impl<T: Num + PartialOrd + AddAssign + SubAssign + Copy> RBSet<T> {
    pub fn new() -> Self {
        RBSet { ranges: Vec::new() }
    }

    pub fn insert(&mut self, value: T) {
        if self.ranges.is_empty() {
            self.ranges.push((value, value));
        } else {
            let mut insert_pos = None;
            let mut check_pos = None;
            let ranges_len = self.ranges.len();
            for (idx, (start, end)) in self.ranges.iter_mut().enumerate() {
                if value >= *start {
                    if value <= *end {
                        // already in existing range
                        return;
                    } else if value == *end + T::one() {
                        // extend existing range by one
                        *end += T::one();
                        check_pos = Some(idx);
                        break;
                    } else if idx == ranges_len - 1 {
                        insert_pos = Some(ranges_len);
                    }
                } else if value == *start - T::one() {
                    *start -= T::one();
                    if idx > 0 {
                        check_pos = Some(idx - 1);
                    }
                    break;
                } else {
                    insert_pos = Some(idx);
                    break;
                }
            }
            // create new range, preserving sorted order
            if let Some(insert_pos) = insert_pos {
                self.ranges.insert(insert_pos, (value, value));
            }
            // check if two ranges can be combined back into one
            if let Some(check_pos) = check_pos {
                if self.ranges.len() <= 1 || check_pos >= self.ranges.len() - 1 {
                    return;
                }
                let next_range = check_pos + 1;
                if self.ranges[check_pos].1 + T::one() == self.ranges[next_range].0 {
                    self.ranges[check_pos].1 = self.ranges[next_range].1;
                    self.ranges.remove(next_range);
                }
            }
        }
    }

    pub fn remove(&mut self, value: &T) {
        let mut add_range = None;
        for (idx, (start, end)) in self.ranges.iter_mut().enumerate() {
            if *value == *start {
                if *value == *end {
                    // found [value, value] range, just remove it
                    self.ranges.remove(idx);
                } else {
                    // found [value, value+x], x>0 range, adjust start
                    *start += T::one();
                }
                return;
            } else if *value == *end {
                // found [value-x, value), x>0, adjust end
                *end -= T::one();
                return;
            } else if *value > *start && *value < *end {
                // found [value, value+x), x > 0, split into two ranges
                add_range = Some((idx + 1, *end));
                *end = *value - T::one();
            }
        }
        if let Some((idx, old_end)) = add_range {
            self.ranges.insert(idx, (*value + T::one(), old_end));
        }
    }

    pub fn clear(&mut self) {
        self.ranges.clear();
    }

    pub fn contains(&self, value: &T) -> bool {
        for (start, end) in &self.ranges {
            if *value >= *start && *value <= *end {
                return true;
            }
        }
        false
    }

    pub fn ranges(&self) -> &[(T, T)] {
        &self.ranges
    }
}

impl<T: PartialOrd> Default for RBSet<T> {
    fn default() -> Self {
        Self { ranges: Vec::new() }
    }
}

impl<T: Display> Display for RBSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for range in self.ranges.iter() {
            write!(f, "{}..={} ", range.0, range.1)?;
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continuous_add() {
        let mut set = RBSet::new();
        set.insert(0);
        set.insert(1);
        set.insert(2);
        set.insert(3);
        assert_eq!(set.ranges.len(), 1);
        assert_eq!(set.ranges[0], (0, 3));
    }

    #[test]
    fn discontinuous_add() {
        let mut set = RBSet::new();
        set.insert(0);
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(7);
        set.insert(8);
        set.insert(10);
        assert_eq!(set.ranges.len(), 3);
        assert_eq!(set.ranges[0], (0, 3));
        assert_eq!(set.ranges[1], (7, 8));
        assert_eq!(set.ranges[2], (10, 10));
    }
}
