use proconio::input;

use solver::*;

struct Atcoder {}

impl Environment for Atcoder {
    fn receive_input(&mut self) -> ProblemInput {
        // Write parser of text input here

        input! {
            a: i64,
            b: i64,
        }

        ProblemInput { a, b }
    }

    fn send_output(&mut self, output: ProblemOutput) {
        // Write print of text output here

        println!("{}", output.result);
    }
}

fn main() {
    solve(&mut Atcoder {});
}
