use tlog::{tlog, type_name};

fn main() {
    tlog!("hello");

    let n = 3;
    tlog!("5 x 7 = {}, {}", 5 * 7, type_name(&n));
}
