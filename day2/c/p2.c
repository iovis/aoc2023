#include "p2.h"
#include "parser.h"

#include <stdint.h>

uint64_t p2(const char *input) {
  uint64_t total = 0;
  const char *line = input;

  while (*line) {
    GameParserResult result = parse_max_colors_from_game(line);
    if (!result.ok) break;

    total += result.red * result.green * result.blue;

    line = result.rest;
  }

  return total;
}

#ifdef TEST
#include <assert.h>

void p2_tests(void) {
  const char *input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n"
                      "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n"
                      "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n"
                      "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n"
                      "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n";

  assert(p2(input) == 2286);
}
#endif
