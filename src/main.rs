/* coding: utf-8 */
/******************************************************************************
 * open-kirara-platform
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/

use warp::Filter;
use warp::reply::Reply;
use warp::reject::Rejection;
use warp::http::uri;
use std::str::FromStr;

mod web;
use web::template::{*};
mod repo;

async fn render<T: yarte::Template>(renderer: T) -> Result<impl Reply, Rejection> {
  match renderer.call() {
    Ok(body) => Ok(warp::reply::html(body)),
    Err(err) => Err(warp::reject::custom::<TemplateError>(err.into())),
  }
}

async fn index() -> Result<impl Reply, Rejection> {
  render(Index{}).await
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

  let router = warp::path::end().and_then(index)
    .or(warp::any().and_then(not_found));

  rt.block_on(async {
    repo::open().await.unwrap();
    warp::serve(router)
      .run(([127, 0, 0, 1], 3030))
      .await;
  });
}
