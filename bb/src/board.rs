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
                }
                body {
                    form(action="/", method="POST") {
                        input(type="text", name="board_name");
                        input(type="text", name="board_desc");
                        input(type="submit");
                    }
                    @ for board in boards {
                        div {
                            a(href=format!("/{}", board.name)) {
                                : &board.name
                            }
                            p {
                                : &board.desc
                            }
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
