# ahc-rust-template

This is a template repository for AHC (e.g., https://atcoder.jp/contests/ahc020).
It helps a bit complex project set up that allows to write multi-testcases evaluation easily.

## Preliminary

* Install https://github.com/qryxip/cargo-equip. (In case you get segfault from the tool, try https://github.com/ichyo/cargo-equip)
* Install https://github.com/online-judge-tools/oj

## How to set up a repository for a new contest

1. Create a new repository from this template ([ref](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template)). **USE "PRIVATE" FOR ACTIVE CONTESTS**
2. Copy `tools` (offical tester linked from the problem page) to the root directory.
3. Replace `CONTEST_ID` in `submit.sh`.

## How to use this template

* Update `solver/src/lib.rs` to define problem IO format and your solution
* Update `submission/src/main.rs` to define text parser/printer for Atcoder.
* Update `evaluator/src/main.rs` to define generation and evaluation of test cases with official `tools` crate.
* Run `cargo run --bin evaluator -- -n 1000` to run your evaluation against multiple test cases to get aggregated score.
* Run `cargo run --bin submission` to run your solver against single test case to see WebUI visualization.

## Tips

* library versions in Atcoder environment are listed at https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/2020-Update (2020 version) or https://img.atcoder.jp/file/language-update/language-list.html or https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/Jan-2023-Language-Update-%E3%83%A9%E3%82%A4%E3%83%96%E3%83%A9%E3%83%AA%E6%A1%88 (2023 version)
