/* coding: utf-8 */
/******************************************************************************
 * open-kirara-platform
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/

#[macro_use]
extern crate lazy_static;

use warp::Filter;
use warp::reply::Reply;
use warp::reject::Rejection;
use warp::http::uri;
use std::str::FromStr;

mod web;
mod api;
mod repo;
use web::template::{*};
use std::sync::Arc;

async fn render<T: yarte::Template>(renderer: T) -> Result<impl Reply, Rejection> {
  //let (mut sender, body) = warp::hyper::body::Body::channel();
  //warp::reply::Response::new(body);
  match renderer.call() {
    Ok(body) => Ok(warp::reply::html(body)),
    Err(err) => Err(warp::reject::custom::<TemplateError>(err.into())),
  }
}

async fn not_found() -> Result<impl Reply, Rejection> {
  Ok(warp::redirect(uri::Uri::from_str("/").unwrap()))
}

async fn init_repo() -> sqlx::Result<Arc<repo::Repo>> {
  let repo = repo::open_with("sqlite:").await?;
  repo.init().await?;
  Ok(repo)
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  let mut rt = tokio::runtime::Builder::new()
    .core_threads(num_cpus::get() + 1)
    .threaded_scheduler()
    .enable_all()
    .build()
    .unwrap();

  let repo = rt.block_on(async {
    let repo = init_repo().await;
    if repo.is_err() {
      return Err(string_error::into_err(repo.unwrap_err().to_string()));
    }
    let repo = repo.unwrap();
    repo.init().await?;
    Ok(repo)
  });
  if repo.is_err() {
    return Err(repo.unwrap_err());
  }

  let router =
    warp::path::end().and(warp::fs::file("fe/static/index.html"))
      .or(warp::path("dist").and(warp::fs::dir("fe/dist")))
      .or(warp::path("static").and(warp::fs::dir("fe/static")))
      .or(warp::path("api").and(api::route()))
      .or(warp::any().and_then(not_found));

  rt.block_on(async move {
    warp::serve(router)
      .run(([127, 0, 0, 1], 3030))
      .await
  });
  Ok(())
}
