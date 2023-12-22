# ahc-rust-template

This is a template repository for AHC (e.g., https://atcoder.jp/contests/ahc020).
It helps a bit complex project set up that allows to write multi-testcases evaluation easily.

## Preliminary

* Install https://github.com/qryxip/cargo-equip.
* Install https://github.com/online-judge-tools/oj

## How to set up a repository for a new contest

1. Create a new repository from this template ([ref](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template)). **USE "PRIVATE" FOR ACTIVE CONTESTS**
2. Copy `tools` (offical tester linked from the problem page) to the root directory.
3. Replace `CONTEST_ID` in `submit.sh`.

## How to use this template

* Update `common/src/lib.rs` to define problem IO format and your solution
* Update `submission/src/main.rs` to define text parser/printer for Atcoder.
* Update `evaluator/src/main.rs` to define generation and evaluation of test cases with official `tools` crate.
* Run `cargo run --bin evaluator -- -n 1000` to run your evaluation against multiple test cases to get aggregated score.
* Run `cargo run --bin submission` to run your solver against single test case to see WebUI visualization.

## Tips

* library versions in Atcoder environment are listed at https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/2020-Update (2020 version) or https://img.atcoder.jp/file/language-update/language-list.html or https://github.com/rust-lang-ja/atcoder-rust-resources/wiki/Jan-2023-Language-Update-%E3%83%A9%E3%82%A4%E3%83%96%E3%83%A9%E3%83%AA%E6%A1%88 (2023 version)

The result of
```
fn main() {
  println!("{}", include_str!("../Cargo.toml"));
}
```

is
```
[profile.release]
lto = true # コンパイル時間が著しく長くなってしまう場合は無し

[package]
name = "main"
version = "0.0.0"
edition = "2021"
publish = false

[dependencies]
# 202301から:
ac-library-rs = "=0.1.1"
once_cell = "=1.18.0"
static_assertions = "=1.1.0"
varisat = "=0.2.2"
memoise = "=0.3.2"
argio = "=0.2.0"
bitvec = "=1.0.1"
counter = "=0.5.7"
hashbag = "=0.1.11"
pathfinding = "=4.3.0"
recur-fn = "=2.2.0"
indexing = { version = "=0.4.1", features = ["experimental_pointer_ranges"] }
amplify = { version = "=3.14.2", features = ["c_raw", "rand", "stringly_conversions"] }
amplify_derive = "=2.11.3"
amplify_num = { version = "=0.4.1", features = ["std"] }
easy-ext = "=1.0.1"
multimap = "=0.9.0"
btreemultimap = "=0.1.1"
bstr = "=1.6.0"
az = "=1.2.1"
glidesort = "=0.1.2"
tap = "=1.0.1"
omniswap = "=0.1.0"
multiversion = "=0.7.2"
# 202004から続投:
num = "=0.4.1"
num-bigint = "=0.4.3"
num-complex = "=0.4.3"
num-integer = "=0.1.45"
num-iter = "=0.1.43"
num-rational = "=0.4.1"
num-traits = "=0.2.15"
num-derive = "=0.4.0"
ndarray = "=0.15.6"
nalgebra = "=0.32.3"
alga = "=0.9.3"
libm = "=0.2.7"
rand = { version = "=0.8.5", features = ["small_rng", "min_const_gen"] }
getrandom = "=0.2.10"
rand_chacha = "=0.3.1"
rand_core = "=0.6.4"
rand_hc = "=0.3.2"
rand_pcg = "=0.3.1"
rand_distr = "=0.4.3"
petgraph = "=0.6.3"
indexmap = "=2.0.0"
regex = "=1.9.1"
lazy_static = "=1.4.0"
ordered-float = "=3.7.0"
ascii = "=1.1.0"
permutohedron = "=0.2.4"
superslice = "=1.0.0"
itertools = "=0.11.0"
itertools-num = "=0.1.3"
maplit = "=1.0.2"
either = "=1.8.1"
im-rc = "=15.1.0"
fixedbitset = "=0.4.2"
bitset-fixed = "=0.1.0"
proconio = { version = "=0.4.5", features = ["derive"] }
text_io = "=0.1.12"
rustc-hash = "=1.1.0"
smallvec = { version = "=1.11.0", features = ["const_generics", "const_new", "write", "union", "serde", "arbitrary"] }
```
