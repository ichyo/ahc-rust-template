use common::*;

fn main() {
    let is_local = !std::env::var("ATCODER").is_ok();
    solve(&mut Atcoder { is_local });
}
