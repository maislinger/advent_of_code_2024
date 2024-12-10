pub fn solve(input: &str) -> aoc_common::AocResult {
    // let input = "2333133121414131402";

    let mut blocks1 = Blocks::from_input(input)?;
    let mut blocks2 = blocks1.clone();

    blocks1.compress_fragmented();
    blocks2.compress_unfragmented();

    let s1 = blocks1.checksum();
    let s2 = blocks2.checksum();

    Ok(format!("d09/01 = {}, d09/02 = {}", s1, s2))
}

#[derive(Debug, Clone)]
struct Blocks {
    blocks: Vec<Block>,
    next: Vec<Option<usize>>,
    previous: Vec<Option<usize>>,

    head: usize,
    tail: usize,
}

impl Blocks {
    fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    fn checksum(&self) -> u64 {
        if self.is_empty() {
            return 0;
        }

        let mut index = self.head;
        let mut result = 0;
        let mut compute_index = 0;

        loop {
            let block = &self.blocks[index];

            if !block.is_empty() && block.len > 0 {
                let factor = (2 * compute_index + block.len - 1) * block.len / 2;
                result += block.id().unwrap() * factor as u64;
            }

            compute_index += block.len;
            if let Some(next) = self.next[index] {
                index = next;
            } else {
                break;
            }
        }

        result
    }

    fn compress_fragmented(&mut self) {
        if self.is_empty() {
            return;
        }

        let mut i = self.head;
        let mut j = self.tail;

        'outer: loop {
            while !self.blocks[i].is_empty() {
                i = self.next[i].unwrap();
                if i == j {
                    break 'outer;
                }
            }

            while self.blocks[j].is_empty() {
                j = self.previous[j].unwrap();
                if j == i {
                    break 'outer;
                }
            }

            self.move_file(j, i);

            // safety, in case of block spit
            if let Some(t) = self.previous[i] {
                i = t;
            }
            if let Some(t) = self.next[j] {
                j = t;
            }
        }
    }

    fn compress_unfragmented(&mut self) {
        if self.is_empty() {
            return;
        }

        let Some(max_id) = self.blocks.iter().filter_map(|b| b.id()).max() else {
            return;
        };

        fn find_id(blocks: &[Block], id: u64) -> Option<usize> {
            blocks.iter().position(|b| b.id() == Some(id))
        }

        for id in (0..=max_id).rev() {
            let Some(index) = find_id(&self.blocks, id) else {
                continue;
            };

            let mut index2 = self.head;
            let mut found = false;
            loop {
                let b1 = &self.blocks[index];
                let b2 = &self.blocks[index2];

                if b2.is_empty() && b2.len >= b1.len {
                    found = true;
                    break;
                }

                if let Some(next) = self.next[index2] {
                    index2 = next;
                } else {
                    break;
                }

                if index == index2 {
                    break;
                }
            }

            if found {
                self.move_file(index, index2);
            }
        }
    }

    fn move_file(&mut self, from: usize, to: usize) {
        if self.is_empty() {
            return;
        }

        if self.blocks[from].is_empty() {
            return;
        }

        if !self.blocks[to].is_empty() {
            return;
        }

        if self.blocks[from].len > self.blocks[to].len {
            let delta = self.blocks[from].len - self.blocks[to].len;
            let new_index = self.insert_empty_after(from, delta);
            self.blocks[new_index].id_plus = self.blocks[from].id_plus;
            self.blocks[from].len = self.blocks[to].len;
        } else if self.blocks[from].len < self.blocks[to].len {
            let delta = self.blocks[to].len - self.blocks[from].len;
            self.insert_empty_after(to, delta);
            self.blocks[to].len = self.blocks[from].len;
        }

        self.swap(from, to)
    }

    fn insert_empty_after(&mut self, index: usize, len: usize) -> usize {
        self.blocks.push(Block::empty(len));
        self.previous.push(None);
        self.next.push(None);
        let new_index = self.blocks.len() - 1;
        if self.tail == index {
            self.tail = new_index;
        }

        self.next[new_index] = self.next[index];
        self.previous[new_index] = Some(index);

        if let Some(old_next) = self.next[index] {
            self.previous[old_next] = Some(new_index);
        }
        self.next[index] = Some(new_index);
        new_index
    }

    fn swap(&mut self, i: usize, j: usize) {
        if i == j {
            return;
        }

        self.blocks.swap(i, j);
    }

    // fn print(&self) {
    //     if self.is_empty() {
    //         return;
    //     }

    //     let mut index = self.head;
    //     loop {
    //         let block = &self.blocks[index];
    //         for _ in 0..block.len {
    //             match block.id() {
    //                 Some(id) => print!("{}", id),
    //                 None => print!("."),
    //             }
    //         }

    //         let Some(next_index) = self.next[index] else {
    //             break;
    //         };
    //         index = next_index;
    //     }
    //     println!();
    // }

    fn from_input(input: &str) -> Result<Self, aoc_common::AocError> {
        let mut empty = false;
        let mut next_id = 0;
        let mut blocks = Vec::new();

        for c in input.trim().chars() {
            let n = c.to_digit(10).ok_or(aoc_common::AocError::InvalidInput)?;
            if n == 0 {
                empty = !empty;
                continue;
            }
            let next_block = if !empty {
                let r = Block::from_id(next_id, n as usize);
                next_id += 1;
                r
            } else {
                Block::empty(n as usize)
            };
            blocks.push(next_block);
            empty = !empty;
        }

        let n = blocks.len();

        if n == 0 {
            return Err(aoc_common::AocError::InvalidInput);
        }

        let mut next = vec![None; n];
        for (i, n) in next.iter_mut().enumerate().take(n - 1) {
            *n = Some(i + 1);
        }

        let mut previous = vec![None; n];
        for (i, p) in previous.iter_mut().enumerate().take(n).skip(1) {
            *p = Some(i - 1);
        }

        Ok(Self {
            blocks,
            next,
            previous,
            head: 0,
            tail: n - 1,
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Block {
    id_plus: u64,
    len: usize,
}

impl Block {
    fn empty(len: usize) -> Self {
        Self { id_plus: 0, len }
    }

    fn from_id(id: u64, len: usize) -> Self {
        Self {
            id_plus: id + 1,
            len,
        }
    }

    fn is_empty(&self) -> bool {
        self.id_plus == 0
    }

    fn id(&self) -> Option<u64> {
        if self.is_empty() {
            None
        } else {
            Some(self.id_plus - 1)
        }
    }
}
