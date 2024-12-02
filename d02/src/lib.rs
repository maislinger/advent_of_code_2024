pub fn solve(input: &str) -> aoc_common::AocResult {
    let mut count1 = 0;
    let mut count2 = 0;

    let mut buffer = Vec::new();

    for line in input.lines().map(|l| l.trim()) {
        if line.is_empty() {
            continue;
        }

        let (save1, save2) = save(line, &mut buffer)?;
        if save1 {
            count1 += 1;
        }

        if save2 {
            count2 += 1;
        }
    }

    Ok(format!("d02/01 = {}, d02/02 = {}", count1, count2))
}

fn save(line: &str, buffer: &mut Vec<i64>) -> Result<(bool, bool), aoc_common::AocError> {
    fill_buffer(buffer, line)?;

    if save_inner(buffer, Direction::Increasing, None) {
        return Ok((true, true));
    }

    if save_inner(buffer, Direction::Decreasing, None) {
        return Ok((true, true));
    }

    for i in 0..buffer.len() {
        if save_inner(buffer, Direction::Increasing, Some(i)) {
            return Ok((false, true));
        }

        if save_inner(buffer, Direction::Decreasing, Some(i)) {
            return Ok((false, true));
        }
    }

    Ok((false, false))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
}

fn fill_buffer(buffer: &mut Vec<i64>, line: &str) -> Result<(), aoc_common::AocError> {
    buffer.clear();

    for s in line.split_whitespace() {
        if s.is_empty() {
            continue;
        }

        let n: i64 = s.parse()?;
        buffer.push(n);
    }

    Ok(())
}

fn save_inner(values: &[i64], direction: Direction, skip: Option<usize>) -> bool {
    let mut previous = None;
    let increasing = direction == Direction::Increasing;

    for (i, v) in values.iter().enumerate() {
        if let Some(j) = skip {
            if i == j {
                continue;
            }
        }

        let p = match previous {
            Some(p) => p,
            None => {
                previous = Some(v);
                continue;
            }
        };

        let delta = v - p;

        if delta == 0 || delta.abs() > 3 {
            return false;
        }

        if increasing != (delta > 0) {
            return false;
        }

        previous = Some(v);
    }

    true
}
