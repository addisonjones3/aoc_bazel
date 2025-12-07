#include <iostream>
#include <string>
#include <vector>

#include "years/2025/src/dial/dial.h"
#include "years/2025/src/files/files.h"

int main(int argc, char* argv[]) {
  std::cout << "Reading input file" << std::endl;

  std::string input_file_path = argv[1];
  std::vector<std::string> lines = ReadFileToStrings(input_file_path);

  std::vector<dial::DialTurn> turns = dial::ParseDialTurnStrings(lines);

  dial::Dial d = dial::Dial(99, 50, 0, dial::PasswordCheckerType::PASS_ZERO);
  d.TurnDial(turns);

  std::cout << "password count: " << d.GetPasswordCount() << std::endl;
  std::cout << (20 - 160) << std::endl;
  std::cout << (20 - 60) % 100 << std::endl;
}
