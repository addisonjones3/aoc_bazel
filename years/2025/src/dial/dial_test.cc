#include "dial.hpp"

#include <gtest/gtest.h>

#include <algorithm>
#include <string>
#include <vector>

TEST(DialTest, BasicAssertions) {
  dial::DialTurn expected_turn;
  expected_turn.distance = -48;
  expected_turn.direction = dial::TurnDirection::LEFT;

  dial::DialTurn good_turn = dial::ParseStrDialTurn("L48");

  EXPECT_EQ(good_turn.distance, expected_turn.distance);
  EXPECT_EQ(good_turn.direction, expected_turn.direction);
};

TEST(DialTurn, TestTurnDial) {
  dial::Dial d = dial::Dial(99, 0, 0);
  dial::DialTurn turn_1 = {dial::TurnDirection::RIGHT, 1};

  int expected_position = 1;

  d.TurnDial(turn_1);

  EXPECT_EQ(d.GetPosition(), expected_position);

  expected_position = 98;
  dial::DialTurn turn_neg_3 = {dial::TurnDirection::LEFT, -3};
  d.TurnDial(turn_neg_3);

  EXPECT_EQ(d.GetPosition(), expected_position);

  expected_position = 1;
  dial::DialTurn turn_pos_3 = {dial::TurnDirection::RIGHT, 3};
  d.TurnDial(turn_pos_3);

  EXPECT_EQ(d.GetPosition(), expected_position);
}

TEST(DialTurnMoreThanMax, TestTurnDialOverMax) {
  dial::Dial d = dial::Dial(99, 50, 0);
  dial::DialTurn turn = {dial::TurnDirection::RIGHT, 150};
  d.TurnDial(turn);

  int expected_position = 0;
  EXPECT_EQ(d.GetPosition(), expected_position);
}

TEST(DialTurnVec, TestTurnDailVecOfTurns) {
  dial::Dial d = dial::Dial(99, 0, 0);
  dial::DialTurn turn_1 = {dial::TurnDirection::RIGHT, 1};

  std::vector<dial::DialTurn> turns = {turn_1, turn_1};
  d.TurnDial(turns);

  EXPECT_EQ(d.GetPosition(), 2);
}

TEST(DialIncrementsPasswordCount, TestIncrementPassword) {
  dial::Dial d = dial::Dial(99, 0, 0);
  dial::DialTurn turn_1 = {dial::TurnDirection::RIGHT, 1};
  dial::DialTurn turn_neg_1 = {dial::TurnDirection::LEFT, -1};

  d.TurnDial(turn_1);
  EXPECT_EQ(d.GetPasswordCount(), 0);
  EXPECT_EQ(d.GetPosition(), 1);
  d.TurnDial(turn_neg_1);
  EXPECT_EQ(d.GetPasswordCount(), 1);
  EXPECT_EQ(d.GetPosition(), 0);
}

TEST(PassZeroTest, TestPassZero) {
  dial::Dial d = dial::Dial(99, 50, 0, dial::PasswordCheckerType::PASS_ZERO);

  dial::DialTurn turn_1 = {dial::TurnDirection::LEFT, -68};
  d.TurnDial(turn_1);

  EXPECT_EQ(d.GetPasswordCount(), 1);
  EXPECT_EQ(d.GetPosition(), 82);

  d.TurnDial(dial::DialTurn{dial::TurnDirection::LEFT, -30});

  EXPECT_EQ(d.GetPasswordCount(), 1);
  EXPECT_EQ(d.GetPosition(), 52);

  d.TurnDial(dial::DialTurn{dial::TurnDirection::RIGHT, 48});

  EXPECT_EQ(d.GetPasswordCount(), 2);
  EXPECT_EQ(d.GetPosition(), 0);
}
