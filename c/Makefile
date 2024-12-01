all: aoc 

aoc: main.o one.o util.o
	gcc main.o one.o util.o -o aoc #-v

main.o: main.c
	gcc -Wall -Wextra -Werror -O2 -std=c99 -pedantic -c main.c -o main.o

one.o: one.c
	gcc -Wall -Wextra -Werror -O2 -std=c99 -pedantic -c one.c -o one.o

util.o: util.c
	gcc -Wall -Wextra -Werror -O2 -std=c99 -pedantic -c util.c -o util.o

clean:
	rm aoc main main.o a.out one.o

