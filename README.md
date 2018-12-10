In the board game Risk, given a number of attacking and defending troops, what
is the probability the attack succeeds and what are the expected number of
troops lost regardless of success?

***WARNING*** This table was not checked for correctness. I have not formalized
the math and the code had a lot of duplication and leaves a lot of room for
copy, paste and modify forgetfulness errors.

```
a01d01  42%  0.58 | a01d02  11%  0.89 | a01d03   3%  0.97 | a01d04   1%  0.99 | a01d05   0%  1.00 | a01d06   0%  1.00 | a01d07   0%  1.00 | a01d08   0%  1.00 | a01d09   0%  1.00 | a01d10   0%  1.00
a02d01  75%  0.67 | a02d02  36%  1.52 | a02d03  21%  1.87 | a02d04   9%  1.97 | a02d05   5%  2.00 | a02d06   2%  2.00 | a02d07   1%  2.00 | a02d08   0%  2.00 | a02d09   0%  2.00 | a02d10   0%  2.00
a03d01  92%  0.57 | a03d02  66%  1.41 | a03d03  47%  1.95 | a03d04  31%  2.39 | a03d05  21%  2.62 | a03d06  13%  2.79 | a03d07   8%  2.87 | a03d08   5%  2.93 | a03d09   3%  2.95 | a03d10   2%  2.97
a04d01  97%  0.53 | a04d02  79%  1.59 | a04d03  64%  2.20 | a04d04  48%  2.78 | a04d05  36%  3.16 | a04d06  25%  3.44 | a04d07  18%  3.63 | a04d08  12%  3.76 | a04d09   9%  3.84 | a04d10   6%  3.90
a05d01  99%  0.52 | a05d02  89%  1.51 | a05d03  77%  2.23 | a05d04  64%  2.95 | a05d05  51%  3.48 | a05d06  40%  3.92 | a05d07  30%  4.23 | a05d08  22%  4.47 | a05d09  16%  4.63 | a05d10  12%  4.75
a06d01 100%  0.52 | a06d02  93%  1.57 | a06d03  86%  2.28 | a06d04  74%  3.09 | a06d05  64%  3.71 | a06d06  52%  4.27 | a06d07  42%  4.70 | a06d08  33%  5.04 | a06d09  26%  5.31 | a06d10  19%  5.50
a07d01 100%  0.52 | a07d02  97%  1.54 | a07d03  91%  2.30 | a07d04  83%  3.14 | a07d05  74%  3.85 | a07d06  64%  4.50 | a07d07  54%  5.04 | a07d08  45%  5.50 | a07d09  36%  5.86 | a07d10  29%  6.15
a08d01 100%  0.52 | a08d02  98%  1.56 | a08d03  95%  2.30 | a08d04  89%  3.19 | a08d05  82%  3.93 | a08d06  73%  4.66 | a08d07  64%  5.29 | a08d08  55%  5.84 | a08d09  46%  6.31 | a08d10  38%  6.68
a09d01 100%  0.52 | a09d02  99%  1.54 | a09d03  97%  2.31 | a09d04  93%  3.19 | a09d05  87%  3.99 | a09d06  81%  4.76 | a09d07  73%  5.46 | a09d08  65%  6.09 | a09d09  56%  6.64 | a09d10  48%  7.12
a10d01 100%  0.52 | a10d02  99%  1.55 | a10d03  98%  2.31 | a10d04  95%  3.21 | a10d05  92%  4.01 | a10d06  86%  4.83 | a10d07  80%  5.57 | a10d08  72%  6.27 | a10d09  65%  6.90 | a10d10  57%  7.45
a11d01 100%  0.52 | a11d02 100%  1.55 | a11d03  99%  2.31 | a11d04  97%  3.21 | a11d05  94%  4.03 | a11d06  91%  4.86 | a11d07  85%  5.64 | a11d08  79%  6.39 | a11d09  72%  7.08 | a11d10  65%  7.71
a12d01 100%  0.52 | a12d02 100%  1.55 | a12d03  99%  2.31 | a12d04  98%  3.21 | a12d05  96%  4.03 | a12d06  93%  4.88 | a12d07  90%  5.69 | a12d08  84%  6.47 | a12d09  79%  7.21 | a12d10  72%  7.89
a13d01 100%  0.52 | a13d02 100%  1.55 | a13d03 100%  2.31 | a13d04  99%  3.21 | a13d05  98%  4.04 | a13d06  96%  4.89 | a13d07  93%  5.72 | a13d08  89%  6.52 | a13d09  84%  7.30 | a13d10  79%  8.03
a14d01 100%  0.52 | a14d02 100%  1.55 | a14d03 100%  2.31 | a14d04  99%  3.21 | a14d05  98%  4.04 | a14d06  97%  4.90 | a14d07  95%  5.73 | a14d08  92%  6.56 | a14d09  88%  7.36 | a14d10  83%  8.13
a15d01 100%  0.52 | a15d02 100%  1.55 | a15d03 100%  2.31 | a15d04 100%  3.21 | a15d05  99%  4.04 | a15d06  98%  4.90 | a15d07  96%  5.74 | a15d08  94%  6.58 | a15d09  91%  7.40 | a15d10  88%  8.19
a16d01 100%  0.52 | a16d02 100%  1.55 | a16d03 100%  2.31 | a16d04 100%  3.21 | a16d05  99%  4.04 | a16d06  99%  4.91 | a16d07  98%  5.75 | a16d08  96%  6.60 | a16d09  94%  7.42 | a16d10  91%  8.24
a17d01 100%  0.52 | a17d02 100%  1.55 | a17d03 100%  2.31 | a17d04 100%  3.21 | a17d05 100%  4.04 | a17d06  99%  4.90 | a17d07  98%  5.75 | a17d08  97%  6.60 | a17d09  95%  7.44 | a17d10  93%  8.27
a18d01 100%  0.52 | a18d02 100%  1.55 | a18d03 100%  2.31 | a18d04 100%  3.21 | a18d05 100%  4.04 | a18d06  99%  4.91 | a18d07  99%  5.75 | a18d08  98%  6.61 | a18d09  97%  7.45 | a18d10  95%  8.29
a19d01 100%  0.52 | a19d02 100%  1.55 | a19d03 100%  2.31 | a19d04 100%  3.21 | a19d05 100%  4.04 | a19d06 100%  4.90 | a19d07  99%  5.75 | a19d08  99%  6.61 | a19d09  98%  7.46 | a19d10  97%  8.30
a20d01 100%  0.52 | a20d02 100%  1.55 | a20d03 100%  2.31 | a20d04 100%  3.21 | a20d05 100%  4.04 | a20d06 100%  4.90 | a20d07 100%  5.75 | a20d08  99%  6.61 | a20d09  98%  7.46 | a20d10  97%  8.31
```

