use clap::Parser;
use common::*;
use rayon::prelude::*;
use std::time::{Duration, Instant};

struct Evaluator {
    seed: u64,
    score: i64,
}

struct Result {
    seed: u64,
    score: i64,
    elapsed: Duration,
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

    fn is_debug(&self) -> bool {
        false
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

    let results: Vec<Result> = (0..args.num_testcase)
        .into_par_iter()
        .map(|seed| {
            let mut evaluator = Evaluator::new(seed);
            let start = Instant::now();
            solve(&mut evaluator);
            let elapsed = start.elapsed();
            Result {
                seed: seed,
                score: evaluator.score,
                elapsed: elapsed,
            }
        })
        .collect();

    for result in results.iter() {
        println!("seed:{:>3} score:{:>9} time:{:>5}ms", 
            result.seed, 
            result.score, 
            result.elapsed.as_millis()
        );
    }

    let score_sum = results.iter().map(|r| r.score).sum::<i64>();
    let max_elapsed = results.iter().map(|r| r.elapsed).max().unwrap_or(Duration::ZERO);

    println!("average: {}", score_sum as f64 / args.num_testcase as f64);
    println!("max time: {}ms", max_elapsed.as_millis());
}
