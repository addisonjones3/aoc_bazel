#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include "include/fmt/format.h"

std::vector<std::string> ReadFileToStrings(std::string filepath) {
  std::ifstream input_file(filepath);
  if (!input_file) {
    std::string err = fmt::format("{:s} could not open file\n", filepath);
    std::cerr << err << std::endl;
    // panic basically?
    exit(1);
  };

  std::vector<std::string> lines;
  std::string line_str;
  while ((input_file >> line_str) && line_str != "-1") {
    lines.push_back(line_str);
  }
  return lines;
};
