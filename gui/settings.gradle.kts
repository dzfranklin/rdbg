pluginManagement {
    repositories {
        gradlePluginPortal()
        mavenCentral()
        maven("https://plugins.gradle.org/m2/")
        maven { url = uri("https://maven.pkg.jetbrains.space/public/p/compose/dev") }
    }

    // See <https://github.com/google/protobuf-gradle-plugin/blob/master/examples/exampleKotlinDslProject/settings.gradle.kts>
    resolutionStrategy {
        eachPlugin {
            if (requested.id.id == "com.google.protobuf") {
                useModule("com.google.protobuf:protobuf-gradle-plugin:${requested.version}")
            }
        }
    }
}
rootProject.name = "rdbg"

