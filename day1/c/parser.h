#pragma once

#include <stdint.h>

typedef struct {
  uint64_t value;
  const char *rest;
  bool ok;
} ParseNumberResult;

ParseNumberResult parse_number(const char *line);
ParseNumberResult parse_number_literals(const char *line);

#ifdef TEST
void parser_tests(void);
#endif
