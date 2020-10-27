import org.gradle.api.tasks.testing.logging.TestLogEvent.*
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
  kotlin ("jvm") version "1.4.10"
  java
  id("com.github.johnrengelman.shadow") version "6.1.0"
  application
}

group = "com.open-dokidokivisual"
version = "NaN"

repositories {
  mavenCentral()
  jcenter()
}

val kotlinVersion = "1.4.10"
val vertxVersion = "4.0.0.Beta3"
val junitJupiterVersion = "5.6.2"

val mainVerticleName = "com.open_dokidokivisual.platform.MainVerticle"
val watchForChange = "src/**/*"
val doOnChange = "./gradlew classes"
val launcherClassName = "io.vertx.core.Launcher"

application {
  mainClassName = launcherClassName
  applicationDefaultJvmArgs = listOf(
    "-Xms2G",
    "-Xmx4G",
    "-server",
    "-XX:+UseNUMA",
    "-XX:+UseParallelGC",
    "-XX:AutoBoxCacheMax=20000",
    "-XX:+AlwaysPreTouch",
    "--add-exports=java.base/jdk.internal.misc=ALL-UNNAMED",
    "-Dfile.encoding=UTF-8",
    "-Dvertx.logger-delegate-factory-class-name=io.vertx.core.logging.Log4j2LogDelegateFactory"
  )
}

dependencies {
  implementation("io.vertx:vertx-lang-kotlin:$vertxVersion")
  implementation("io.vertx:vertx-lang-kotlin-coroutines:$vertxVersion")
  implementation("io.vertx:vertx-mysql-client:$vertxVersion")
  implementation("io.vertx:vertx-codegen:$vertxVersion")
  implementation("io.vertx:vertx-web:$vertxVersion")
  testImplementation("io.vertx:vertx-junit5:$vertxVersion")
  testImplementation("org.junit.jupiter:junit-jupiter:$junitJupiterVersion")
}

val compileKotlin: KotlinCompile by tasks
compileKotlin.kotlinOptions.jvmTarget = "13"

tasks {
  withType<JavaExec> {
    //args = listOf("run", mainVerticleName, "--redeploy=$watchForChange", "--launcher-class=$launcherClassName", "--on-redeploy=$doOnChange")
    args = listOf("run", mainVerticleName, "--launcher-class=$launcherClassName")
  }
  test {
    useJUnitPlatform()
  }
  named<com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar>("shadowJar") {
    version = ""
    setProperty("archiveClassifier", "")
    manifest.attributes(mapOf("Main-Verticle" to mainVerticleName))
    mergeServiceFiles().include("META-INF/services/io.vertx.core.spi.VerticleFactory")
  }
}

