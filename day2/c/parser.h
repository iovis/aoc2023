#pragma once

#include <stdint.h>

typedef struct {
  uint16_t game_id;
  uint16_t red;
  uint16_t green;
  uint16_t blue;
  const char *rest;
  bool ok;
} GameParserResult;

GameParserResult parse_max_colors_from_game(const char *line);

#ifdef TEST
void parser_tests(void);
#endif
