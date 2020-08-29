/* coding: utf-8 */
/******************************************************************************
 * open-kirara-platform
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/


extern crate yarte;
extern crate warp;

use yarte::{Template};
pub use yarte::TemplateTrait;

#[derive(Debug)]
pub struct TemplateError(pub yarte::Error);

impl warp::reject::Reject for TemplateError {
}

impl From<yarte::Error> for TemplateError {
  fn from(err: yarte::Error) -> Self {
    TemplateError(err)
  }
}

#[derive(Template)]
#[template(path = "index.hbs")]
pub struct Index {
}
