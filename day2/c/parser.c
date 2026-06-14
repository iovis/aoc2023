#include "parser.h"

#include "base.h"
#include <stdint.h>

static bool parse_tag(const char **line, const char *tag) {
  const char *ptr = *line;
  while (*tag && *ptr == *tag) {
    tag++;
    ptr++;
  }

  // tag was not consumed
  if (*tag != '\0') return false;

  *line = ptr;
  return true;
}

static bool is_digit(char c) {
  return c >= '0' && c <= '9';
}

static uint16_t parse_u16(const char **line) {
  uint_fast64_t total = 0;

  while (is_digit(**line)) {
    total = 10 * total + **line - '0';
    (*line)++;
  }

  expect(total <= UINT16_MAX);

  return (uint16_t)total;
}

GameParserResult parse_max_colors_from_game(const char *line) {
  GameParserResult result = {.rest = line};
  if (!*line) return result;

  const char *ptr = line;
  expect(parse_tag(&ptr, "Game "));
  result.game_id = parse_u16(&ptr);
  expect(parse_tag(&ptr, ": "));

  while (*ptr && *ptr != '\n') {
    uint16_t n = parse_u16(&ptr);
    expect(n);

    expect(parse_tag(&ptr, " "));

    if (parse_tag(&ptr, "red")) {
      if (n > result.red) result.red = n;
    } else if (parse_tag(&ptr, "green")) {
      if (n > result.green) result.green = n;
    } else if (parse_tag(&ptr, "blue")) {
      if (n > result.blue) result.blue = n;
    } else {
      expect(false, "expected color, got: %s", ptr);
    }

    parse_tag(&ptr, ", ") || parse_tag(&ptr, "; ");
  }

  result.ok = true;
  result.rest = (*ptr == '\n') ? ptr + 1 : ptr;

  return result;
}

#ifdef TEST
#include <assert.h>

static void parse_max_colors_from_game_test(void) {
  const char *input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n"
                      "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n"
                      "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n"
                      "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n"
                      "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n";

  GameParserResult result = parse_max_colors_from_game(input);
  assert(result.ok);
  assert(result.game_id == 1);
  assert(result.red == 4);
  assert(result.green == 2);
  assert(result.blue == 6);

  result = parse_max_colors_from_game(result.rest);
  assert(result.ok);
  assert(result.game_id == 2);
  assert(result.red == 1);
  assert(result.green == 3);
  assert(result.blue == 4);

  result = parse_max_colors_from_game(result.rest);
  assert(result.ok);
  assert(result.game_id == 3);
  assert(result.red == 20);
  assert(result.green == 13);
  assert(result.blue == 6);

  result = parse_max_colors_from_game(result.rest);
  assert(result.ok);
  assert(result.game_id == 4);
  assert(result.red == 14);
  assert(result.green == 3);
  assert(result.blue == 15);

  result = parse_max_colors_from_game(result.rest);
  assert(result.ok);
  assert(result.game_id == 5);
  assert(result.red == 6);
  assert(result.green == 3);
  assert(result.blue == 2);

  result = parse_max_colors_from_game(result.rest);
  assert(!result.ok);
}

void parser_tests(void) {
  parse_max_colors_from_game_test();
}
#endif
