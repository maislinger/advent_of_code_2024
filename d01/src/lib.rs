pub fn solve(input: &str) -> aoc_common::AocResult {
    let (mut left, mut right) = parse_input(input)?;

    left.sort_unstable();
    right.sort_unstable();

    let mut s1 = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        s1 += (l - r).abs();
    }

    let mut s2 = 0;
    for l in left.iter() {
        let Some(indices) = find(*l, &right) else {
            continue;
        };

        s2 += l * (indices.1 - indices.0 + 1) as i64;
    }

    Ok(format!("d01/01 = {}, d01/02 = {}", s1, s2))
}

fn parse_input(input: &str) -> Result<(Vec<i64>, Vec<i64>), aoc_common::AocError> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let [s_left, s_right] = aoc_common::split_whitespace_to_array(line)
            .ok_or(aoc_common::AocError::InvalidInput)?;

        let v_left = s_left.parse::<i64>()?;
        let v_right = s_right.parse::<i64>()?;

        left.push(v_left);
        right.push(v_right);
    }

    if right.len() != left.len() || left.is_empty() {
        return Err(aoc_common::AocError::InvalidInput);
    }

    Ok((left, right))
}

// assumes that list is sorted
fn find(value: i64, list: &[i64]) -> Option<(usize, usize)> {
    let lower = find_first_last(value, list, true);
    let upper = find_first_last(value, list, false);

    match (lower, upper) {
        (Some(l), Some(u)) => Some((l, u)),
        _ => None,
    }
}

// assumes that list is sorted
fn find_first_last(value: i64, list: &[i64], on_equal_down: bool) -> Option<usize> {
    if list.is_empty() {
        return None;
    }

    let mut i0 = 0;
    let mut i2 = list.len() - 1;
    let mut i1 = (i0 + i2) / 2;

    loop {
        if list[i0] > value || list[i2] < value {
            return None;
        }

        if list[i1] < value {
            i0 = i1;
        } else if list[i1] > value {
            i2 = i1;
        } else if on_equal_down {
            i2 = i1;
        } else {
            i0 = i1;
        }

        if on_equal_down && list[i0] == value {
            return Some(i0);
        } else if !on_equal_down && list[i2] == value {
            return Some(i2);
        }

        i1 = (i0 + i2) / 2;

        if i0 == i1 {
            break;
        }
    }

    if on_equal_down {
        for i in i0..=i2 {
            if list[i] == value {
                return Some(i);
            }
        }
    } else {
        for i in (i0..=i2).rev() {
            if list[i] == value {
                return Some(i);
            }
        }
    }
    None
}
