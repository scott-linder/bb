extern crate mysql;
extern crate iron;
extern crate hyper;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate horrorshow;

mod board;
mod thread;
mod post;
mod htmlafter;
mod doctypeafter;

use board::Board;
use thread::Thread;
use post::Post;

use std::default::Default;
use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use iron::{Iron, Request, Response, Url, Plugin, status};
use iron::middleware::Chain;
use iron::modifiers::Redirect;
use router::Router;
use urlencoded::UrlEncodedBody;

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
    router.post("/", move |req: &mut Request| {
        {
            let params = req.get_ref::<UrlEncodedBody>().unwrap();
            let board_name = params.get("board_name").unwrap().first().unwrap();
            let board_desc = params.get("board_desc").unwrap().first().unwrap();
            let mut conn = pool.get_conn().unwrap();
            Board::insert(&mut conn, board_name, board_desc).unwrap();
        }
        let redir_url = Url { path: vec![], ..req.url.clone() };
        Ok(Response::with((status::SeeOther, Redirect(redir_url))))
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
    router.post("/:board_name/thread/", move |req: &mut Request| {
        let board_name: String = {
            let router = req.extensions.get::<Router>().unwrap();
            router.find("board_name").unwrap().into()
        };
        {
            let params = req.get::<UrlEncodedBody>().unwrap();
            let thread_title = params.get("thread_title").unwrap().first().unwrap();
            let mut conn = pool.get_conn().unwrap();
            Thread::insert(&mut conn, thread_title, &*board_name).unwrap();
        }
        let redir_url = Url { path: vec![board_name], ..req.url.clone() };
        Ok(Response::with((status::SeeOther, Redirect(redir_url))))
    });

    let pool = my_pool.clone();
    router.get("/:board_name/thread/:thread_id/", move |req: &mut Request| {
        let board_name: String = {
            let router = req.extensions.get::<Router>().unwrap();
            router.find("board_name").unwrap().into()
        };
        let router = req.extensions.get::<Router>().unwrap();
        let thread_id = router.find("thread_id").unwrap().parse().unwrap();
        let mut conn = pool.get_conn().unwrap();
        let posts = Post::for_thread(&mut conn, thread_id).unwrap();
        let html = Post::html(&posts[..], &*board_name, thread_id).unwrap();
        Ok(Response::with((status::Ok, html)))
    });

    let pool = my_pool.clone();
    router.post("/:board_name/thread/:thread_id/post", move |req: &mut Request| {
        let board_name: String = {
            let router = req.extensions.get::<Router>().unwrap();
            router.find("board_name").unwrap().into()
        };
        let thread_id: i32 = {
            let router = req.extensions.get::<Router>().unwrap();
            router.find("thread_id").unwrap().parse().unwrap()
        };
        {
            let params = req.get::<UrlEncodedBody>().unwrap();
            let post_text = params.get("post_text").unwrap().first().unwrap();
            let mut conn = pool.get_conn().unwrap();
            Post::insert(&mut conn, post_text, thread_id).unwrap();
        }
        let redir_url = Url { path: vec![board_name, "thread".into(), format!("{}", thread_id)], ..req.url.clone() };
        Ok(Response::with((status::SeeOther, Redirect(redir_url))))
    });
    let mut chain = Chain::new(router);
    chain.link_after(htmlafter::HtmlAfter);
    chain.link_after(doctypeafter::DoctypeAfter);

    Iron::new(chain).http("localhost:8080").unwrap();
}
