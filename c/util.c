#include <stdio.h>
#include <string.h>

int print_launch_error(void) {
    printf("Provide a day to run\n");
    return 0;
}

int print_invalid_day(void) {
    printf("Provide a valid day to run\n");
    return 0;
}

int determine_day(char* argv[]) {
  int ret = 0;
  if (strcmp("one", argv[1]) == 0 || strcmp("1", argv[1]) == 0) {
    ret = 1;
  }
  return ret;
}
