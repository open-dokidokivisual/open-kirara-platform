/* coding: utf-8 */
/******************************************************************************
 * open-kirara-platform
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/

use sqlx::prelude::*;

pub async fn open() -> sqlx::Result<impl Connection> {
  sqlx::sqlite::SqliteConnection::connect("sqlite:").await
}
