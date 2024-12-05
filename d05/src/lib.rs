use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

use aoc_common::split_to_array;

type Rules = HashMap<usize, HashSet<usize>>;

pub fn solve(input: &str) -> aoc_common::AocResult {
    let (rules, mut updates) = parse_input(input)?;

    let mut s1 = 0;
    let mut s2 = 0;
    for u in updates.iter_mut() {
        if u.valid(&rules) {
            s1 += u.center();
        } else {
            u.sort(&rules);
            debug_assert!(u.valid(&rules));
            s2 += u.center();
        }
    }

    Ok(format!("d05/01 = {}, d05/02 = {}", s1, s2))
}

fn parse_input(input: &str) -> Result<(Rules, Updates), aoc_common::AocError> {
    let mut rules = Rules::default();
    let mut update = Updates::new();

    let input = input.trim();
    let mut line_iter = input.lines().map(|l| l.trim());

    for line in line_iter.by_ref() {
        if line.is_empty() {
            break;
        }

        let nums = split_to_array::<2>(line, "|").ok_or(aoc_common::AocError::InvalidInput)?;

        let n1 = nums[0].parse::<usize>()?;
        let n2 = nums[1].parse::<usize>()?;

        rules.entry(n2).or_insert_with(HashSet::default);
        rules.entry(n2).or_default().insert(n1);
    }

    for line in line_iter {
        if line.is_empty() {
            continue;
        }

        let vals = {
            let vals: Result<Vec<usize>, _> = line
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse())
                .collect();
            vals?
        };

        if !vals.is_empty() {
            update.push_vec(vals);
        }
    }

    Ok((rules, update))
}

#[derive(Debug)]
struct Update {
    data: Vec<usize>,
}

impl Update {
    fn new(data: Vec<usize>) -> Self {
        Self { data }
    }

    fn valid(&self, rules: &Rules) -> bool {
        let mut previous = HashSet::default();

        for &v in self.data.iter().rev() {
            let Some(r) = rules.get(&v) else {
                previous.insert(v);
                continue;
            };

            if !r.is_disjoint(&previous) {
                return false;
            }

            previous.insert(v);
        }

        true
    }

    fn center(&self) -> usize {
        self.data[self.data.len() / 2]
    }

    fn sort(&mut self, rules: &Rules) {
        bubble_sort(&mut self.data, |a, b| {
            let b_before_a = if let Some(r) = rules.get(a) {
                r.contains(b)
            } else {
                false
            };

            let a_before_b = if let Some(r) = rules.get(b) {
                r.contains(a)
            } else {
                false
            };

            if b_before_a {
                std::cmp::Ordering::Greater
            } else if a_before_b {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
    }
}

#[derive(Debug)]
struct Updates {
    data: Vec<Update>,
}

impl Updates {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push_vec(&mut self, update: Vec<usize>) {
        self.data.push(Update::new(update));
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Update> {
        self.data.iter_mut()
    }
}

fn bubble_sort<T, F>(values: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    if values.len() <= 1 {
        return;
    }

    for _ in 0..values.len() {
        let mut swapped = false;
        for i in 0..(values.len() - 1) {
            if compare(&values[i], &values[i + 1]) == std::cmp::Ordering::Greater {
                values.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
