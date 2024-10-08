use std::net::IpAddr;

use crate::routes;

pub async fn start(config: crate::config::Config) {
  let home_route = routes::home::home_route();

  let host: IpAddr = config.host.parse().expect("Wrong IP Address");

  warp::serve(home_route)
    .run((host, config.port))
    .await;
}