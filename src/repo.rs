/* coding: utf-8 */
/******************************************************************************
 * open-kirara-platform
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/

use std::sync::{Arc, RwLock};
use sqlx::prelude::*;

pub mod model;

lazy_static! {
  static ref STORAGE: RwLock<Option<Arc<Repo>>> = RwLock::new(None);
}

#[derive(Debug)]
pub struct Repo {
  conn: sqlx::Pool<sqlx::SqliteConnection>,
}

pub fn get() -> Arc<Repo> {
  STORAGE.read().as_ref().unwrap().as_ref().unwrap().clone()
}
pub async fn open(url: &str) -> sqlx::Result<Arc<Repo>> {
  let mut w = STORAGE.write().unwrap();
  let conn = sqlx::SqlitePool::new(url).await?;
  let repo = Repo{
    conn,
  };
  *w = Some(Arc::new(repo));
  Ok(w.as_ref().unwrap().clone())
}

static SCHEMA: &'static str = r###############"
create table if not exists titles (
  `id` integer primary key autoincrement,
  `revision` integer not null,
  `name` text not null,
  `description` text not null
);
"###############;

impl Repo {
  pub async fn init(&self) -> sqlx::Result<()> {
    sqlx::query(SCHEMA).execute(&self.conn).await?;
    Ok(())
  }
  pub async fn save_title(&self, title: &model::Title) -> sqlx::Result<i64> {
    Ok(0)
  }
  pub async fn read_titles(&self) -> sqlx::Result<Vec<model::Title>> {
    sqlx::query_as::<sqlx::Sqlite, model::Title>(r"select * from titles").fetch_all(&self.conn).await
  }
}
