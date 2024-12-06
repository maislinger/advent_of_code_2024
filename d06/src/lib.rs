use rayon::prelude::*;

pub fn solve(input: &str) -> aoc_common::AocResult {
    let (player, mut field) = parse_input(input)?;
    let start_position = player.position;

    let field2 = field.clone();
    if sweep(player, &mut field) {
        // loop in field
        return Err(aoc_common::AocError::InvalidInput);
    }
    let s1 = field.count_visited();

    let visited_fields = field.iter_visited().collect::<Vec<_>>();

    let n_threads = 16;

    let s2: usize = (0..n_threads)
        .par_bridge()
        .map(|thread_index| {
            let mut field2 = field2.clone();
            let mut s2 = 0;
            for index in visited_fields.iter().skip(thread_index).step_by(n_threads) {
                let i = index / field.width;
                let j = index % field.width;
                if (i, j) == start_position {
                    continue;
                }
                field2.data[i * field2.width + j] = Tile::Blocked as u8;
                field2.bitset_reset();
                if sweep(player, &mut field2) {
                    s2 += 1;
                }
                field2.data[i * field2.width + j] = 0;
            }
            s2
        })
        .sum();

    Ok(format!("d06/01 = {}, d06/02 = {}", s1, s2))
}

// 0 1 2 3       4     5  6    7
// x x x blocked right up left down

#[derive(Debug, Clone)]
struct Field {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Field {
    fn position_blocked(&self, ij: (usize, usize)) -> bool {
        let (i, j) = ij;
        let index = i * self.width + j;
        (self.data[index] & (Tile::Blocked as u8)) > 0
    }

    fn bitset_add(&mut self, p: &Player) {
        let index = p.position.0 * self.width + p.position.1;
        self.data[index] |= p.direction as u8;
    }

    fn bitset_contains(&self, p: &Player) -> bool {
        let index = p.position.0 * self.width + p.position.1;
        (self.data[index] & (p.direction as u8)) > 0
    }

    // fn bitset_contains_position(&self, ij: (usize, usize)) -> bool {
    //     let (i, j) = ij;
    //     let index = i * self.width + j;
    //     self.data[index] & ALL_DIRECTIONS > 0
    // }

    fn bitset_reset(&mut self) {
        for d in self.data.iter_mut() {
            *d &= !ALL_DIRECTIONS;
        }
    }

    fn count_visited(&self) -> usize {
        self.iter_visited().count()
    }

    fn iter_visited(&self) -> impl Iterator<Item = usize> + '_ {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, d)| (*d & ALL_DIRECTIONS) > 0)
            .map(|(i, _)| i)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Tile {
    Free = 0,
    Blocked = 1 << 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    Right = 1 << 3,
    Up = 1 << 2,
    Left = 1 << 1,
    Down = 1 << 0,
}

impl Direction {
    fn turned_right(self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }
}

const ALL_DIRECTIONS: u8 = {
    let mut bits = 0;
    bits |= Direction::Right as u8;
    bits |= Direction::Up as u8;
    bits |= Direction::Left as u8;
    bits |= Direction::Down as u8;
    bits
};

#[derive(Debug, Clone, Copy)]
struct Player {
    position: (usize, usize),
    direction: Direction,
}

fn parse_input(input: &str) -> Result<(Player, Field), aoc_common::AocError> {
    let input = input.trim();

    let mut width = 0;
    let mut height = 0;
    let mut player = None;
    let mut data = Vec::new();

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
                '#' => data.push(Tile::Blocked as u8),
                '.' => data.push(Tile::Free as u8),
                '^' => {
                    player = Some(Player {
                        position: (i, j),
                        direction: Direction::Up,
                    });
                    data.push(Tile::Free as u8);
                }
                _ => return Err(aoc_common::AocError::InvalidInput),
            }
        }

        if width != 0 && width != local_width {
            return Err(aoc_common::AocError::InvalidInput);
        }
        width = local_width;
    }

    if width == 0 || height == 0 || player.is_none() || data.len() != width * height {
        return Err(aoc_common::AocError::InvalidInput);
    }

    Ok((
        player.unwrap(),
        Field {
            data,
            width,
            height,
        },
    ))
}

// true -> still inside, false -> outside
fn step(player: &mut Player, field: &mut Field) -> bool {
    let max_j = field.width - 1;
    let max_i = field.height - 1;

    for _ in 0..4 {
        let mut invalid_move = false;
        invalid_move |= (player.direction == Direction::Left) && (player.position.1 == 0);
        invalid_move |= (player.direction == Direction::Right) && (player.position.1 >= max_j);
        invalid_move |= (player.direction == Direction::Up) && (player.position.0 == 0);
        invalid_move |= (player.direction == Direction::Down) && (player.position.0 >= max_i);
        if invalid_move {
            return false;
        }

        let (i, j) = player.position;
        let candidate = match player.direction {
            Direction::Right => (i, j + 1),
            Direction::Up => (i - 1, j),
            Direction::Left => (i, j - 1),
            Direction::Down => (i + 1, j),
        };

        if field.position_blocked(candidate) {
            player.direction = player.direction.turned_right();
        } else {
            player.position = candidate;
            return true;
        }
    }

    false
}

// true -> loop, false -> outside
fn sweep(player_start: Player, field: &mut Field) -> bool {
    let mut player = player_start;
    loop {
        field.bitset_add(&player);
        if !step(&mut player, field) {
            return false;
        }

        if field.bitset_contains(&player) {
            return true;
        }
    }
}
