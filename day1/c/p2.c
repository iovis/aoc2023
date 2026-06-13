#include "p2.h"
#include "parser.h"
#include <stdint.h>

uint64_t p2(const char *input) {
  uint64_t total = 0;
  const char *line = input;

  while (true) {
    ParseNumberResult result = parse_number_literals(line);
    if (!result.ok) break;

    total += result.value;
    line = result.rest;
  }

  return total;
}

#ifdef TEST
#include <assert.h>

void p2_tests(void) {
  const char *input = "two1nine\n"
                      "eightwothree\n"
                      "abcone2threexyz\n"
                      "xtwone3four\n"
                      "4nineeightseven2\n"
                      "zoneight234\n"
                      "7pqrstsixteen\n";

  assert(p2(input) == 281);
}
#endif
