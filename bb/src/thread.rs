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

    pub fn html(threads: &[(&Self, i32)], board_name: &str) -> Result<String, horrorshow::Error> {
        let html = try!(html! {
            html {
                head {
                    title { : "threads" }
                    link(rel="stylesheet", href="/static/bootstrap/css/bootstrap.min.css");
                    link(rel="stylesheet", href="/static/bootstrap/css/bootstrap-theme.min.css");
                    link(rel="stylesheet", href="/static/style/main.css");
                }
                body {
                    ul(class="breadcrumb") {
                        li { a(href="/") { : "boards" } }
                        li { : board_name }
                    }
                    div(class="threads") {
                        @ for thread in threads {
                            a(href=format!("/{}/thread/{}", board_name, thread.0.id)) {
                                div(class="thread panel panel-default") {
                                    div(class="panel-heading") {
                                        h3(class="panel-title") {
                                            : &thread.0.title
                                        }
                                    }
                                    div(class="panel-body") {
                                        : format!("{} posts", thread.1)
                                    }
                                }
                            }
                        }
                    }
                    form(action=format!("/{}/thread", board_name), method="POST") {
                        fieldset {
                            legend { : "create thread" }
                            label(for="thread_title") { : "thread title" }
                            input(type="text", id="thread_title", required, maxlength="64", name="thread_title");
                            input(type="submit", class="btn btn-default", value="create thread");
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

    pub fn post_count(&self, conn: &mut MyPooledConn) -> MyResult<i32> {
        let mut stmt = try!(conn.prepare(
            "SELECT COUNT(*)
            FROM threads
            JOIN posts ON post_thread_id = thread_id
            WHERE thread_id = ?"
        ));
        let mut result = try!(stmt.execute(&[&self.id]));
        return if let Some(row) = result.next() {
            let row = try!(row);
            Ok(from_value(&row[0]))
        } else {
            Ok(0)
        }
    }
}
