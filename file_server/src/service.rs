use hyper::body::Incoming as IncomingBody;
use hyper::http::Request;
use hyper::service::Service;
use hyper::StatusCode;
use std::future::Future;
use std::path;
use std::pin::Pin;

use crate::responses;

pub struct Svc {
    pub directory: path::PathBuf,
}

impl Service<Request<IncomingBody>> for Svc {
    type Response = responses::BoxedResponse;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        if let Ok(path) = responses::get_pathbuff_from_request(&self.directory, &req) {
            // confirm canon'd path resides in directory
            if path.starts_with(&self.directory) {
                return Box::pin(async { responses::build_response(path).await });
            }
        }

        Box::pin(async {
            responses::create_error_response(&StatusCode::NOT_FOUND, "404 not found")
        })
    }
}
