use rustc_hash::FxHashMap as HashMap;

pub fn solve(input: &str) -> aoc_common::AocResult {
    // let input = "125 17";

    let mut stones = parse_input(input)?;
    let mut buffer = HashMap::default();

    for _ in 0..25 {
        blink_stones(&mut buffer, &mut stones);
        std::mem::swap(&mut stones, &mut buffer);
    }
    let s1 = stones.values().sum::<u64>();

    for _ in 0..50 {
        blink_stones(&mut buffer, &mut stones);
        std::mem::swap(&mut stones, &mut buffer);
    }
    let s2 = stones.values().sum::<u64>();

    Ok(format!("d11/01 = {}, d11/02 = {}", s1, s2))
}

fn blink_stones(target: &mut HashMap<u64, u64>, stones: &mut HashMap<u64, u64>) {
    target.clear();

    fn insert(stone: u64, count: u64, stones: &mut HashMap<u64, u64>) {
        stones
            .entry(stone)
            .and_modify(|c| *c += count)
            .or_insert(count);
    }

    for (&stone, &count) in stones.iter() {
        let (a, b) = blink_stone(stone);

        insert(a, count, target);
        if let Some(b) = b {
            insert(b, count, target);
        }
    }
}

fn blink_stone(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }

    let digits = n_digits(stone);
    if digits % 2 == 0 {
        let (a, b) = split(stone, digits);
        (a, Some(b))
    } else {
        (stone * 2024, None)
    }
}

fn n_digits(n: u64) -> u8 {
    // 18446744073709551615

    match n {
        0..10 => 1,
        10..100 => 2,
        100..1000 => 3,
        1000..10000 => 4,
        10000..100000 => 5,
        100000..1000000 => 6,
        1000000..10000000 => 7,
        10000000..100000000 => 8,
        100000000..1000000000 => 9,
        1000000000..10000000000 => 10,
        10000000000..100000000000 => 11,
        100000000000..1000000000000 => 12,
        1000000000000..10000000000000 => 13,
        10000000000000..100000000000000 => 14,
        100000000000000..1000000000000000 => 15,
        1000000000000000..10000000000000000 => 16,
        10000000000000000..100000000000000000 => 17,
        100000000000000000..1000000000000000000 => 18,
        1000000000000000000..10000000000000000000 => 19,
        _ => 20,
    }
}

fn split(n: u64, n_digits: u8) -> (u64, u64) {
    fn split_inner(n: u64, factor: u64) -> (u64, u64) {
        let a = n / factor;
        let b = n - a * factor;
        (a, b)
    }

    let factor = match n_digits {
        2 => 10,
        4 => 100,
        6 => 1000,
        8 => 10000,
        10 => 100000,
        12 => 1000000,
        14 => 10000000,
        16 => 100000000,
        18 => 1000000000,
        20 => 10000000000,
        _ => 1,
    };

    split_inner(n, factor)
}

fn parse_input(input: &str) -> Result<HashMap<u64, u64>, aoc_common::AocError> {
    let mut result = HashMap::default();

    for ns in input.split_whitespace() {
        let n = ns.parse::<u64>()?;
        result.insert(n, 1);
    }

    Ok(result)
}
