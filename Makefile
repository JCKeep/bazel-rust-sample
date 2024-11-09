all:
	CARGO_BAZEL_ISOLATED=false bazel build //:bazel_rust_sample_bin