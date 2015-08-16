use std::io::{Read, Cursor};
use iron::{Request, Response, IronResult};
use hyper::header::ContentLength;
use iron::middleware::AfterMiddleware;
use iron::modifier::Set;

/// Iron middleware which prepends an HTML5 doctype declaration to response bodies.
pub struct DoctypeAfter;

impl AfterMiddleware for DoctypeAfter {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        if let Some(body) = res.body.take() {
            let doctype: Cursor<Vec<u8>> = Cursor::new("<!DOCTYPE html>".into());
            let new_body = Box::new(doctype.chain(body));
            res.headers.remove::<ContentLength>();
            res = res.set(new_body as Box<Read + Send>);
        }
        Ok(res)
    }
}
