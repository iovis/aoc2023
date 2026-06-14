#pragma once

#include <stdio.h>  // IWYU pragma: keep (fprintf)
#include <stdlib.h> // IWYU pragma: keep (abort)

typedef float f32_t;
typedef double f64_t;

#define panic(fmt, ...)                                                                                                \
  do {                                                                                                                 \
    fprintf(stderr, "[%s:%d:%s] panic: " fmt "\n", __FILE__, __LINE__, __func__ __VA_OPT__(, ) __VA_ARGS__);           \
    abort();                                                                                                           \
  } while (0)

#define expect(cond, ...)                                                                                              \
  do {                                                                                                                 \
    if (!(cond)) {                                                                                                     \
      fprintf(stderr, "[%s:%d:%s] panic: expectation failed: %s", __FILE__, __LINE__, __func__, #cond);                \
      __VA_OPT__(fprintf(stderr, ": "); fprintf(stderr, __VA_ARGS__);)                                                 \
      fputc('\n', stderr);                                                                                             \
      abort();                                                                                                         \
    }                                                                                                                  \
  } while (0)

#ifndef NDEBUG
#define dbg(fmt, ...)                                                                                                  \
  do {                                                                                                                 \
    fprintf(stderr, "[%s:%d:%s] " fmt "\n", __FILE__, __LINE__, __func__ __VA_OPT__(, ) __VA_ARGS__);                  \
  } while (0)
#else
#define dbg(...)                                                                                                       \
  do {                                                                                                                 \
  } while (0)
#endif
