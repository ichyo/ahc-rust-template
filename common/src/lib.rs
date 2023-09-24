use rand::prelude::*;

pub struct ProblemInput {
    // Write the format of problem input
    pub a: i64,
    pub b: i64,
}

pub struct ProblemOutput {
    // Write the format of problem output
    pub result: i64,
}

// Common interface for atcoder and evaluator
pub trait Environment {
    // Update the methods if the problem is interactive

    fn receive_input(&mut self) -> ProblemInput;
    fn send_output(&mut self, output: ProblemOutput);
}

pub fn solve<E: Environment>(environment: &mut E) {
    // Write your solution here

    let input = environment.receive_input();
    let mut rng = SmallRng::seed_from_u64(58);
    let output = ProblemOutput {
        result: input.a + input.b + rng.gen_range(0..10),
    };
    environment.send_output(output);
}
