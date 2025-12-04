#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include "include/fmt/format.h"
#include "years/2025/src/dial/dial.h"
#include "years/2025/src/files/files.h"

int main(int argc, char* argv[]) {
  if (argc != 3) {
    std::cout << "Need 3 parameters: program name (not used), input_file, output_path" << std::endl;
    return 1;
  };
  std::cout << "Reading input file" << std::endl;

  std::string input_file_path = argv[1];
  std::vector<std::string> lines = ReadFileToStrings(input_file_path);

  std::vector<dial::DialTurn> turns = dial::ParseDialTurnStrings(lines);

  dial::Dial d = dial::Dial(99, 50, 0);
  d.SetCheckerType(dial::PasswordCheckerType::PASS_ZERO);
  d.TurnDial(turns);

  std::string output_file_path = argv[2];
  std::ofstream output_file(output_file_path);

  output_file << d.GetPasswordCount();
}
