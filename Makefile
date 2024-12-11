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
	diff q1.out test-c/q1.exp && echo "Q1 Test passed"; rm q1.out; rm q1.o|| echo "Q1 Passed"
q2: src-c/q2.c
	$(CC) $(CFLAGS) $< -o $@.o
q2run: q2
	./q2.o
q2test: q2
	./q2.o > q2.out
	diff q2.out test-c/q2.exp && echo "Q2 Test passed"; rm q2.out; q2.0 || echo "Q2 Passed"