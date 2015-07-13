use mysql::value::from_value;
use mysql::conn::pool::MyPooledConn;
use mysql::error::MyResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    id: i32,
    name: String,
    desc: String,
}

impl Board {
    pub fn all(conn: &mut MyPooledConn) -> MyResult<Vec<Board>> {
        let mut stmt = try!(conn.prepare("SELECT * FROM boards"));
        let result = try!(stmt.execute(&[]));
        let mut boards = Vec::new();
        for row in result {
            let row = try!(row);
            boards.push(Board {
                id: from_value(&row[0]),
                name: from_value(&row[1]),
                desc: from_value(&row[2]),
            });
        }
        Ok(boards)
    }
}
