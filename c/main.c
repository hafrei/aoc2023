#include "one.h"
#include "util.h"

int main(int argc, char* argv[]) {
  int ret;
  if (argc != 2) {
    ret = print_launch_error();
  } else {
    int day = determine_day(argv);
    if (day == 1) {
      ret = one();
    } else {
      ret = print_invalid_day();
    }
  }
  return ret;
}
