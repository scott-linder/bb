use mysql::value::from_value;
use mysql::conn::pool::MyPooledConn;
use mysql::error::MyResult;
use horrorshow;
use horrorshow::prelude::*;

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

    pub fn html(posts: &[Self], board_name: &str, thread_id: i32) -> Result<String, horrorshow::Error> {
        let html = try!(html! {
            html {
                head {
                    title { : "posts" }
                    link(rel="stylesheet", href="/static/style/main.css");
                }
                body {
                    ul(class="breadcrumb") {
                        li { a(href="/") { : "boards" } }
                        li { a(href=format!("/{}", board_name)) { : board_name } }
                        li { : thread_id }
                    }
                    div(class="posts") {
                        @ for post in posts {
                            div(class="post") {
                                pre {
                                    : &post.text
                                }
                            }
                        }
                    }
                    form(action=format!("/{}/thread/{}/post", board_name, thread_id), method="POST") {
                        fieldset {
                            legend { : "create post" }
                            label(for="post_text") { : "post text" }
                            textarea(id="post_text", name="post_text") { : "" }
                            input(type="submit", value="create post");
                        }
                    }
                }
            }
        }.into_string());
        Ok(html)
    }

    pub fn insert(conn: &mut MyPooledConn, text: &str, thread_id: i32) -> MyResult<()> {
        let mut stmt = try!(conn.prepare("INSERT INTO posts(post_text, post_thread_id) VALUES (?,?)"));
        try!(stmt.execute(&[&text, &thread_id]));
        Ok(())
    }
}
