workspace(name = "phasicj")

# RULES FOR GETTING EXTERNAL DEPENDENCIES #####################################

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# CONFIGURE BAZEL_SKYLIB ######################################################

# Note: `@io_bazel_rules_rust` depends upon this starlark helper library.
# Note: This version was chosen because it is the latest (pre-)release as of 2020-03-09.
BAZEL_SKYLIB_VERSION = "1.0.2"
BAZEL_SKYLIB_SHA256 = "97e70364e9249702246c0e9444bccdc4b847bed1eb03c5a3ece4f83dfe6abc44"
http_archive(
    name = "bazel_skylib",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/{}/bazel-skylib-{}.tar.gz".format(BAZEL_SKYLIB_VERSION, BAZEL_SKYLIB_VERSION),
        "https://github.com/bazelbuild/bazel-skylib/releases/download/{}/bazel-skylib-{}.tar.gz".format(BAZEL_SKYLIB_VERSION, BAZEL_SKYLIB_VERSION),
    ],
    sha256 = BAZEL_SKYLIB_SHA256,
)

# CONFIGURE RULES_RUST ########################################################

# Master commit as of 2020-03-09
RULES_RUST_COMMIT = "fe50d3b54aecbaeac48abdc2ca7cd00a94969e15"
RULES_RUST_SHA256 = "3f6db529492d821a91560c230e2634e34b3e0a3147fc5c4c41ac5bc6ccd45d3f"

http_archive(
    name = "io_bazel_rules_rust",
    sha256 = RULES_RUST_SHA256,
    strip_prefix = "rules_rust-{}".format(RULES_RUST_COMMIT),
    url = "https://github.com/bazelbuild/rules_rust/archive/{}.tar.gz".format(RULES_RUST_COMMIT),
)

# Run the bazel version check that comes with `rules_rust`:
load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")

# Use a more recent stable Rust release.
RUST_VERSION = "1.42.0"
RUSTFMT_VERSION = "1.4.11"
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repository_set")
rust_repository_set(
    name = "rust_linux_x86_64",
    exec_triple = "x86_64-unknown-linux-gnu",
    extra_target_triples = [],
    version = RUST_VERSION,
    rustfmt_version = RUSTFMT_VERSION,
)

# Fetch remote cargo-raze crates.
load("//third_party/cargo:crates.bzl", "raze_fetch_remote_crates")
raze_fetch_remote_crates() 

# CONFIGURE RULES_GRAALVM #####################################################

RULES_GRAALVM_COMMIT = "ca52548f8c6a29b0ff67d18e659560595505b4d7"
RULES_GRAALVM_SHA256 = "22fa03e5cf07ee10ae4e8455b4a38c090c932a647e7d1f96a5090469d3b0362b"

http_archive(
    name = "rules_graalvm",
    sha256 = RULES_GRAALVM_SHA256,
    strip_prefix = "rules_graalvm-{}".format(RULES_GRAALVM_COMMIT),
    url = "https://github.com/dwtj/rules_graalvm/archive/{}.zip".format(RULES_GRAALVM_COMMIT),
)

load("@rules_graalvm//graalvm:repositories.bzl",
     "rules_graalvm_dependencies",
     "rules_graalvm_toolchains")

rules_graalvm_dependencies()
rules_graalvm_toolchains()

# CONFIGURE RULES_JVM_EXTERNAL TO GET MAVEN CENTRAL DEPENDENCIES ##############

# This is the latest `rules_jvm_external` release as of 2020-04-14:
RULES_JVM_EXTERNAL_TAG = "3.2"

RULES_JVM_EXTERNAL_SHA256 = "82262ff4223c5fda6fb7ff8bd63db8131b51b413d26eb49e3131037e79e324af"

http_archive(
    name = "rules_jvm_external",
    sha256 = RULES_JVM_EXTERNAL_SHA256,
    strip_prefix = "rules_jvm_external-{}".format(RULES_JVM_EXTERNAL_TAG),
    url = "https://github.com/bazelbuild/rules_jvm_external/archive/{}.zip".format(RULES_JVM_EXTERNAL_TAG),
)

load("@rules_jvm_external//:defs.bzl", "maven_install")

ORG_GRAALVM_TRUFFLE_VERSION = "20.0.0"

maven_install(
    artifacts = [
        "org.graalvm.truffle:truffle-api:{}".format(ORG_GRAALVM_TRUFFLE_VERSION),
        "org.graalvm.sdk:graal-sdk:{}".format(ORG_GRAALVM_TRUFFLE_VERSION),
    ],
    fetch_sources = True,
    repositories = [
        "https://jcenter.bintray.com/",
        "https://maven.google.com",
        "https://repo1.maven.org/maven2",
    ],
)