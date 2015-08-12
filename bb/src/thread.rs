use mysql::value::from_value;
use mysql::conn::pool::MyPooledConn;
use mysql::error::MyResult;
use horrorshow;
use horrorshow::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Thread {
    id: i32,
    title: String,
    board_name: String,
}

impl Thread {
    pub fn for_board(conn: &mut MyPooledConn, board_name: &str) -> MyResult<Vec<Thread>> {
        let mut stmt = try!(conn.prepare("SELECT * FROM threads WHERE thread_board_name = ?"));
        let result = try!(stmt.execute(&[&board_name]));
        let mut threads = Vec::new();
        for row in result {
            let row = try!(row);
            threads.push(Thread {
                id: from_value(&row[0]),
                title: from_value(&row[1]),
                board_name: from_value(&row[2]),
            });
        }
        Ok(threads)
    }

    pub fn html(threads: &[Self], board_name: &str) -> Result<String, horrorshow::Error> {
        let html = try!(html! {
            html {
                head {
                    title { : "threads" }
                    link(rel="stylesheet", href="/static/style/main.css");
                }
                body {
                    ul(class="breadcrumb") {
                        li { a(href="/") { : "boards" } }
                        li { : board_name }
                    }
                    ul(class="threads") {
                        @ for thread in threads {
                            li(class="thread") {
                                a(href=format!("/{}/thread/{}", board_name, thread.id)) {
                                    : &thread.title
                                }
                            }
                        }
                    }
                    form(action=format!("/{}/thread", board_name), method="POST") {
                        fieldset {
                            legend { : "create thread" }
                            label(for="thread_title") { : "thread title" }
                            input(type="text", id="thread_title", name="thread_title");
                            input(type="submit", value="create thread");
                        }
                    }
                }
            }
        }.into_string());
        Ok(html)
    }

    pub fn insert(conn: &mut MyPooledConn, title: &str, board_name: &str) -> MyResult<()> {
        let mut stmt = try!(conn.prepare("INSERT INTO threads(thread_title, thread_board_name) VALUES (?,?)"));
        try!(stmt.execute(&[&title, &board_name]));
        Ok(())
    }
}
