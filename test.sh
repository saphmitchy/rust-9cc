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

assert 0 0
assert 42 42
assert 41 "12 + 34 - 5 "
assert 19 "3 * 5 + 4 "
assert 60 "3 * 5 * 4 "
assert 2 "3 - 5 / 4 "
assert 108 "128  - 5 * 4 "
assert 42 "16 + (5 * 5 / 2) + 5 * 4 - 6"

echo OK