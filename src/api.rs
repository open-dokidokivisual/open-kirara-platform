/* coding: utf-8 */
/******************************************************************************
 * open-kirara-platform
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/

use warp::{Filter, Reply, Rejection};
use warp::reply::Json;

use crate::repo;

async fn titles() -> Result<Json, Rejection> {
  let repo = repo::open().await;
  let reply = repo.read_titles().await.unwrap();
  Ok(warp::reply::json(&reply))
}

async fn index() -> Result<Box<dyn Reply>, Rejection> {
  Err(warp::reject::reject())
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Sized + Clone {
  warp::path::end().and_then(index)
    .or(warp::path("titles").and_then(titles))
}
