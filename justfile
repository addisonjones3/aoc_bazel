aoc_files_repo_name := "aoc_files"

cargo +ARGS:
	bazel run @rules_rust//tools/upstream_wrapper:cargo -- {{ARGS}}

cargo-clippy *ARGS:
	bazel run @rules_rust//tools/upstream_wrapper:cargo_clippy -- {{ARGS}}

rustc +ARGS:
	bazel run @rules_rust//tools/upstream_wrapper:rustc -- {{ARGS}}

gazelle:
	bazel run //:gazelle

test +ARGS:
	bazel test {{ARGS}}

generate-rust-project:
	bazel run @rules_rust//tools/rust_analyzer:gen_rust_project

generate-day year day: 
	bazel run @multitool//tools/cargo-generate:cwd -- generate --path ./day-template --name day{{day}} --destination years/{{year}}/src --no-workspace

generate-year year:
	bazel run @multitool//tools/cargo-generate:cwd -- generate --path ./year-template --name {{year}} --destination years --define=aoc-files-repo-name={{aoc_files_repo_name}}

alias g := gazelle
