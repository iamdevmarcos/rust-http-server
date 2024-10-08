use warp::Filter;
use crate::handlers::hello::hello_handler;

pub fn home_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path::end()
    .and(warp::get())
    .and_then(hello_handler)
}