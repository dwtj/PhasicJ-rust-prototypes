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
