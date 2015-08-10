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
            head {
                title { : "threads" }
            }
            body {
                @ for thread in threads {
                    div {
                        a(href=format!("/{}/thread/{}", board_name, thread.id)) {
                            : &thread.title
                        }
                    }
                }
            }
        }.into_string());
        Ok(html)
    }
}
