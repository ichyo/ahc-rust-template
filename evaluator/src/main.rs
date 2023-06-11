use clap::Parser;
use rayon::prelude::*;
use solver::*;

struct Evaluator {
    seed: u64,
    score: i64,
}

impl Evaluator {
    fn new(seed: u64) -> Evaluator {
        Evaluator { seed, score: 0 }
    }
}

impl Environment for Evaluator {
    fn receive_input(&mut self) -> ProblemInput {
        // Use `tools` to generate test case
        ProblemInput {
            a: self.seed as i64,
            b: self.seed as i64 + 1,
        }
    }

    fn send_output(&mut self, output: ProblemOutput) {
        // Use `tools` to compute score
        self.score = output.result;
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// The number of test cases
    #[arg(short, long, default_value = "100")]
    num_testcase: u64,
}

fn main() {
    let args = Args::parse();

    let score_sum: i64 = (0..args.num_testcase)
        .into_par_iter()
        .map(|seed| {
            let mut evaluator = Evaluator::new(seed);
            solve(&mut evaluator);
            println!("seed={} score={}", seed, evaluator.score);
            evaluator.score
        })
        .sum();

    println!("average: {}", score_sum as f64 / args.num_testcase as f64);
}
