use warp::http::status::StatusCode;
use warp::Filter;
use warp::reply::{Reply, Response};
use warp::reject::Rejection;
use warp::http::uri;
use std::collections::HashMap;
use std::str::FromStr;

async fn index() -> Result<impl Reply, Rejection> {
  Ok(warp::reply::html("hey"))
}

async fn not_found() -> Result<impl Reply, Rejection> {
  Ok(warp::redirect(uri::Uri::from_str("/").unwrap()))
}

fn main() {
  let mut rt = tokio::runtime::Builder::new()
    .core_threads(32)
    .threaded_scheduler()
    .enable_all()
    .build()
    .unwrap();

  rt.block_on(async {
    let router = {
      let index =
        warp::path::end()
          //.and(warp::query::<HashMap<String, String>>())
          .and_then(index);
      index
        .or(warp::any().and_then(not_found))
    };
    warp::serve(router)
      .run(([127, 0, 0, 1], 3030))
      .await;
  });

}
