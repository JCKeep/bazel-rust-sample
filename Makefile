all:
	CARGO_BAZEL_ISOLATED=false bazel build //:bazel_rust_sample_bin
	CARGO_BAZEL_ISOLATED=false bazel run //:bazel_rust_sample_bin