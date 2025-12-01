#include <fstream>
#include <iostream>

int main(int argc, char *argv[]) {
  if (argc != 2) {
    std::cout << "Need file path arg" << std::endl;
    return 1;
  };

  std::ifstream inputFile(argv[1]);
  std::string lineStr;
  while (getline(inputFile, lineStr)) {
    std::cout << lineStr << std::endl;
  }

  inputFile.close();
}
