#include "product.hpp"

#include <gtest/gtest.h>

#include <string>

TEST(TestParseID, BasicParse) {
  std::string basic = "11-22";

  product::ProductID p = product::ParseStrProduct(basic);

  EXPECT_EQ(p.begin, 11);
  EXPECT_EQ(p.end, 22);
}

TEST(TestParseID, VecParse) {
  std::string list = "11-22,44-66";

  std::vector<product::ProductID> v = product::ParseStrProducts(list);

  EXPECT_EQ(v[0].begin, 11);
  EXPECT_EQ(v[0].end, 22);

  EXPECT_EQ(v[1].begin, 44);
  EXPECT_EQ(v[1].end, 66);
}
