load("@rules_cc//cc:defs.bzl", "cc_library", "cc_test")

package(default_visibility = ["//visibility:public"])

cc_library(
    name = "dial",
    srcs = [
        "dial.cc",
    ],
    hdrs = ["dial.h"],
    deps = [
        "@fmt",
    ],
)

cc_test(
    name = "dial_test",
    srcs = ["dial_test.cc"],
    deps = [
        ":dial",
        "@googletest//:gtest",
        "@googletest//:gtest_main",
    ],
)
