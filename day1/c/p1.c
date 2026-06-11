#include "p1.h"
#include "parser.h"
#include <stdint.h>

uint64_t p1(const char *input) {
  uint64_t total = 0;
  const char *line = input;

  while (true) {
    ParseNumberResult result = parse_number(line);
    if (!result.ok) break;

    total += result.value;
    line = result.rest;
  }

  return total;
}

#ifdef TEST
#include <assert.h>

void p1_tests(void) {
  const char *input = "1abc2\n"
                      "pqr3stu8vwx\n"
                      "a1b2c3d4e5f\n"
                      "treb7uchet\n";

  assert(p1(input) == 142);
}
#endif
