use mysql::value::from_value;
use mysql::conn::pool::MyPooledConn;
use mysql::error::MyResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Post {
    id: i32,
    text: String,
    thread_id: i32,
}

impl Post {
    pub fn for_thread(conn: &mut MyPooledConn, thread_id: i32) -> MyResult<Vec<Post>> {
        let mut stmt = try!(conn.prepare("SELECT * FROM posts WHERE post_thread_id = ?"));
        let result = try!(stmt.execute(&[&thread_id]));
        let mut posts = Vec::new();
        for row in result {
            let row = try!(row);
            posts.push(Post {
                id: from_value(&row[0]),
                text: from_value(&row[1]),
                thread_id: from_value(&row[2]),
            });
        }
        Ok(posts)
    }
}