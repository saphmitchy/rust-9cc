#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  target/debug/rust-9cc "$input" tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 "0;"
assert 42 "42;"
assert 41 "12 + 34 - 5 ;"
assert 19 "3 * 5 + 4 ;"
assert 60 "3 * 5 * 4 ;"
assert 2 "3 - 5 / 4 ;"
assert 108 "128  - 5 * 4 ;"
assert 42 "16 + (5 * 5 / 2) + 5 * 4 - 6;"
assert 11 "16 - +5;"
assert 12 "  (-16 + 10) * -2;"
assert 7 "+3 - -4;"
assert 1 "-3 - -4;"
assert 0 "0 == 4;"
assert 1 "3 == 3;"
assert 0 "2 == 1;"
assert 1 "0 != 4;"
assert 0 "3 != 3;"
assert 1 "2 != 1;"
assert 1 "0 < 4;"
assert 0 "3 < 3;"
assert 0 "2 < 1;"
assert 1 "0 <= 4;"
assert 1 "3 <= 3;"
assert 0 "2 <= 1;"
assert 0 "0 > 4;"
assert 0 "3 > 3;"
assert 1 "2 > 1;"
assert 0 "0 >= 4;"
assert 1 "3 >= 3;"
assert 1 "2 >= 1;"
assert 7 "(3 <= 3 * 3 - 6 == 1) + 2 * 3;"
assert 1 "(3 > 3 != 1) + 2 == 3;"
assert 1 "3 == 4 != 1;"
assert 0 "3 > 4 > 0;"
assert 0 "q1 = 0;"
assert 3 "q1 = 3; q1;"
assert 2 "q1 = 3; (q1 < 5) + 1;"
assert 1 "q1 = 5; q1 + -4;"
assert 16 "ab = 3; bc = 4; cd = bc + ab * 4;"

echo OK