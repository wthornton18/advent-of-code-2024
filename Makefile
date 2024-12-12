CC=gcc
CFLAGS=-O3 -pedantic -Wall -Wextra -Werror -std=c99 -lm
TEST_PASSED = PASSED ✅
TEST_FAILED = FAILED ❌

all: q1

test_all: q1test q2test q3test q4test q5test

q1: src-c/q1.c
	$(CC) $(CFLAGS) $< -o $@.o
q1run: q1
	./q1.o
	rm q1.o
q1test: q1
	./q1.o > q1.out
	diff q1.out test-c/q1.exp && echo "Q1 ${TEST_PASSED}" || echo "Q1 ${TEST_FAILED}"
	rm q1.out
	rm q1.o
q1gen_test: q1
	./q1.o > test-c/q1.exp
q2: src-c/q2.c
	$(CC) $(CFLAGS) $< -o $@.o
q2run: q2
	./q2.o
	rm q2.o
q2test: q2
	./q2.o > q2.out
	diff q2.out test-c/q2.exp && echo "Q2 ${TEST_PASSED}" || echo "Q2 ${TEST_FAILED}"
	rm q2.out
	rm q2.o
q2gen_test: q2
	./q2.o > test-c/q2.exp
	rm q2.o
q3: src-c/q3.c
	$(CC) $(CFLAGS) $< -o $@.o
q3run: q3
	./q3.o
	rm q3.o
q3test: q3
	./q3.o > q3.out
	diff q3.out test-c/q3.exp && echo "Q3 ${TEST_PASSED}" || echo "Q3 ${TEST_FAILED}"
	rm q3.out
	rm q3.o
q3gen_test: q3
	./q3.o > test-c/q3.exp
	rm q3.o
q4: src-c/q4.c
	$(CC) $(CFLAGS) $< -o $@.o
q4run: q4
	./q4.o
	rm q4.o
q4test: q4
	./q4.o > q4.out
	diff q4.out test-c/q4.exp && echo "Q4 ${TEST_PASSED}" || echo "Q4 ${TEST_FAILED}"
	rm q4.out
	rm q4.o
q4gen_test: q4
	./q4.o > test-c/q4.exp
	rm q4.o
q5: src-c/q5.c
	$(CC) $(CFLAGS) $< -o $@.o
q5run: q5
	./q5.o
	rm q5.o
q5test: q5
	./q5.o > q5.out
	diff q5.out test-c/q5.exp && echo "Q5 ${TEST_PASSED}" || echo "Q5 ${TEST_FAILED}"
	rm q5.out
	rm q5.o
q5gen_test: q5
	./q5.o > test-c/q5.exp
	rm q5.o