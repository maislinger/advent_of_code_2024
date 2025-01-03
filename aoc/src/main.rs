fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build_global()
        .unwrap();

    let func_input: [(fn(&str) -> aoc_common::AocResult, String); 14] = [
        (d01::solve, read_input("./inputs/d01")),
        (d02::solve, read_input("./inputs/d02")),
        (d03::solve, read_input("./inputs/d03")),
        (d04::solve, read_input("./inputs/d04")),
        (d05::solve, read_input("./inputs/d05")),
        (d06::solve, read_input("./inputs/d06")),
        (d07::solve, read_input("./inputs/d07")),
        (d08::solve, read_input("./inputs/d08")),
        (d09::solve, read_input("./inputs/d09")),
        (d10::solve, read_input("./inputs/d10")),
        (d11::solve, read_input("./inputs/d11")),
        (d12::solve, read_input("./inputs/d12")),
        (d13::solve, read_input("./inputs/d13")),
        (d14::solve, read_input("./inputs/d14")),
    ];

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
