#include "parser.h"

#include "base.h"
#include "stb_ds.h"

#include <stddef.h>
#include <stdint.h>
#include <string.h>

static bool is_digit(const char c) {
  return c >= '0' && c <= '9';
}

// Returns 0 on not found
static uint64_t is_number_literal(const char *line) {
  if (strncmp(line, "one", 3) == 0) return 1;
  if (strncmp(line, "two", 3) == 0) return 2;
  if (strncmp(line, "three", 5) == 0) return 3;
  if (strncmp(line, "four", 4) == 0) return 4;
  if (strncmp(line, "five", 4) == 0) return 5;
  if (strncmp(line, "six", 3) == 0) return 6;
  if (strncmp(line, "seven", 5) == 0) return 7;
  if (strncmp(line, "eight", 5) == 0) return 8;
  if (strncmp(line, "nine", 4) == 0) return 9;

  return 0;
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

uint64_t *buffer = nullptr; // set capacity too?
ParseNumberResult parse_number_literals(const char *line) {
  ParseNumberResult result = {.ok = false, .rest = line};
  if (!*line) return result;

  // reset buffer
  arrsetlen(buffer, 0);

  const char *ptr = line;
  uint64_t n = 0;

  while (*ptr && *ptr != '\n') {
    if (is_digit(*ptr)) {
      arrpush(buffer, *ptr - '0');
      ptr++;
      continue;
    }

    n = is_number_literal(ptr);
    if (n) arrpush(buffer, n);
    ptr++;
  }

  result.value = 10 * buffer[0] + buffer[arrlen(buffer) - 1];
  result.ok = true;
  result.rest = *ptr ? ptr + 1 : ptr;

  return result;
}

#ifdef TEST
#include <assert.h>

static void parse_number_test(void) {
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

static void parse_number_literals_test(void) {
  const char *input = "two1nine\n"
                      "eightwothree\n"
                      "abcone2threexyz\n"
                      "xtwone3four\n"
                      "4nineeightseven2\n"
                      "zoneight234\n"
                      "7pqrstsixteen\n";

  ParseNumberResult result = parse_number_literals(input);
  assert(result.ok);
  assert(result.value == 29);

  result = parse_number_literals(result.rest);
  assert(result.ok);
  assert(result.value == 83);

  result = parse_number_literals(result.rest);
  assert(result.ok);
  assert(result.value == 13);

  result = parse_number_literals(result.rest);
  assert(result.ok);
  assert(result.value == 24);

  result = parse_number_literals(result.rest);
  assert(result.ok);
  assert(result.value == 42);

  result = parse_number_literals(result.rest);
  assert(result.ok);
  assert(result.value == 14);

  result = parse_number_literals(result.rest);
  assert(result.ok);
  assert(result.value == 76);

  result = parse_number_literals(result.rest);
  assert(!result.ok);
}

void parser_tests(void) {
  parse_number_test();
  parse_number_literals_test();
}
#endif
