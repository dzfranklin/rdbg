import org.jetbrains.compose.compose
import org.jetbrains.compose.desktop.application.dsl.TargetFormat
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import com.google.protobuf.gradle.*

plugins {
    kotlin("jvm") version "1.5.10"
    id("org.jetbrains.compose") version "0.4.0-build211"
    id("com.google.protobuf") version "0.8.16"
}

group = "me.daniel"
version = "1.0.0"

repositories {
    jcenter()
    mavenCentral()
    maven { url = uri("https://maven.pkg.jetbrains.space/public/p/compose/dev") }
}

dependencies {
    testImplementation(kotlin("test-junit"))
    implementation(compose.desktop.currentOs)

    // proto, see <https://github.com/google/protobuf-gradle-plugin/blob/master/examples/exampleKotlinDslProject/build.gradle.kts>
    implementation("com.google.protobuf:protobuf-java:3.17.1")
    implementation("io.grpc:grpc-stub:1.38.0")
    implementation("io.grpc:grpc-protobuf:1.38.0")
    if (JavaVersion.current().isJava9Compatible) {
        // Workaround for @javax.annotation.Generated
        // see: https://github.com/grpc/grpc-java/issues/3633
        implementation("javax.annotation:javax.annotation-api:1.3.1")
    }
}

tasks.test {
    useJUnit()
}

tasks.withType<KotlinCompile>() {
    kotlinOptions.jvmTarget = "11"
}

compose.desktop {
    application {
        mainClass = "MainKt"
        nativeDistributions {
            targetFormats(TargetFormat.Dmg, TargetFormat.Msi, TargetFormat.Deb)
            packageName = "rdbg"
        }
    }
}

sourceSets {
    main {
        proto {
            srcDir("../protos/")
        }
        java {
            srcDirs("build/generated/source/proto/main/grpc/org/danielzfranklin/rdbg")
            srcDirs("build/generated/source/proto/main/java/org/danielzfranklin/rdbg")
        }
    }
}

protobuf {
    protoc {
        // The artifact spec for the Protobuf Compiler
        artifact = "com.google.protobuf:protoc:3.6.1"
    }
    plugins {
        // Optional: an artifact spec for a protoc plugin, with "grpc" as
        // the identifier, which can be referred to in the "plugins"
        // container of the "generateProtoTasks" closure.
        id("grpc") {
            artifact = "io.grpc:protoc-gen-grpc-java:1.15.1"
        }
    }
    generateProtoTasks {
        ofSourceSet("main").forEach {
            it.plugins {
                // Apply the "grpc" plugin whose spec is defined above, without options.
                id("grpc")
            }
        }
    }
}
