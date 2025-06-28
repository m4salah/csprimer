#include <assert.h>
#include <stdio.h>

extern int fact(int n);

int main(void) {
  assert(fact(0) == 1);
  assert(fact(1) == 1);
  assert(fact(2) == 2);
  assert(fact(3) == 6);
  assert(fact(4) == 24);
  assert(fact(5) == 120);
  assert(fact(6) == 720);
  assert(fact(7) == 5040);
  assert(fact(8) == 40320);
  assert(fact(9) == 362880);
  assert(fact(10) == 3628800);
  assert(fact(11) == 39916800);
  printf("OK\n");
}
