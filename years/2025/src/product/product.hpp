#include <string>
#include <vector>

namespace product {

struct ProductID {
  int begin;
  int end;
};

std::vector<ProductID> ParseStrProducts(const std::string& str);
ProductID ParseStrProduct(const std::string& str);

}  // namespace product
