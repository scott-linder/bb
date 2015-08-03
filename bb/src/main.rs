extern crate mysql;
extern crate iron;
extern crate hyper;
extern crate router;
#[macro_use] extern crate horrorshow;

mod board;
mod thread;
mod post;
mod htmlafter;

use board::Board;
use thread::Thread;
use post::Post;

use std::default::Default;
use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use iron::{Iron, Request, Response, status};
use iron::middleware::Chain;
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
    router.get("/", move |_req: &mut Request| {
        let mut conn = pool.get_conn().unwrap();
        let boards = Board::all(&mut conn).unwrap();
        let html = Board::html(&boards[..]).unwrap();
        Ok(Response::with((status::Ok, html)))
    });

    let pool = my_pool.clone();
    router.get("/:board_name/", move |req: &mut Request| {
        let router = req.extensions.get::<Router>().unwrap();
        let board_name = router.find("board_name").unwrap();
        let mut conn = pool.get_conn().unwrap();
        let threads = Thread::for_board(&mut conn, board_name).unwrap();
        let html = Thread::html(&threads[..], board_name).unwrap();
        Ok(Response::with((status::Ok, html)))
    });

    let pool = my_pool.clone();
    router.get("/:board_name/thread/:thread_id/", move |req: &mut Request| {
        let router = req.extensions.get::<Router>().unwrap();
        let thread_id = router.find("thread_id").unwrap().parse().unwrap();
        let mut conn = pool.get_conn().unwrap();
        let posts = Post::for_thread(&mut conn, thread_id).unwrap();
        let html = Post::html(&posts[..]).unwrap();
        Ok(Response::with((status::Ok, html)))
    });

    let mut chain = Chain::new(router);
    chain.link_after(htmlafter::HtmlAfter);

    Iron::new(chain).http("localhost:8080").unwrap();
}
