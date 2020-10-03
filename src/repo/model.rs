/* coding: utf-8 */
/******************************************************************************
 * open-kirara-platform
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/

use serde::Serialize;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Title {
  pub id: i64,
  pub revision: i64,
  pub name: String,
  pub description: String,
}
