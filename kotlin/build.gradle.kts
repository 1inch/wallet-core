// Workaround https://github.com/gradle/gradle/issues/22797
@file:Suppress("DSL_SCOPE_VIOLATION")

import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    id("com.android.application") version libs.versions.agp.get() apply false
    id("com.android.library") version libs.versions.agp.get() apply false
    kotlin("android") version libs.versions.kotlin.get() apply false
    kotlin("multiplatform") version libs.versions.kotlin.get() apply false
}

allprojects {
    tasks.withType<KotlinCompile> {
        compilerOptions {
            allWarningsAsErrors.set(true)
            jvmTarget.set(JvmTarget.JVM_17)
            freeCompilerArgs.add("-Xexpect-actual-classes")
        }
    }
}

// Pin patched transitive npm versions in the Kotlin/JS yarn.lock.
// Dual-major packages (brace-expansion, minimatch) are updated in yarn.lock
// within their existing ranges rather than forced to a single version here.
rootProject.plugins.withType<org.jetbrains.kotlin.gradle.targets.js.yarn.YarnPlugin> {
    rootProject.the<org.jetbrains.kotlin.gradle.targets.js.yarn.YarnRootExtension>().apply {
        resolution("webpack", "5.104.1")
        resolution("lodash", "4.18.0")
        resolution("decode-uri-component", "0.5.0")
        resolution("engine.io", "6.6.7")
        resolution("nanoid", "3.3.18")
        resolution("js-yaml", "4.3.1")
        resolution("socket.io-parser", "4.2.7")
        resolution("body-parser", "1.20.6")
        resolution("ws", "8.21.0")
        resolution("tmp", "0.2.6")
        resolution("serialize-javascript", "7.0.5")
        resolution("picomatch", "2.3.2")
        resolution("flatted", "3.4.2")
        resolution("qs", "6.14.2")
        resolution("diff", "5.2.2")
        resolution("cookie", "0.7.2")
        resolution("cross-spawn", "7.0.6")
        resolution("braces", "3.0.3")
        resolution("follow-redirects", "1.16.0")
        resolution("rollup", "2.80.0")
    }
}
