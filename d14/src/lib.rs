use nalgebra::Vector2;
use rustc_hash::FxHashMap as HashMap;

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

pub fn solve(input: &str) -> aoc_common::AocResult {
    let re = regex::Regex::new(r"\s*p=(-?\d+),(-?\d+)\s*v=(-?\d+),(-?\d+)").unwrap();

    let mut robots = HashMap::default();

    for m in re.captures_iter(input.trim()) {
        let robot = Robot::from_strs(&m[1], &m[2], &m[3], &m[4])?;
        let stepped = robot.step(100);

        robots
            .entry(stepped.position)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    let s1 = safetyfactor(&robots);
    // part2(10000, input);
    Ok(format!("d14/01 = {}, d14/02 = not automated", s1))
}

struct Robot {
    position: Vector2<i64>,
    velocity: Vector2<i64>,
}

impl Robot {
    fn step(&self, seconds: i64) -> Robot {
        let mut result = Robot {
            position: self.position + self.velocity * seconds,
            velocity: self.velocity,
        };

        result.position.x %= WIDTH as i64;
        result.position.y %= HEIGHT as i64;

        if result.position.x < 0 {
            result.position.x += WIDTH as i64;
        }

        if result.position.y < 0 {
            result.position.y += HEIGHT as i64;
        }

        result
    }

    fn from_strs(px: &str, py: &str, vx: &str, vz: &str) -> Result<Self, aoc_common::AocError> {
        let position = Vector2::new(px.parse()?, py.parse()?);
        let velocity = Vector2::new(vx.parse()?, vz.parse()?);
        Ok(Robot { position, velocity })
    }
}

fn safetyfactor(robots: &HashMap<Vector2<i64>, u64>) -> u64 {
    let mut qudrants = [0; 4];

    let w = WIDTH as i64;
    let h = HEIGHT as i64;

    for (pos, count) in robots.iter() {
        if pos.x < w / 2 && pos.y < h / 2 {
            qudrants[0] += count;
        } else if pos.x < w / 2 && pos.y > h / 2 {
            qudrants[1] += count;
        } else if pos.x > w / 2 && pos.y < h / 2 {
            qudrants[2] += count;
        } else if pos.x > w / 2 && pos.y > h / 2 {
            qudrants[3] += count;
        }
    }

    qudrants.iter().product()
}

// // helper for part2
// fn part2(max_steps: usize, input: &str) {
//     fn compute_std(robots: &[Robot]) -> f64 {
//         let mut mean = Vector2::new(0.0, 0.0);
//         for r in robots.iter() {
//             mean += r.position.map(|x| x as f64);
//         }
//         mean /= robots.len() as f64;

//         let mut var = Vector2::new(0.0, 0.0);
//         for r in robots.iter() {
//             let mut diff = r.position.map(|x| x as f64) - mean;
//             diff = diff.component_mul(&diff);
//             var += diff.component_mul(&diff);
//         }
//         var /= robots.len() as f64;
//         var -= mean.component_mul(&mean);
//         if var.x < 0.0 {
//             var.x = 0.0;
//         }
//         if var.y < 0.0 {
//             var.y = 0.0;
//         }

//         var.x.sqrt() + var.y.sqrt()
//     }

//     let re = regex::Regex::new(r"\s*p=(-?\d+),(-?\d+)\s*v=(-?\d+),(-?\d+)").unwrap();

//     let mut robots = Vec::new();

//     for m in re.captures_iter(input.trim()) {
//         let robot = Robot::from_strs(&m[1], &m[2], &m[3], &m[4]).unwrap();
//         robots.push(robot);
//     }

//     let mut stds = Vec::new();
//     for _ in 0..max_steps {
//         stds.push(compute_std(&robots));
//         for r in robots.iter_mut() {
//             *r = r.step(1);
//         }
//     }

//     let min_std = stds
//         .iter()
//         .min_by(|a, b| a.partial_cmp(b).unwrap())
//         .unwrap();
//     let min_step = stds.iter().position(|x| x == min_std).unwrap();
//     println!("d14/02 = {}", min_step);

//     robots.clear();
//     for m in re.captures_iter(input.trim()) {
//         let robot = Robot::from_strs(&m[1], &m[2], &m[3], &m[4]).unwrap();
//         robots.push(robot);
//     }
//     for _ in 0..min_step {
//         for r in robots.iter_mut() {
//             *r = r.step(1);
//         }
//     }
//     print_robots(&robots);
// }

// fn print_robots(robots: &[Robot]) {
//     for y in 0..HEIGHT as i64 {
//         for x in 0..WIDTH as i64 {
//             let mut found = false;
//             for r in robots.iter() {
//                 if r.position.x == x && r.position.y == y {
//                     found = true;
//                     break;
//                 }
//             }

//             if found {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }
