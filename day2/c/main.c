#include "lib.h"

#include "p1.h"
#include "p2.h"

#include <stdio.h>

static const char input[] = {
#embed "../input.txt" suffix(, )
    '\0'
};

int main(void) {
  printf("p1 = %lu\n", p1(input));
  printf("p2 = %lu\n", p2(input));

  return 0;
}
