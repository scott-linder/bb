use mysql::value::from_value;
use mysql::conn::pool::MyPool;
use mysql::error::MyResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    id: i32,
    name: String,
    desc: String,
}

impl Board {
    pub fn all(pool: &mut MyPool) -> MyResult<Vec<Board>> {
        let mut stmt = try!(pool.prepare("SELECT * FROM bb.boards"));
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
