use iron::{Request, Response, IronResult};
use iron::middleware::AfterMiddleware;
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::header::ContentType;
use iron::modifier::Set;
use iron::modifiers::Header;

/// Iron middleware which sets content-type header to 'text/html'
pub struct HtmlAfter;

impl AfterMiddleware for HtmlAfter {
    fn after(&self, _: &mut Request, res: Response) -> IronResult<Response> {
        Ok(res.set(Header(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])))))
    }
}
