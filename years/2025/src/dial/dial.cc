#include "dial.h"

#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

#include "include/fmt/format.h"

namespace dial {

std::ostream& operator<<(std::ostream& os, TurnDirection const& td) {
  switch (td) {
    case TurnDirection::LEFT:
      return os << "LEFT";
    case TurnDirection::RIGHT:
      return os << "RIGHT";
  };
};

std::string TurnDirectionStr(TurnDirection direction) {
  switch (direction) {
    case TurnDirection::LEFT:
      return "LEFT";
    case TurnDirection::RIGHT:
      return "RIGHT";
  }
};

DialTurn ParseStrDialTurn(std::string str) {
  DialTurn turn;
  switch (str[0]) {
    case 'L':
      turn.direction = TurnDirection::LEFT;
      break;
    case 'R':
      turn.direction = TurnDirection::RIGHT;
      break;
    default:
      std::string err = fmt::format("{:s} not a valid turn format\n", str);
      throw std::invalid_argument(err);
  };

  std::string digits_str = str.substr(1, str.length());
  int distance = std::stoi(digits_str);
  turn.distance = distance;

  return turn;
};

std::vector<DialTurn> ParseDialTurnStrings(std::vector<std::string> lines) {
  std::vector<DialTurn> turns;
  std::string line_str;
  for (std::string line : lines) {
    DialTurn turn = ParseStrDialTurn(line);
    turns.push_back(turn);
  };
  return turns;
};

Dial::Dial(int max_position, int initial_positition, int password_position) {
  max_position_ = max_position;
  dial_positition_ = initial_positition;
  count_positions_ = max_position_ + 1;
  password_position_ = password_position;
};

void Dial::TurnDial(DialTurn turn) {
  std::cout << "starting dial position is " << dial_positition_ << std::endl;
  int start_position = dial_positition_;
  int new_position = dial_positition_;
  int floor_distance = turn.distance % count_positions_;
  int count_passes = 0;
  int rel_position = 0;

  switch (turn.direction) {
    case dial::TurnDirection::RIGHT:
      new_position += floor_distance;
      std::cout << "turning " << turn.distance << " " << turn.direction << std::endl;
      rel_position = start_position + turn.distance;
      break;
    case dial::TurnDirection::LEFT:
      new_position -= floor_distance;
      std::cout << "turning " << turn.distance << " " << turn.direction << std::endl;
      rel_position = start_position - turn.distance;
      break;
  };

  std::cout << "new_position: " << new_position << std::endl;
  std::cout << "rel_position: " << rel_position << std::endl;

  if (new_position < 0) {
    count_passes++;
    count_passes += std::abs(rel_position) / count_positions_;
  };

  std::cout << "count passes: " << count_passes << std::endl;

  if (new_position < 0) {
    dial_positition_ = new_position + count_positions_;
  } else if (new_position > max_position_) {
    dial_positition_ = new_position - count_positions_;
  } else {
    dial_positition_ = new_position;
  };

  if (dial_positition_ > max_position_ || dial_positition_ < 0) {
    std::cout << "dial position is now " << dial_positition_ << " and something went really wrong" << std::endl;
  };
  std::cout << "dial position is now " << dial_positition_ << std::endl;
  if (dial_positition_ == password_position_) {
    std::cout << "should increment password count" << std::endl;
    password_count_++;
  };
  if (checker_type_ == PasswordCheckerType::PASS_ZERO) {
    password_count_ += count_passes;
  };
};

void Dial::TurnDial(std::vector<DialTurn> turns) {
  for (DialTurn turn : turns) {
    TurnDial(turn);
  }
};

void Dial::SetCheckerType(PasswordCheckerType type) { checker_type_ = type; };

int Dial::GetPosition() { return dial_positition_; };
int Dial::GetPasswordCount() { return password_count_; };

}  // namespace dial
