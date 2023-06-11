# ahc-rust-template

This is a template repository to help a bit complex project set up that allows to write multi-testcases evaluation easily.

## How to set up

1. Create a new repository from this template ([ref](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template))
2. Copy `tools` to the root directory.
3. Replace `CONTEST_ID` in `submit.sh`

## How to use this template

* Update `solver/src/lib.rs` to define problem IO format and your solution
* Update `submission/src/main.rs` to define text parser/printer for Atcoder.
* Update `evaluator/src/main.rs` to define generation and evaluation of test cases with official `tools` crate.
* Run `cargo run --bin evaluator -- -n 1000` to run your evaluation against multiple test cases to get aggregated score.
* Run `cargo run --bin submission` to run your solver against single test case to see WebUI visualization.

## Tips

* library versions in Atcoder environment are listed at https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/2020-Update
