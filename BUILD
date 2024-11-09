load("@crate_index_cargo_workspace//:defs.bzl",
     "all_crate_deps"
)

load("@rules_rust//rust:defs.bzl",
     "rust_binary",
     "rust_library",
     "rust_shared_library",
     "rust_static_library"
)

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "bazel_rust_sample",
    srcs = ["src/lib.rs"],
    edition = "2021",
    proc_macro_deps = all_crate_deps(proc_macro = True),
    deps = all_crate_deps(normal = True),
)

rust_binary(
    name = "bazel_rust_sample_bin",
    srcs = ["src/main.rs"],
    edition = "2021",
    proc_macro_deps = all_crate_deps(proc_macro = True),
    deps = all_crate_deps(normal = True) + [":bazel_rust_sample"],
)
