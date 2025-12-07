load("@rules_cc//cc:defs.bzl", "cc_library", "cc_test")

package(default_visibility = ["//visibility:public"])

cc_library(
    name = "product",
    srcs = [
        "product.cc",
    ],
    hdrs = ["product.hpp"],
)

cc_test(
    name = "product_test",
    srcs = ["product_test.cc"],
    deps = [
        ":product",
        "@googletest//:gtest",
        "@googletest//:gtest_main",
    ],
)
