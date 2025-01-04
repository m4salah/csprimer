#include <assert.h>
#include <stddef.h>
#include <stdio.h>

int bitcount(size_t n) {
  int count = 0;
  while (n) {
    count += n & 0b1;
    n >>= 1;
  }
  return count;
}

int main(int argc, char *argv[]) {
  assert(bitcount(0) == 0);           // 0b0
  assert(bitcount(1) == 1);           // 0b01
  assert(bitcount(2) == 1);           // 0b10
  assert(bitcount(3) == 2);           // 0b11
  assert(bitcount(8) == 1);           // 0b1000
  assert(bitcount(0xffffffff) == 32); // 0b1000

  printf("Hello, World!\n");
  return 0;
}
