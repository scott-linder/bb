extern crate mysql;

mod board;

use std::default::Default;

use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_value;

fn main() {
    let opts = MyOpts {
        user: Some("bb".into()),
        db_name: Some("bb".into()),
        ..Default::default()
    };
    let mut pool = MyPool::new(opts).unwrap();

    let boards = board::Board::all(&mut pool);
    println!("{:#?}", boards);
}
