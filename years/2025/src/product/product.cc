#include "product.hpp"

#include <sstream>
#include <string>
#include <vector>

namespace product {
std::vector<ProductID> ParseStrProducts(const std::string& str) {
  std::vector<ProductID> product_ids;

  std::vector<std::string> id_strings;
  std::stringstream ss(str);
  std::string id_string;

  while (std::getline(ss, id_string, ',')) {
    product_ids.push_back(ParseStrProduct(id_string));
  }

  return product_ids;
}

ProductID ParseStrProduct(const std::string& str) {
  ProductID p;
  int delim_position = str.find('-');

  p.begin = std::stoi(str.substr(0, delim_position));
  p.end = std::stoi(str.substr(delim_position + 1, str.length()));
  return p;
}
}  // namespace product
