pub fn solve(input: &str) -> aoc_common::AocResult {
    let board = Board::from_input(input)?;
    let count_xmas = board.count_xmas();
    let count_mas = board.count_mas();

    Ok(format!("d04/01 = {}, d04/02 = {}", count_xmas, count_mas))
}

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    data: Vec<BoardCell>,
}

impl Board {
    fn from_input(input: &str) -> Result<Self, aoc_common::AocError> {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();

        for line in input.lines().map(|l| l.trim()) {
            if line.is_empty() {
                continue;
            }

            height += 1;
            let mut local_width = 0;
            for c in line.chars() {
                local_width += 1;

                let newval = match c {
                    'X' => BoardCell::X,
                    'M' => BoardCell::M,
                    'A' => BoardCell::A,
                    'S' => BoardCell::S,
                    _ => BoardCell::None,
                };
                data.push(newval);
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

        Ok(Self {
            width,
            height,
            data,
        })
    }

    fn count_xmas(&self) -> usize {
        let mut count = 0;
        for index in 0..self.data.len() {
            count += self.count_xmas_by_index(index);
        }
        count
    }

    fn count_xmas_by_index(&self, index: usize) -> usize {
        if self.data[index] != BoardCell::X {
            return 0;
        }

        let xmas = [BoardCell::X, BoardCell::M, BoardCell::A, BoardCell::S];

        let i = (index / self.width) as i64;
        let j = (index % self.width) as i64;

        let mut count = 0;
        for di in -1..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 {
                    continue;
                }

                let ray = self.ray::<4>(i, di, j, dj);
                if ray == xmas {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_mas(&self) -> usize {
        let mut count = 0;
        for index in 0..self.data.len() {
            count += self.count_mas_by_index(index);
        }

        count
    }

    fn count_mas_by_index(&self, index: usize) -> usize {
        if self.data[index] != BoardCell::A {
            return 0;
        }

        let i = (index / self.width) as i64;
        let j = (index % self.width) as i64;

        let mas = [BoardCell::M, BoardCell::A, BoardCell::S];
        let mut count = 0;

        for direction in [
            Direction::Right,
            Direction::Up,
            Direction::Left,
            Direction::Down,
        ] {
            let rays = self.cross_rays(i, j, direction);
            if rays[0] == mas && rays[1] == mas {
                count += 1;
            }
        }

        count
    }

    fn ray<const N: usize>(&self, i: i64, di: i64, j: i64, dj: i64) -> [BoardCell; N] {
        let mut result = [BoardCell::None; N];

        for (i, index2) in self.ray_indices(i, di, j, dj).enumerate().take(N) {
            result[i] = self.data[index2];
        }

        result
    }

    fn ray_indices(&self, i: i64, di: i64, j: i64, dj: i64) -> impl Iterator<Item = usize> + '_ {
        (0..)
            .map(move |n| (i + di * n, j + dj * n))
            .take_while(|&(ni, nj)| ni >= 0 && nj >= 0)
            .take_while(|&(ni, nj)| ni < (self.height as i64) && nj < (self.width as i64))
            .map(move |(ni, nj)| (ni as usize) * self.width + (nj as usize))
    }

    fn cross_rays(&self, i: i64, j: i64, direction: Direction) -> [[BoardCell; 3]; 2] {
        let mut result = [[BoardCell::None; 3]; 2];

        let (i1, i2) = match direction {
            Direction::Right | Direction::Left => (i + 1, i - 1),
            Direction::Up => (i + 1, i + 1),
            Direction::Down => (i - 1, i - 1),
        };

        let (j1, j2) = match direction {
            Direction::Up | Direction::Down => (j - 1, j + 1),
            Direction::Left => (j + 1, j + 1),
            Direction::Right => (j - 1, j - 1),
        };

        let di1 = i - i1;
        let dj1 = j - j1;
        let di2 = i - i2;
        let dj2 = j - j2;

        result[0] = self.ray::<3>(i1, di1, j1, dj1);
        result[1] = self.ray::<3>(i2, di2, j2, dj2);
        result
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum BoardCell {
    None,
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}
