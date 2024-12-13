use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

type Mat = nalgebra::OMatrix<u8, nalgebra::Dyn, nalgebra::Dyn>;

pub fn solve(input: &str) -> aoc_common::AocResult {
    // let input = "OOOOO
    // OXOXO
    // OOOOO
    // OXOXO
    // OOOOO";

    let mat = parse_input(input)?;

    let (s1, s2) = fence_price(&mat);

    Ok(format!("d12/01 = {}, d12/02 = {}", s1, s2))
}

fn fence_price(mat: &Mat) -> (u64, u64) {
    let mut result1 = 0;
    let mut result2 = 0;

    let mut visited = HashMap::default();
    let mut cluster = HashSet::default();
    let mut todo = Vec::new();

    let height = mat.nrows();
    let width = mat.ncols();

    for (i, j, v) in (0..width).flat_map(|j| (0..height).map(move |i| (i, j, mat[(i, j)]))) {
        if visited.contains_key(&(i, j)) {
            continue;
        }

        todo.clear();
        cluster.clear();
        todo.push((i, j));

        while let Some((i, j)) = todo.pop() {
            if visited.contains_key(&(i, j)) {
                continue;
            }

            cluster.insert((i, j));
            visited.insert((i, j), v);

            for (ni, nj) in neighbors(i, j, mat.ncols(), mat.nrows()) {
                if visited.contains_key(&(ni, nj)) || mat[(ni, nj)] != v {
                    continue;
                }

                todo.push((ni, nj));
            }
        }

        let mut field_count = 0;
        let mut fence_count = 0;
        for &(i, j) in cluster.iter() {
            field_count += 1;
            fence_count += 4;
            for (ni, nj) in neighbors(i, j, mat.ncols(), mat.nrows()) {
                if cluster.contains(&(ni, nj)) {
                    fence_count -= 1;
                }
            }
        }
        result1 += field_count * fence_count;
        result2 += field_count * sides(&cluster);
    }

    (result1, result2)
}

fn neighbors(
    i: usize,
    j: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let i = i as i64;
    let j = j as i64;
    let h = height as i64;
    let w = width as i64;

    [(0, 1), (-1, 0), (0, -1), (1, 0)]
        .iter()
        .filter_map(move |(di, dj)| {
            let ni = i + di;
            let nj = j + dj;

            if ni < 0 || nj < 0 || ni >= h || nj >= w {
                None
            } else {
                Some((ni as usize, nj as usize))
            }
        })
}

fn sides(cluster: &HashSet<(usize, usize)>) -> u64 {
    if cluster.is_empty() {
        return 0;
    }

    fn is_border_up(i: usize, j: usize, c: &HashSet<(usize, usize)>) -> bool {
        if i == 0 {
            return c.contains(&(i, j));
        }

        c.contains(&(i, j)) && !c.contains(&(i - 1, j))
    }

    fn is_border_down(i: usize, j: usize, c: &HashSet<(usize, usize)>) -> bool {
        c.contains(&(i, j)) && !c.contains(&(i + 1, j))
    }

    fn is_border_left(i: usize, j: usize, c: &HashSet<(usize, usize)>) -> bool {
        if j == 0 {
            return c.contains(&(i, j));
        }

        c.contains(&(i, j)) && !c.contains(&(i, j - 1))
    }

    fn is_border_right(i: usize, j: usize, c: &HashSet<(usize, usize)>) -> bool {
        c.contains(&(i, j)) && !c.contains(&(i, j + 1))
    }

    let min_i = *cluster.iter().map(|(i, _)| i).min().unwrap();
    let max_i = *cluster.iter().map(|(i, _)| i).max().unwrap();
    let min_j = *cluster.iter().map(|(_, j)| j).min().unwrap();
    let max_j = *cluster.iter().map(|(_, j)| j).max().unwrap();

    let mut result = 0;

    // up
    for i in min_i..=max_i {
        let mut counting = false;
        for j in min_j..=max_j {
            let b = is_border_up(i, j, cluster);
            if b && !counting {
                counting = true;
                result += 1;
            } else if !b {
                counting = false;
            }
        }
    }

    // down
    for i in min_i..=max_i {
        let mut counting = false;
        for j in min_j..=max_j {
            let b = is_border_down(i, j, cluster);
            if b && !counting {
                counting = true;
                result += 1;
            } else if !b {
                counting = false;
            }
        }
    }

    // left
    for j in min_j..=max_j {
        let mut counting = false;
        for i in min_i..=max_i {
            let b = is_border_left(i, j, cluster);
            if b && !counting {
                counting = true;
                result += 1;
            } else if !b {
                counting = false;
            }
        }
    }

    // right
    for j in min_j..=max_j {
        let mut counting = false;
        for i in min_i..=max_i {
            let b = is_border_right(i, j, cluster);
            if b && !counting {
                counting = true;
                result += 1;
            } else if !b {
                counting = false;
            }
        }
    }

    result
}

fn parse_input(input: &str) -> Result<Mat, aoc_common::AocError> {
    let mut height = 0;
    let mut width = 0;

    let mut buffer = Vec::new();
    let mut min = u8::MAX;

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        height += 1;
        let mut local_width = 0;

        for c in line.chars() {
            local_width += 1;
            let v = c as u8;
            buffer.push(v);
            min = min.min(v);
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

    let result = Mat::from_vec(height, width, buffer);
    Ok(result)
}
