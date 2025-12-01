#include <fstream>
#include <iostream>

int main(int argc, char *argv[]) {
  if (argc != 3) {
    std::cout << "Need 3 parameters: program name (not used), input_file, output_path" << std::endl;
    return 1;
  };
  std::cout << "Reading input file" << std::endl;

  std::string input_file_path = argv[1];

  std::ifstream input_file(input_file_path);
  if (!input_file) {
    std::cout << "Bad input file" << std::endl;
  }
  std::string line_str;
  int line_count = 0;
  while (getline(input_file, line_str)) {
    std::cout << line_str << std::endl;
    line_count++;
  }

  input_file.close();

  std::string output_file_path = argv[2];
  std::cout << "Writing test output to file " << output_file_path << std::endl;
  std::ofstream output_file(output_file_path);
  output_file << line_count;
  output_file.close();
}
