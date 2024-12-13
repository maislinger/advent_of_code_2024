use nalgebra::Vector2;

pub fn solve(input: &str) -> aoc_common::AocResult {
    let mut s1 = 0;
    let mut s2 = 0;

    let re = regex::Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    for m in re.captures_iter(input.trim()) {
        let claw = Claw::from_strs(&m[1], &m[2], &m[3], &m[4], &m[5], &m[6])?;
        s1 += claw.min_tokens(0);
        s2 += claw.min_tokens(10_000_000_000_000);
    }

    Ok(format!("d12/01 = {}, d12/02 = {}", s1, s2))
}

#[derive(Debug, Clone, Copy)]
struct Claw {
    a: nalgebra::Vector2<i64>,
    b: nalgebra::Vector2<i64>,
    prize: nalgebra::Vector2<i64>,
}

impl Claw {
    fn min_tokens(&self, factor: i64) -> i64 {
        let p = self.prize + nalgebra::Vector2::new(factor, factor);

        let aa = self.a.dot(&self.a);
        let ab = self.a.dot(&self.b);
        let bb = self.b.dot(&self.b);

        // let numerator = aa * bp - ap * ab;
        let numerator = (aa * self.b - ab * self.a).dot(&p);
        let denominator = aa * bb - ab * ab;

        // Inclompete but not for my input and i don't want to spend time on it
        assert_ne!(denominator, 0);

        if numerator % denominator != 0 {
            return 0;
        }

        let nb = numerator / denominator;
        let delta = p - self.b * nb;

        let numerator = self.a.dot(&delta);
        let denominator = aa;

        if numerator % denominator != 0 {
            return 0;
        }

        let na = numerator / denominator;

        if na < 0 || nb < 0 {
            0
        } else {
            3 * na + nb
        }
    }

    fn from_strs(
        xa: &str,
        ya: &str,
        xb: &str,
        yb: &str,
        xp: &str,
        yp: &str,
    ) -> Result<Self, aoc_common::AocError> {
        Ok(Self {
            a: Vector2::new(xa.parse()?, ya.parse()?),
            b: Vector2::new(xb.parse()?, yb.parse()?),
            prize: Vector2::new(xp.parse()?, yp.parse()?),
        })
    }
}
