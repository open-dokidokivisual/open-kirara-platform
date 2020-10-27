package org.dokidokivisual.platform

import io.vertx.core.impl.logging.LoggerFactory
import io.vertx.ext.web.Route
import io.vertx.ext.web.Router
import io.vertx.ext.web.RoutingContext
import io.vertx.kotlin.coroutines.CoroutineVerticle
import io.vertx.kotlin.coroutines.await
import kotlinx.coroutines.launch
import org.dokidokivisual.platform.controller.IndexController

class MainVerticle : CoroutineVerticle() {
  private val log = LoggerFactory.getLogger(MainVerticle::class.java)
  override suspend fun start() {
    log.info("listen at %d".format(8080))
    val http = vertx
      .createHttpServer()
      .requestHandler(createRouter())
      .listen(8080) { result ->
        if (result.succeeded()) {
          log.info(String.format("今日も一日がんばるぞい！ @ [Listen at %s]", ":8080"))
        } else {
          log.info("今日は全くがんばれない…")
          result.cause().printStackTrace()
        }
      }
  }

  private fun createRouter() = Router.router(vertx).apply {
    handle(get("/"), IndexController::index)
  }

  // Kotlin CoroutineからHandlerに変換する、ある意味核心部分。
  private fun handle(route: Route, fn: suspend (RoutingContext) -> Unit) {
    route.handler { ctx ->
      launch {
        try {
          fn(ctx)
          if(!ctx.response().ended()) {
            ctx.response().end()
          }
        } catch (e: Throwable) {
          log.error("Unknown error", e)
          ctx.fail(e)
        }
      }
    }
  }

}
