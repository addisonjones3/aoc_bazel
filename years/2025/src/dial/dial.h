#include <string>
#include <vector>

namespace dial {

enum class TurnDirection { LEFT, RIGHT };

struct DialTurn {
  TurnDirection direction;
  int distance;
};

std::string TurnDirectionStr(TurnDirection direction);
DialTurn ParseStrDialTurn(std::string);
std::vector<DialTurn> ParseDialTurnStrings(std::vector<std::string> lines);

enum class PasswordCheckerType { ON_ZERO, PASS_ZERO };

class Dial {
 private:
  int dial_positition_;
  int max_position_;
  int count_positions_;
  int password_position_;
  int password_count_ = 0;
  PasswordCheckerType checker_type_ = PasswordCheckerType::ON_ZERO;

 public:
  Dial(int max_position, int initial_positition, int password_position);
  void TurnDial(DialTurn turn);
  void TurnDial(std::vector<DialTurn> turns);
  void SetCheckerType(PasswordCheckerType type);
  int GetPosition();
  int GetPasswordCount();
};

}  // namespace dial
