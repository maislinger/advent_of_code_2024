use rustc_hash::FxHashSet as HashSet;

type Mat = ndarray::Array<u8, ndarray::Dim<[usize; 2]>>;

pub fn solve(input: &str) -> aoc_common::AocResult {
    let mat = parse_input(input)?;
    let s1 = trailhead_score(&mat);
    let s2 = trailhead_rating(&mat);

    Ok(format!("d10/01 = {}, d10/02 = {}", s1, s2))
}

fn trailhead_score(mat: &Mat) -> u64 {
    trailhead_helper(mat, false)
}

fn trailhead_rating(mat: &Mat) -> u64 {
    trailhead_helper(mat, true)
}

fn trailhead_helper(mat: &Mat, multiroutes: bool) -> u64 {
    let (height, width) = mat.dim();

    let mut result = 0;

    let mut visited = HashSet::default();
    let mut todo = Vec::new();

    for ((i, j), &v) in mat.indexed_iter() {
        if v != 0 {
            continue;
        }

        visited.clear();
        todo.push((i, j, 0));
        while let Some((i, j, v)) = todo.pop() {
            if !multiroutes && visited.contains(&(i, j)) {
                continue;
            }
            if !multiroutes {
                visited.insert((i, j));
            }

            if mat[(i, j)] == 9 {
                result += 1;
            }

            for (ni, nj) in neighbors(i, height, j, width) {
                if mat[(ni, nj)] == v + 1 {
                    todo.push((ni, nj, v + 1));
                }
            }
        }
    }

    result
}

fn neighbors(
    i: usize,
    height: usize,
    j: usize,
    width: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let i = i as i64;
    let j = j as i64;
    let height = height as i64;
    let width = width as i64;

    [(0, 1), (-1, 0), (0, -1), (1, 0)]
        .iter()
        .filter_map(move |(di, dj)| {
            let ni = i + di;
            let nj = j + dj;

            if ni >= 0 && ni < height && nj >= 0 && nj < width {
                Some((ni as usize, nj as usize))
            } else {
                None
            }
        })
}

fn parse_input(input: &str) -> Result<Mat, aoc_common::AocError> {
    let mut buffer = Vec::new();

    let mut width = 0;
    let mut height = 0;

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        height += 1;

        let mut local_width = 0;
        for c in line.chars() {
            let v = c.to_digit(10).ok_or(aoc_common::AocError::InvalidInput)?;
            buffer.push(v as u8);
            local_width += 1;
        }

        if width == 0 {
            width = local_width;
        } else if width != local_width {
            return Err(aoc_common::AocError::InvalidInput);
        }
    }

    if width == 0 || height == 0 {
        return Err(aoc_common::AocError::InvalidInput);
    }

    let result = ndarray::Array::from_shape_vec((height, width), buffer)
        .map_err(|_| aoc_common::AocError::InvalidInput)?;

    Ok(result)
}
