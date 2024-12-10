pub fn solve(input: &str) -> aoc_common::AocResult {
    let mut s1 = 0;
    let mut s2 = 0;

    let mut operands = Vec::new();

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let (r1, r2) = check_line(line, &mut operands)?;
        s1 += r1;
        s2 += r2;
    }

    Ok(format!("d07/01 = {}, d07/02 = {}", s1, s2))
}

// (delta 1, delta 2)
fn check_line(
    line: &str,
    operands: &mut Vec<(u64, bool)>,
) -> Result<(u64, u64), aoc_common::AocError> {
    operands.clear();

    let [result_s, operands_s] =
        aoc_common::split_to_array::<2>(line, ": ").ok_or(aoc_common::AocError::InvalidInput)?;

    for operand_s in operands_s
        .split_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        operands.push((operand_s.parse::<u64>()?, false));
    }

    for i in 0..operands.len() {
        if i == 0 {
            operands[i].1 = operands[i].0 == 0;
            continue;
        }

        operands[i].1 = operands[i - 1].1;
        operands[i].1 |= operands[i].0 == 0;
    }

    let result = result_s.parse::<u64>()?;

    let (r, used_concat) = check_result(0, 0, false, result, operands);
    if r == result && !used_concat {
        Ok((result, result))
    } else if r == result && used_concat {
        Ok((0, result))
    } else {
        Ok((0, 0))
    }
}

fn check_result(
    depth: usize,
    accumulator: u64,
    used_concat: bool,
    target: u64,
    operands: &[(u64, bool)],
) -> (u64, bool) {
    // 305235895
    if operands.is_empty() {
        return (accumulator, used_concat);
    }

    if accumulator > target {
        return (target.wrapping_add(1), false);
    }

    if depth == 0 {
        return check_result(depth + 1, operands[0].0, false, target, &operands[1..]);
    }

    let v1 = accumulator.wrapping_add(operands[0].0);
    let (r1, c1) = check_result(depth + 1, v1, used_concat, target, &operands[1..]);
    if r1 == target && !c1 {
        return (r1, c1);
    }

    let v2 = accumulator.wrapping_mul(operands[0].0);
    let (r2, c2) = check_result(depth + 1, v2, used_concat, target, &operands[1..]);
    if r2 == target && !c2 {
        return (r2, c2);
    }

    let v3 = concatenate(accumulator, operands[0].0);
    let (r3, c3) = check_result(depth + 1, v3, true, target, &operands[1..]);

    if r1 == target {
        (r1, c1)
    } else if r2 == target {
        (r2, c2)
    } else {
        (r3, c3)
    }
}

fn concatenate(a: u64, b: u64) -> u64 {
    let mut a2 = a;
    let mut b2 = b;
    while b2 > 0 {
        a2 = a2.wrapping_mul(10);
        b2 /= 10;
    }
    a2 + b
}
