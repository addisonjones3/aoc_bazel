#include "dial.hpp"

#include <cstdlib>
#include <iostream>
#include <iterator>
#include <ostream>
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

DialTurn ParseStrDialTurn(std::string str) {
  std::string digits_str = str.substr(1, str.length());
  DialTurn turn;
  switch (str[0]) {
    case 'L':
      turn.direction = TurnDirection::LEFT;
      turn.distance = -std::stoi(digits_str);
      break;
    case 'R':
      turn.direction = TurnDirection::RIGHT;
      turn.distance = std::stoi(digits_str);
      break;
    default:
      std::string err = fmt::format("{:s} not a valid turn format\n", str);
      throw std::invalid_argument(err);
  };

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

Dial::Dial(int max_position, int initial_positition, int password_position, PasswordCheckerType checker_type) {
  max_position_ = max_position;
  dial_positition_ = initial_positition;
  count_positions_ = max_position_ + 1;
  password_position_ = password_position;
  checker_type_ = checker_type;
};

void Dial::TurnDial(DialTurn turn) {
  // std::cout << "initial dial position: " << dial_positition_ << std::endl;
  int initial_position = dial_positition_;

  // std::cout << "turning " << turn.distance << " " << turn.direction << std::endl;
  // std::cout << "turn.distance % count_positions_ " << turn.distance % count_positions_ << std::endl;

  int count_rotations = std::abs(turn.distance / count_positions_);
  // std::cout << "count_rotations: " << count_rotations << std::endl;

  int next_position;
  next_position = (dial_positition_ + (turn.distance % count_positions_));
  // std::cout << "next_position " << next_position << std::endl;
  int abs_position = dial_positition_ + turn.distance;
  // std::cout << "abs_position " << abs_position << std::endl;

  if (next_position < 0) {
    dial_positition_ = next_position + count_positions_;
  } else if (next_position >= count_positions_) {
    dial_positition_ = next_position - count_positions_;
  } else {
    dial_positition_ = next_position;
  }

  // std::cout << "ending dial position: " << dial_positition_ << std::endl;

  int count_clicks = 0;
  switch (turn.direction) {
    case TurnDirection::RIGHT:
      count_clicks += abs_position / count_positions_;
      break;
    case TurnDirection::LEFT:
      if (abs_position <= 0) {
        if (initial_position > 0) {
          count_clicks++;
        }
        count_clicks += count_rotations;
      }
      break;
  }

  // std::cout << count_clicks << " click(s)" << std::endl;
  switch (checker_type_) {
    case PasswordCheckerType::PASS_ZERO:
      if (count_clicks > 0) {
        std::cout << "incrementing password " << count_clicks << " clicks" << std::endl;
      }
      password_count_ += count_clicks;
      break;
    case PasswordCheckerType::ON_ZERO:
      if (dial_positition_ == 0) {
        std::cout << "incrementing password count on 0" << std::endl;
        password_count_++;
      }
      break;
  }

  // std::cout << std::endl;
};

void Dial::TurnDial(std::vector<DialTurn> turns) {
  for (DialTurn turn : turns) {
    TurnDial(turn);
  }
};

int Dial::GetPosition() { return dial_positition_; };
int Dial::GetPasswordCount() { return password_count_; };

}  // namespace dial
