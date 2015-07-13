use mysql::value::from_value;
use mysql::conn::pool::MyPooledConn;
use mysql::error::MyResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Thread {
    id: i32,
    title: String,
    board_id: i32,
}

impl Thread {
    pub fn for_board(conn: &mut MyPooledConn, board_id: i32) -> MyResult<Vec<Thread>> {
        let mut stmt = try!(conn.prepare("SELECT * FROM threads WHERE thread_board_id = ?"));
        let result = try!(stmt.execute(&[&board_id]));
        let mut threads = Vec::new();
        for row in result {
            let row = try!(row);
            threads.push(Thread {
                id: from_value(&row[0]),
                title: from_value(&row[1]),
                board_id: from_value(&row[2]),
            });
        }
        Ok(threads)
    }
}
