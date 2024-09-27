use tower::Service;
use std::task::{Context, Poll};
use crate::session::Session;

#[derive(Debug, Clone)]
pub struct SessionManager<S> {
    inner: S,
    //
}

impl<S> SessionManager<S> {
    pub fn new(inner: S) -> Self {
        SessionManager { inner }
    }
}

pub struct ResponseFuture<F> {
    response_future: F,
    session: Session
}

impl<S, Request> Service<Request> for SessionManager<S>
where 
    S: Service<Request>
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        //let response_future = self.inner.call(request);

        self.inner.call(request)
    }
}


