#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  target/debug/rust-9cc "$input" tmp.s
  cc -o tmp tmp.s test.o
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

gcc -c test.c

assert 0 "return 0;"
assert 42 "return 42;"
assert 41 "return 12 + 34 - 5 ;"
assert 19 "return 3 * 5 + 4 ;"
assert 60 "return 3 * 5 * 4 ;"
assert 2 "return 3 - 5 / 4 ;"
assert 108 "return 128  - 5 * 4 ;"
assert 42 "return 16 + (5 * 5 / 2) + 5 * 4 - 6;"
assert 11 "return 16 - +5;"
assert 12 "  return (-16 + 10) * -2;"
assert 7 "return +3 - -4;"
assert 1 "return -3 - -4;"
assert 0 "return 0 == 4;"
assert 1 "return 3 == 3;"
assert 0 "return 2 == 1;"
assert 1 "return 0 != 4;"
assert 0 "return 3 != 3;"
assert 1 "return 2 != 1;"
assert 1 "return 0 < 4;"
assert 0 "return 3 < 3;"
assert 0 "return 2 < 1;"
assert 1 "return 0 <= 4;"
assert 1 "return 3 <= 3;"
assert 0 "return 2 <= 1;"
assert 0 "return 0 > 4;"
assert 0 "return 3 > 3;"
assert 1 "return 2 > 1;"
assert 0 "return 0 >= 4;"
assert 1 "return 3 >= 3;"
assert 1 "return 2 >= 1;"
assert 7 "return (3 <= 3 * 3 - 6 == 1) + 2 * 3;"
assert 1 "return (3 > 3 != 1) + 2 == 3;"
assert 1 "return 3 == 4 != 1;"
assert 0 "return 3 > 4 > 0;"
assert 0 "return q1 = 0;"
assert 3 "q1 = 3; return q1;"
assert 2 "q1 = 3; return (q1 < 5) + 1;"
assert 1 "q1 = 5; return q1 + -4;"
assert 16 "ab = 3; bc = 4; return cd = bc + ab * 4;"
assert 13 "return 10 + 3;"
assert 5 "1; 2; return 5;"
assert 5 "1; return 5; 2;"
assert 2 "a = 1; return a + 1;"
assert 7 "if(1 + 2 == 3) return 7; return 5;"
assert 5 "if(1 + 2 < 3) return 7; return 5;"
assert 12 "a = 9; if(a == 9) a = a + 3; else a = a + 1; return a;"
assert 10 "a = 9; if(a != 9) a = a + 3; else a = a + 1; return a;"
assert 7 "ab = 3; {cd = ab + 3; ab = cd + 1; } return ab;"
assert 13 "a = 9; if(a == 9) { a = a + 3; a = a + 1; } return a;"
assert 9 "a = 9; if(a != 9) { a = a + 3; a = a + 1; } return a;"
assert 10 "a = 1; while (a < 10) { a = a + 1; } return a;"
assert 11 "a = 1; while (a < 10) { if(a + 1 == 10) { a = a + 2; } else { a = a + 1; } } return a;"
assert 120 "a = 1; for(i = 1; i <= 5; i = i + 1) a = a * i; return a;"
assert 240 "a = 2; i = 1; for(; i <= 5;) { a = a * i;  i = i + 1; } return a;"
assert 3 "for(;;) { return 3; }"
assert 42 "return test();"
assert 14 "hoge = test2(3, 4); return 2 * hoge;"
assert 21 "a = 2; hoge = test6(3 + 1, a, 3 * 1 - 2, 0, test2(3, 9), a); return hoge;"

echo OK