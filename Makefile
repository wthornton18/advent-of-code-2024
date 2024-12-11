CC=gcc
CFLAGS=-O3 -pedantic -Wall -Wextra -Werror -std=c99

all: q1

test_all: q1test

q1: src-c/q1.c
	$(CC) $(CFLAGS) $< -o $@.o
q1run: q1
	./q1.o
q1test: q1
	./q1.o > q1.out
	diff q1.out test-c/q1.exp && echo "Q1 Test passed"; rm q1.out || echo "Q1 Passed"