#include <stdio.h>
#include <string.h>
#include "one.h"

int main(int argc, char* argv[]) {
    int ret;
  if (argc != 2) {
    printf("Provide a day to run\n");
    ret = 0;
  } else {
    if ( strcmp("one", argv[1]) == 0 || strcmp("1", argv[1]) == 0) {
      ret = one();
    } else {
      printf("Who what now?\n");
      ret = 0;
    }
  }
  return ret;
}