Can do a simulation, which should be a lot simpler to code, to figure out if the
numerically found numbers above make sense.

## Implementation

First we compute the probabilities of each outcome in all possible round
scenarios: a1d1, a2d1, a3d1, a1d2, a2d2 and a3d2. The scenarios where the
minimum number of troops between both sides is 1 have the possible losses a0d1
and a1d0. The scenarios where the minimum number of troops between both sides is
2 have the possible losses a0d2, a1d1 and a2d0.

Then we compute the probabilities that a fight ends with a particular outcome.
We assume that both sides will use the maximum possible number of troops in each
round, as that gives them the best chance of winning. The probability
p_final_a1d0(a, d) is (assuming a >= 3 and d >= 2) p_round_a3d2_losing_a0d2 *
p_final_a1d0(a, d - 2) + p_round_a3d2_losing_a1d1 * p_final_a1d0(a - 1, d - 1) +
p_round_a3d2_losing_a2d0 * p_final_a1d0(a - 2, d).

Now work out the right round probabilities to pick depending on a and d, figure
out the starting values for p_final_axdx, fill a table of p_final_axdx since the
results can be reused in the recursive computation. There you go.

## Future work

 * Simplify the code. 
 * Minimize memory usage, the w[i] and l[i] arrays can be decoupled. We don't
   need to compute p_final_a9d0 for starting armies where a < 9 because that
   probability is 0!
 * The sorted dice iteration has the counts hardcoded. What is the math for that?
 * Test the limits, how large can we make N?
 * Get more insight by plotting the data
 * Determine rough but quick rules to estimate your chances and losses during a game of Risk.
