#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STARTING_CAPACITY 8
#define CAPACITY_GROWTH 2

typedef struct DA {
  void **data;
  size_t size;
  size_t capacity;
} DA;

DA *DA_new(void) {
  DA *da = malloc(sizeof(DA));
  if (da == NULL) {
    return NULL;
  }
  da->size = 0;
  da->capacity = STARTING_CAPACITY;

  da->data = malloc(STARTING_CAPACITY * sizeof(void *));
  if (da->data == NULL) {
    free(da);
    return NULL;
  }
  return da;
}

int DA_size(DA *da) { return da->size; }

void DA_push(DA *da, void *x) {
  if (da->size == da->capacity) {
    da->capacity *= CAPACITY_GROWTH;
    da->data = realloc(da->data, da->capacity * sizeof(void *));
  }
  da->data[da->size++] = x;
}

void *DA_pop(DA *da) {
  if (da->size == 0) {
    return NULL;
  }
  return da->data[--da->size];
}

void DA_set(DA *da, void *x, int i) {
  if (da->size < i) {
    return;
  }
  da->data[i] = x;
}

void *DA_get(DA *da, int i) {
  if (da->size < i) {
    return NULL;
  }
  return da->data[i];
}

void DA_free(DA *da) {
  free(da->data);
  free(da);
}

void DA_print(DA *da) {
  for (size_t i = 0; i < da->size; i++) {
    printf("%ld: %p\n", i, DA_get(da, i));
  }
}

int main() {
  DA *da = DA_new();

  assert(DA_size(da) == 0);

  // basic push and pop test
  int x = 5;
  float y = 12.4;
  DA_push(da, &x);
  DA_push(da, &y);
  assert(DA_size(da) == 2);

  assert(DA_pop(da) == &y);
  assert(DA_size(da) == 1);

  assert(DA_pop(da) == &x);
  assert(DA_size(da) == 0);
  assert(DA_pop(da) == NULL);

  // basic set/get test
  DA_push(da, &x);
  DA_set(da, &y, 0);
  assert(DA_get(da, 0) == &y);
  DA_pop(da);
  assert(DA_size(da) == 0);

  // expansion test
  DA *da2 = DA_new(); // use another DA to show it doesn't get overriden
  DA_push(da2, &x);
  int i, n = 100 * STARTING_CAPACITY, arr[n];
  for (i = 0; i < n; i++) {
    arr[i] = i;
    DA_push(da, &arr[i]);
  }
  assert(DA_size(da) == n);
  for (i = 0; i < n; i++) {
    assert(DA_get(da, i) == &arr[i]);
  }
  for (; n; n--)
    DA_pop(da);
  assert(DA_size(da) == 0);
  assert(DA_pop(da2) == &x); // this will fail if da doesn't expand

  DA_free(da);
  DA_free(da2);
  printf("OK\n");
}
