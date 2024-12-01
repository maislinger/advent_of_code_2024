fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build_global()
        .unwrap();

    let func_input = [(d01::solve, read_input("./inputs/d01"))];
    let mut solutions: Vec<aoc_common::AocResult> = Vec::with_capacity(func_input.len());
    for _ in 0..func_input.len() {
        solutions.push(Ok("".into()));
    }

    let t = std::time::Instant::now();

    rayon::scope(|s| {
        let mut sols: &mut [aoc_common::AocResult] = &mut solutions;
        for (f, input) in func_input.iter() {
            let (single, rest) = sols.split_at_mut(1);
            s.spawn(move |_| single[0] = f(input));
            sols = rest;
        }
    });

    let dt = t.elapsed();

    for (i, solution) in solutions.iter().enumerate() {
        match solution {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error in day {:02}: {}", i + 1, e),
        }
    }

    println!("Elapsed time for all solutions: {:?}", dt);
}

fn read_input(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
