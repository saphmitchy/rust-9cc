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

assert 0 "int main(){ return 0; }"
assert 42 "int main(){ return 42; }"
assert 41 "int main(){ return 12 + 34 - 5 ; }"
assert 19 "int main(){ return 3 * 5 + 4 ; }"
assert 60 "int main(){ return 3 * 5 * 4 ; }"
assert 2 "int main(){ return 3 - 5 / 4 ; }"
assert 108 "int main(){ return 128  - 5 * 4 ; }"
assert 42 "int main(){ return 16 + (5 * 5 / 2) + 5 * 4 - 6; }"
assert 11 "int main(){ return 16 - +5; }"
assert 12 "int main(){   return (-16 + 10) * -2; }"
assert 7 "int main(){ return +3 - -4; }"
assert 1 "int main(){ return -3 - -4; }"
assert 0 "int main(){ return 0 == 4; }"
assert 1 "int main(){ return 3 == 3; }"
assert 0 "int main(){ return 2 == 1; }"
assert 1 "int main(){ return 0 != 4; }"
assert 0 "int main(){ return 3 != 3; }"
assert 1 "int main(){ return 2 != 1; }"
assert 1 "int main(){ return 0 < 4; }"
assert 0 "int main(){ return 3 < 3; }"
assert 0 "int main(){ return 2 < 1; }"
assert 1 "int main(){ return 0 <= 4; }"
assert 1 "int main(){ return 3 <= 3; }"
assert 0 "int main(){ return 2 <= 1; }"
assert 0 "int main(){ return 0 > 4; }"
assert 0 "int main(){ return 3 > 3; }"
assert 1 "int main(){ return 2 > 1; }"
assert 0 "int main(){ return 0 >= 4; }"
assert 1 "int main(){ return 3 >= 3; }"
assert 1 "int main(){ return 2 >= 1; }"
assert 7 "int main(){ return (3 <= 3 * 3 - 6 == 1) + 2 * 3; }"
assert 1 "int main(){ return (3 > 3 != 1) + 2 == 3; }"
assert 1 "int main(){ return 3 == 4 != 1; }"
assert 0 "int main(){ return 3 > 4 > 0; }"
assert 0 "int main(){ int q1; return q1 = 0; }"
assert 3 "int main(){ int q1; q1 = 3; return q1; }"
assert 2 "int main(){ int q1; q1 = 3; return (q1 < 5) + 1; }"
assert 1 "int main(){ int q1; q1 = 5; return q1 + -4; }"
assert 16 "int main(){ int ab; int bc; int cd; ab = 3; bc = 4; return cd = bc + ab * 4; }"
assert 13 "int main(){ return 10 + 3; }"
assert 5 "int main(){ 1; 2; return 5; }"
assert 5 "int main(){ 1; return 5; 2; }"
assert 2 "int main(){ int a; a = 1; return a + 1; }"
assert 7 "int main(){ if(1 + 2 == 3) return 7; return 5; }"
assert 5 "int main(){ if(1 + 2 < 3) return 7; return 5; }"
assert 12 "int main(){ int a; a = 9; if(a == 9) a = a + 3; else a = a + 1; return a; }"
assert 10 "int main(){ int a; a = 9; if(a != 9) a = a + 3; else a = a + 1; return a; }"
assert 7 "int main(){ int ab; int cd; ab = 3; {cd = ab + 3; ab = cd + 1; } return ab; }"
assert 13 "int main(){ int a; a = 9; if(a == 9) { a = a + 3; a = a + 1; } return a; }"
assert 9 "int main(){ int a; a = 9; if(a != 9) { a = a + 3; a = a + 1; } return a; }"
assert 10 "int main(){ int a; a = 1; while (a < 10) { a = a + 1; } return a; }"
assert 11 "int main(){ int a; a = 1; while (a < 10) { if(a + 1 == 10) { a = a + 2; } else { a = a + 1; } } return a; }"
assert 120 "int main(){ int a; int i; a = 1; for(i = 1; i <= 5; i = i + 1) a = a * i; return a; }"
assert 240 "int main(){ int a; int i; a = 2; i = 1; for(; i <= 5;) { a = a * i;  i = i + 1; } return a; }"
assert 3 "int main(){ for(;;) { return 3; } }"
assert 42 "int main(){ return test(); }"
assert 14 "int main(){ int hoge; hoge = test2(3, 4); return 2 * hoge; }"
assert 21 "int main(){ int a; int hoge; a = 2; hoge = test6(3 + 1, a, 3 * 1 - 2, 0, test2(3, 9), a); return hoge; }"
assert 3 "int f() { return 3; }int  main() { return f(); }"
assert 63 "int f() { return 3; } int g(int x) { return x * 9; } int main() { int a; a = 4 + f(); return g(a); }"
assert 7 "int f(int a, int b, int c, int d, int e, int f) { int x; a = a * b; x = c * d; return x + a + e / f; } int main() { int x; x = 0; return f(x, 3 + 4, x + 1, 4, 6, 2); }"
assert 13 "int f(int n) { if(n <= 1) return 1; else return f(n - 2) + f(n - 1); } int main() { return f(6); }"
assert 5 "int main() { int a; int* b; a = 5; b = &a; return *b; }"
assert 10 "int main() { int a; int* b; int** c; int*** d; a = 5; b = &a; c = &b; d = &c; return test2(a, ***d); }"
assert 2 "int main() {int a; int ba; a = 5; ba = 2; return *(&a-8); }" # this is implemention defined
assert 5 "int main() { int a; int ba; a = 5; ba = 2; return *(&ba+8); }" # this is implemention defined
assert 42 "int main() {int a; int *b; a = 3; b = &a; *b = 42; return a; } "
assert 21 "int main() {int a; int *b; int **c; a = 3; b = &a; c = &b; **c = 21; return *(&a); } "
assert 5 "int main() { int a; int *b; a = 2; b = &a; **(&b) = 5; return a; } "
assert 91 "int f(int* a) { return *a; } int main() { int a; a = 91; return f(&a); } "
assert 3 "int main() {int a; int ba; ba = 2; *(&a-8) = 3; return ba; }" # this is implemention defined
assert 5 "int main() { int a; int ba; a = 4; *(&ba+8) = 5; return a; }" # this is implemention defined

echo OK