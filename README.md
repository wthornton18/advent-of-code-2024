## Overview

This is a project for Advent of Code 2024. The goal is to first solve the puzzles in Rust, and then attempt to solve them in C. The puzzles are released daily, and I will attempt to solve them as soon as possible (UK time so GMT or BST).


## Running the code

All the rust code can be run using the following command:

```bash
cargo run --release --bin q<day>_part<part>
```

Where `<day>` is the day of the puzzle and `<part>` is the part of the puzzle. For example, to run day 1 part 1, you would run:

```bash
cargo run --release --bin 1_part1

```

For the C code, you can run the following command:

```bash

make q<day>

```

Where `<day>` is the day of the puzzle. For example, to run day 1, you would run:

```bash
make q1
```

For the C code both parts of the puzzle are run at the same time.

## Testing

To run the tests for the rust code, you can run the following command:

```bash
cargo test --release
```

This will run all the tests for all the days.

To run the tests for the C code, you can run the following command:

```bash
make test_all
```

This doesn't yet have a nice test harness, but it will run the tests for all the days. An output may look like this:

```bash
gcc -O3 -pedantic -Wall -Wextra -Werror -std=c99 -lm src-c/q1.c -o q1.o
./q1.o > q1.out
diff q1.out test-c/q1.exp && echo "Q1 PASSED ✅" || echo "Q1 FAILED ❌"
Q1 PASSED ✅
rm q1.out
rm q1.o
gcc -O3 -pedantic -Wall -Wextra -Werror -std=c99 -lm src-c/q2.c -o q2.o
./q2.o > q2.out
diff q2.out test-c/q2.exp && echo "Q2 PASSED ✅" || echo "Q2 FAILED ❌"
Q2 FAILED ❌
rm q2.out
rm q2.o
gcc -O3 -pedantic -Wall -Wextra -Werror -std=c99 -lm src-c/q3.c -o q3.o
./q3.o > q3.out
diff q3.out test-c/q3.exp && echo "Q3 PASSED ✅" || echo "Q3 FAILED ❌"
Q3 PASSED ✅
rm q3.out
rm q3.o
gcc -O3 -pedantic -Wall -Wextra -Werror -std=c99 -lm src-c/q4.c -o q4.o
./q4.o > q4.out
diff q4.out test-c/q4.exp && echo "Q4 PASSED ✅" || echo "Q4 FAILED ❌"
Q4 PASSED ✅
rm q4.out
rm q4.o
```

Which shows that all the tests but day 2 have passed