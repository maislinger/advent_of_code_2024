use rustc_hash::FxHashSet as HashSet;

pub fn solve(input: &str) -> aoc_common::AocResult {
    let antennas = parse_input(input)?;

    let s1 = count_antinodes(&antennas);
    let s2 = count_harmonic_antinodes(&antennas);

    Ok(format!("d08/01 = {}, d08/02 = {}", s1, s2))
}

fn count_antinodes(antennas: &Antennas) -> usize {
    count_set(antennas, antinodes_iter)
}

fn count_harmonic_antinodes(antennas: &Antennas) -> usize {
    count_set(antennas, line)
}

fn count_set<F, G>(antennas: &Antennas, iter_func: F) -> usize
where
    F: Fn(Point, Point, usize, usize) -> G,
    G: Iterator<Item = Point>,
{
    let mut antinodes = HashSet::default();

    for frequency in antennas.coordinates.iter() {
        for i in 0..frequency.len() {
            for j in (i + 1)..frequency.len() {
                antinodes.extend(iter_func(
                    frequency[i],
                    frequency[j],
                    antennas.width,
                    antennas.height,
                ));
            }
        }
    }

    antinodes.len()
}

fn antinodes_iter(
    p1: Point,
    p2: Point,
    width: usize,
    height: usize,
) -> impl Iterator<Item = Point> {
    line(p1, p2, width, height).filter(move |p| {
        let d1 = p.dist_sq(p1);
        let d2 = p.dist_sq(p2);
        (4 * d1 == d2) || (4 * d2 == d1)
    })
}

fn line(p1: Point, p2: Point, width: usize, height: usize) -> impl Iterator<Item = Point> {
    let delta = p2 - p1;
    let step = delta / gcd(delta.x, delta.y);

    fn inside(p: Point, width: usize, height: usize) -> bool {
        p.x >= 0 && p.x < (width as i64) && p.y >= 0 && p.y < (height as i64)
    }

    let first_half = (0i64..)
        .map(move |d| p1 + step * d)
        .take_while(move |p| inside(*p, width, height));

    let second_half = (1i64..)
        .map(move |d| p1 - step * d)
        .take_while(move |p| inside(*p, width, height));

    first_half.chain(second_half)
}

fn parse_input(input: &str) -> Result<Antennas, aoc_common::AocError> {
    let mut height = 0;
    let mut width = 0;

    let mut result = Antennas {
        names: Vec::new(),
        coordinates: Vec::new(),
        width,
        height,
    };

    for (i, line) in input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .enumerate()
    {
        height += 1;
        let mut local_width = 0;

        for (j, c) in line.chars().enumerate() {
            local_width += 1;

            match c {
                '.' => (),
                ch => {
                    if let Some(p) = result.names.iter().position(|&ch2| ch == ch2) {
                        result.coordinates[p].push(Point::new(j as i64, -(i as i64)));
                    } else {
                        result.names.push(ch);
                        result
                            .coordinates
                            .push(vec![Point::new(j as i64, -(i as i64))]);
                    }
                }
            }
        }

        if width == 0 {
            width = local_width;
        }

        if local_width != width {
            return Err(aoc_common::AocError::InvalidInput);
        }
    }

    if height == 0 || width == 0 {
        return Err(aoc_common::AocError::InvalidInput);
    }

    for coord in result.coordinates.iter_mut().flat_map(|v| v.iter_mut()) {
        coord.y += (height - 1) as i64;
    }

    result.width = width;
    result.height = height;

    Ok(result)
}

#[derive(Debug)]
struct Antennas {
    names: Vec<char>,
    coordinates: Vec<Vec<Point>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn abs_sq(&self) -> i64 {
        self.x * self.x + self.y * self.y
    }

    fn dist_sq(&self, p: Point) -> i64 {
        (*self - p).abs_sq()
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<i64> for Point {
    type Output = Point;

    fn mul(self, scalar: i64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Div<i64> for Point {
    type Output = Point;

    fn div(self, scalar: i64) -> Point {
        Point {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 1;
    }

    let mut a = a.abs();
    let mut b = b.abs();

    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}
