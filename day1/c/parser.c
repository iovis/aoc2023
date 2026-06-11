#include "parser.h"

#include "base.h"

#include <stddef.h>
#include <stdint.h>
#include <string.h>

static bool is_digit(const char c) {
  return c >= '0' && c <= '9';
}

ParseNumberResult parse_number(const char *line) {
  ParseNumberResult result = {.ok = false, .rest = line};
  if (!*line) return result;

  // first digit
  const char *ptr = line;
  while (*ptr && *ptr != '\n') {
    if (is_digit(*ptr)) {
      result.value = *ptr - '0';
      break;
    }

    ptr++;
  }

  // second digit
  const char *end = strchrnul(line, '\n');
  expect(end != line);
  ptr = end - 1;

  while (*ptr && *ptr != '\n') {
    if (is_digit(*ptr)) {
      result.value = 10 * result.value + *ptr - '0';
      break;
    }

    ptr--;
  }

  result.ok = true;
  result.rest = *end ? end + 1 : end;

  return result;
}

#ifdef TEST
#include <assert.h>

void parser_tests(void) {
  const char *input = "1abc2\n"
                      "pqr3stu8vwx\n"
                      "a1b2c3d4e5f\n"
                      "treb7uchet\n";

  ParseNumberResult result = parse_number(input);
  assert(result.ok);
  assert(result.value == 12);

  result = parse_number(result.rest);
  assert(result.ok);
  assert(result.value == 38);

  result = parse_number(result.rest);
  assert(result.ok);
  assert(result.value == 15);

  result = parse_number(result.rest);
  assert(result.ok);
  assert(result.value == 77);

  result = parse_number(result.rest);
  assert(result.ok == false);
}
#endif
