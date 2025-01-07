#include <ctype.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

bool ispangram(char *s) {
  //              abcdefghijklmnopqrstuvwxyz
  uint32_t bs = 0b00000000000000000000000000000000;

  size_t n = strlen(s);
  for (int i = 0; i < n; i++) {
    // if it's uppercase.
    char c = tolower(s[i]);
    if (c < 123 && c > 96) {
      bs |= (1 << (32 - (s[i] - 96)));
    }
  }
  return bs == 0b11111111111111111111111111000000;
}

int main() {
  size_t len;
  ssize_t read;
  char *line = NULL;
  while ((read = getline(&line, &len, stdin)) != -1) {
    if (ispangram(line)) {
      printf("%s", line);
    }
  }

  if (ferror(stdin)) {
    fprintf(stderr, "Error reading from stdin");
  }

  free(line);
  fprintf(stderr, "ok\n");
}
