load("@rules_cc//cc:defs.bzl", "cc_binary")
load("//aoc:defs.bzl", "day_run")

exports_files(["sample_input.txt"])

cc_binary(
    name = "day1",
    srcs = [":day1.cc"],
)

day_run(
    name = "day1_run",
    input = ":sample_input.txt",
    solver = ":day1",
)
