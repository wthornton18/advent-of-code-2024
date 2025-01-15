CC=gcc
CFLAGS=-O3 -Wall -Wextra -std=gnu99 -lm
TEST_PASSED = PASSED ✅
TEST_FAILED = FAILED ❌

all: q1

test_all: q1test q2test q3test q4test q5test q6test q7test q8test q10test

q1: src-c/q1.c
	$(CC) $(CFLAGS) $< -o $@.o
q1run: q1
	./q1.o
	rm q1.o
q1test: q1
	./q1.o > q1.out
	diff q1.out test-c/q1.exp && echo "Q1 ${TEST_PASSED}" || (echo "Q1 ${TEST_FAILED}" && exit 1)
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
	diff q2.out test-c/q2.exp && echo "Q2 ${TEST_PASSED}" || (echo "Q2 ${TEST_FAILED}" && exit 1)
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
	diff q3.out test-c/q3.exp && echo "Q3 ${TEST_PASSED}" || (echo "Q3 ${TEST_FAILED}" && exit 1)
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
	diff q4.out test-c/q4.exp && echo "Q4 ${TEST_PASSED}" || (echo "Q4 ${TEST_FAILED}" && exit 1)
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
	diff q5.out test-c/q5.exp && echo "Q5 ${TEST_PASSED}" || (echo "Q5 ${TEST_FAILED}" && exit 1)
	rm q5.out
	rm q5.o
q5gen_test: q5
	./q5.o > test-c/q5.exp
	rm q5.o
q6: src-c/q6.c
	$(CC) $(CFLAGS) $< -o $@.o
q6run: q6
	./q6.o
	rm q6.o
q6test: q6
	./q6.o > q6.out
	diff q6.out test-c/q6.exp && echo "Q6 ${TEST_PASSED}" || (echo "Q6 ${TEST_FAILED}" && exit 1)
	rm q6.out
	rm q6.o
q6gen_test: q6
	./q6.o > test-c/q6.exp
	rm q6.o
q7: src-c/q7.c
	$(CC) $(CFLAGS) $< -o $@.o
q7run: q7
	./q7.o
	rm q7.o
q7test: q7
	./q7.o > q7.out
	diff q7.out test-c/q7.exp && echo "Q7 ${TEST_PASSED}" || (echo "Q7 ${TEST_FAILED}" && exit 1)
	rm q7.out
	rm q7.o
q7gen_test: q7
	./q7.o > test-c/q7.exp
	rm q7.o
q8: src-c/q8.c
	$(CC) $(CFLAGS) $< -o $@.o
q8run: q8
	./q8.o
	rm q8.o
q8test: q8	
	./q8.o > q8.out
	diff q8.out test-c/q8.exp && echo "Q8 ${TEST_PASSED}" || (echo "Q8 ${TEST_FAILED}" && exit 1)
	rm q8.out
	rm q8.o
q8gen_test: q8
	./q8.o > test-c/q8.exp
	rm q8.o
q10: src-c/q10.c
	$(CC) $(CFLAGS) $< -o $@.o
q10run: q10
	./q10.o
	rm q10.o
q10test: q10
	./q10.o > q10.out
	diff q10.out test-c/q10.exp && echo "Q10 ${TEST_PASSED}" || (echo "Q10 ${TEST_FAILED}" && exit 1)
	rm q10.out
	rm q10.o
q10gen_test: q10
	./q10.o > test-c/q10.exp
	rm q10.o
q11: src-c/q11.c
	$(CC) $(CFLAGS) $< -o $@.o
q11run: q11
	./q11.o
	rm q11.o
q11test: q11
	./q11.o > q11.out
	diff q11.out test-c/q11.exp && echo "Q11 ${TEST_PASSED}" || (echo "Q11 ${TEST_FAILED}" && exit 1)
	rm q11.out
	rm q11.o
q11gen_test: q11
	./q11.o > test-c/q11.exp
	rm q11.o