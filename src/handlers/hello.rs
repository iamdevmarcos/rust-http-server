use crate::models::response::Response;

pub async fn hello_handler() -> Result<impl warp::Reply, warp::Rejection> {
  let response = Response {
    message: "ping to hello_handler".to_string(),
  };

  Ok(warp::reply::json(&response))
}