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

assert 0 "main(){ return 0; }"
assert 42 "main(){ return 42; }"
assert 41 "main(){ return 12 + 34 - 5 ; }"
assert 19 "main(){ return 3 * 5 + 4 ; }"
assert 60 "main(){ return 3 * 5 * 4 ; }"
assert 2 "main(){ return 3 - 5 / 4 ; }"
assert 108 "main(){ return 128  - 5 * 4 ; }"
assert 42 "main(){ return 16 + (5 * 5 / 2) + 5 * 4 - 6; }"
assert 11 "main(){ return 16 - +5; }"
assert 12 "main(){   return (-16 + 10) * -2; }"
assert 7 "main(){ return +3 - -4; }"
assert 1 "main(){ return -3 - -4; }"
assert 0 "main(){ return 0 == 4; }"
assert 1 "main(){ return 3 == 3; }"
assert 0 "main(){ return 2 == 1; }"
assert 1 "main(){ return 0 != 4; }"
assert 0 "main(){ return 3 != 3; }"
assert 1 "main(){ return 2 != 1; }"
assert 1 "main(){ return 0 < 4; }"
assert 0 "main(){ return 3 < 3; }"
assert 0 "main(){ return 2 < 1; }"
assert 1 "main(){ return 0 <= 4; }"
assert 1 "main(){ return 3 <= 3; }"
assert 0 "main(){ return 2 <= 1; }"
assert 0 "main(){ return 0 > 4; }"
assert 0 "main(){ return 3 > 3; }"
assert 1 "main(){ return 2 > 1; }"
assert 0 "main(){ return 0 >= 4; }"
assert 1 "main(){ return 3 >= 3; }"
assert 1 "main(){ return 2 >= 1; }"
assert 7 "main(){ return (3 <= 3 * 3 - 6 == 1) + 2 * 3; }"
assert 1 "main(){ return (3 > 3 != 1) + 2 == 3; }"
assert 1 "main(){ return 3 == 4 != 1; }"
assert 0 "main(){ return 3 > 4 > 0; }"
assert 0 "main(){ return q1 = 0; }"
assert 3 "main(){ q1 = 3; return q1; }"
assert 2 "main(){ q1 = 3; return (q1 < 5) + 1; }"
assert 1 "main(){ q1 = 5; return q1 + -4; }"
assert 16 "main(){ ab = 3; bc = 4; return cd = bc + ab * 4; }"
assert 13 "main(){ return 10 + 3; }"
assert 5 "main(){ 1; 2; return 5; }"
assert 5 "main(){ 1; return 5; 2; }"
assert 2 "main(){ a = 1; return a + 1; }"
assert 7 "main(){ if(1 + 2 == 3) return 7; return 5; }"
assert 5 "main(){ if(1 + 2 < 3) return 7; return 5; }"
assert 12 "main(){ a = 9; if(a == 9) a = a + 3; else a = a + 1; return a; }"
assert 10 "main(){ a = 9; if(a != 9) a = a + 3; else a = a + 1; return a; }"
assert 7 "main(){ ab = 3; {cd = ab + 3; ab = cd + 1; } return ab; }"
assert 13 "main(){ a = 9; if(a == 9) { a = a + 3; a = a + 1; } return a; }"
assert 9 "main(){ a = 9; if(a != 9) { a = a + 3; a = a + 1; } return a; }"
assert 10 "main(){ a = 1; while (a < 10) { a = a + 1; } return a; }"
assert 11 "main(){ a = 1; while (a < 10) { if(a + 1 == 10) { a = a + 2; } else { a = a + 1; } } return a; }"
assert 120 "main(){ a = 1; for(i = 1; i <= 5; i = i + 1) a = a * i; return a; }"
assert 240 "main(){ a = 2; i = 1; for(; i <= 5;) { a = a * i;  i = i + 1; } return a; }"
assert 3 "main(){ for(;;) { return 3; } }"
assert 42 "main(){ return test(); }"
assert 14 "main(){ hoge = test2(3, 4); return 2 * hoge; }"
assert 21 "main(){ a = 2; hoge = test6(3 + 1, a, 3 * 1 - 2, 0, test2(3, 9), a); return hoge; }"
assert 3 "f() { return 3; } main() { return f(); }"
assert 63 "f() { return 3; } g(x) { return x * 9; } main() { a = 4 + f(); return g(a); }"
assert 7 "f(a, b, c, d, e, f) { a = a * b; x = c * d; return x + a + e / f; }  main() { x = 0; return f(x, 3 + 4, x + 1, 4, 6, 2); }"
assert 13 "f(n) { if(n <= 1) return 1; else return f(n - 2) + f(n - 1); } main() { return f(6); }"

echo OK