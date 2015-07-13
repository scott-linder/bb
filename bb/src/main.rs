extern crate mysql;
extern crate iron;
extern crate router;

mod board;

use std::default::Default;
use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use iron::{Iron, Request, Response, status};
use router::Router;

fn main() {
    let opts = MyOpts {
        user: Some("bb".into()),
        db_name: Some("bb".into()),
        ..Default::default()
    };
    let pool = MyPool::new(opts).unwrap();

    let mut router = Router::new();
    router.get("/boards", move |_req: &mut Request| {
        let boards = board::Board::all(&mut pool.get_conn().unwrap());
        Ok(Response::with((status::Ok, format!("{:#?}", boards))))
    });

    Iron::new(router).http("localhost:8080").unwrap();
}
