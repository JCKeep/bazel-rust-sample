load(
    "@bazel_tools//tools/build_defs/repo:http.bzl",
    "http_archive",
)

# load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
# http_archive(
#     name = "rules_rust",
#     integrity = "sha256-r09Wyq5QqZpov845sUG1Cd1oVIyCBLmKt6HK/JTVuwI=",
#     urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.54.1/rules_rust-v0.54.1.tar.gz"],
# )

local_repository(
    name = "rules_rust",
    path = "rules/rules_rust",
)

load(
    "@rules_rust//rust:repositories.bzl",
    "rules_rust_dependencies",
    "rust_register_toolchains",
)

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
)

load(
    "@rules_rust//crate_universe:repositories.bzl",
    "crate_universe_dependencies",
)

crate_universe_dependencies(bootstrap = True)

load(
    "@rules_rust//tools/rust_analyzer:deps.bzl",
    "rust_analyzer_dependencies",
)

rust_analyzer_dependencies()

# load("@rules_rust//bindgen:repositories.bzl",
#      "rust_bindgen_dependencies",
#      "rust_bindgen_register_toolchains"
# )

# rust_bindgen_dependencies()

# rust_bindgen_register_toolchains()

# load("@rules_rust//bindgen:transitive_repositories.bzl",
#      "rust_bindgen_transitive_dependencies"
# )

# rust_bindgen_transitive_dependencies()

load(
    "@rules_rust//crate_universe:defs.bzl",
    "crate",
    "crates_repository",
    "render_config",
    "splicing_config",
)

crates_repository(
    name = "crate_index_cargo_workspace",
    cargo_config = "//:.cargo/config.toml",
    cargo_lockfile = "//:Cargo.lock",
    generator = "@cargo_bazel_bootstrap//:cargo-bazel",
    manifests = [
        "//:Cargo.toml",
    ],
)

load(
    "@crate_index_cargo_workspace//:defs.bzl",
    cargo_local_crate_repositories = "crate_repositories",
)

cargo_local_crate_repositories()
