WORKSPACE_NAME = "rustcxx_test"

workspace(name = WORKSPACE_NAME)

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive", "http_file")

# Rust

http_archive(
    name = "rules_rust",
    sha256 = "4def898e2e65e5fce37009dc33ee57187ac96ff14fb5720e168103b883bc77c0",
    strip_prefix = "rules_rust-b3e26e0c6e02bebd954135f2ce4dd578fc6d94c0",
    urls = [
        # Master branch as of 2021-11-05
        "https://github.com/bazelbuild/rules_rust/archive/b3e26e0c6e02bebd954135f2ce4dd578fc6d94c0.tar.gz",
    ],
)

load("//3rdparty/rust/cargo:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories(
    edition = "2021",
    version = "1.56.1",
)
