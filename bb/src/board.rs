use mysql::value::from_value;
use mysql::conn::pool::MyPooledConn;
use mysql::error::MyResult;
use horrorshow;
use horrorshow::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    pub name: String,
    pub desc: String,
}

impl Board {
    pub fn all(conn: &mut MyPooledConn) -> MyResult<Vec<Board>> {
        let mut stmt = try!(conn.prepare("SELECT * FROM boards"));
        let result = try!(stmt.execute(&[]));
        let mut boards = Vec::new();
        for row in result {
            let row = try!(row);
            boards.push(Board {
                name: from_value(&row[0]),
                desc: from_value(&row[1]),
            });
        }
        Ok(boards)
    }

    pub fn html(boards: &[Self]) -> Result<String, horrorshow::Error> {
        let html = try!(html! {
            html {
                head {
                    title { : "boards" }
                    link(rel="stylesheet", href="/static/bootstrap/css/bootstrap.min.css");
                    link(rel="stylesheet", href="/static/bootstrap/css/bootstrap-theme.min.css");
                    link(rel="stylesheet", href="/static/style/main.css");
                }
                body {
                    ul(class="breadcrumb") {
                        li { : "boards" }
                    }
                    div(class="boards") {
                        @ for board in boards {
                            a(href=format!("/{}", board.name)) {
                                div(class="board panel panel-default") {
                                    div(class="panel-heading") {
                                        h3(class="panel-title") {
                                            : &board.name
                                        }
                                    }
                                    div(class="panel-body") {
                                        : &board.desc
                                    }
                                }
                            }
                        }
                    }
                    form(action="/", method="POST") {
                        fieldset {
                            legend { : "create board" }
                            label(for="board_name") { : "board name" }
                            input(type="text", id="board_name", name="board_name");
                            label(for="board_desc") { : "board desc" }
                            input(type="text", id="board_desc", name="board_desc");
                            input(type="submit", class="btn btn-default", value="create board");
                        }
                    }
                }
            }
        }.into_string());
        Ok(html)
    }

    pub fn insert(conn: &mut MyPooledConn, name: &str, desc: &str) -> MyResult<()> {
        let mut stmt = try!(conn.prepare("INSERT INTO boards(board_name, board_desc) VALUES (?,?)"));
        try!(stmt.execute(&[&name, &desc]));
        Ok(())
    }
}
