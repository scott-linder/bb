extern crate mysql;
extern crate iron;
extern crate router;

mod board;
mod thread;
mod post;

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
    let my_pool = MyPool::new(opts).unwrap();

    let mut router = Router::new();
    let pool = my_pool.clone();
    router.get("/board", move |_req: &mut Request| {
        let boards = board::Board::all(&mut pool.get_conn().unwrap());
        Ok(Response::with((status::Ok, format!("{:#?}", boards))))
    });
    let pool = my_pool.clone();
    router.get("/board/:board_id/thread", move |req: &mut Request| {
        let router = req.extensions.get::<Router>().unwrap();
        let board_id = router.find("board_id").unwrap();
        let threads = thread::Thread::for_board(&mut pool.get_conn().unwrap(),
                                                board_id.parse().unwrap());
        Ok(Response::with((status::Ok, format!("{:#?}", threads))))
    });
    let pool = my_pool.clone();
    router.get("/thread/:thread_id/post", move |req: &mut Request| {
        let router = req.extensions.get::<Router>().unwrap();
        let thread_id = router.find("thread_id").unwrap();
        let posts = post::Post::for_thread(&mut pool.get_conn().unwrap(),
                                           thread_id.parse().unwrap());
        Ok(Response::with((status::Ok, format!("{:#?}", posts))))
    });

    Iron::new(router).http("localhost:8080").unwrap();
}
