package org.dokidokivisual.platform.controller


import io.vertx.ext.web.RoutingContext
import io.vertx.kotlin.coroutines.await

public class IndexController {
  companion object {
    suspend fun index(ctx: RoutingContext) {
      val resp = ctx.response()
      resp.end("Hello world!").await()
    }
  }
}
