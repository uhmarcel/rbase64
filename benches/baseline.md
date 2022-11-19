# Profiling Report
```yml
encode/3                time:   [33.386 ns 33.447 ns 33.513 ns]
                        thrpt:  [85.370 MiB/s 85.540 MiB/s 85.696 MiB/s]
                 change:
                        time:   [+5.7466% +6.0295% +6.3033%] (p = 0.00 < 0.05)
                        thrpt:  [-5.9296% -5.6866% -5.4343%]
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
encode/50               time:   [61.595 ns 61.739 ns 61.884 ns]
                        thrpt:  [770.54 MiB/s 772.34 MiB/s 774.15 MiB/s]
                 change:
                        time:   [+19.063% +19.395% +19.730%] (p = 0.00 < 0.05)
                        thrpt:  [-16.479% -16.244% -16.011%]
                        Performance has regressed.
encode/100              time:   [81.979 ns 82.157 ns 82.351 ns]
                        thrpt:  [1.1309 GiB/s 1.1336 GiB/s 1.1360 GiB/s]
                 change:
                        time:   [+14.280% +14.583% +14.867%] (p = 0.00 < 0.05)
                        thrpt:  [-12.943% -12.727% -12.496%]
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
encode/500              time:   [261.80 ns 262.26 ns 262.75 ns]
                        thrpt:  [1.7723 GiB/s 1.7755 GiB/s 1.7787 GiB/s]
                 change:
                        time:   [-1.3379% -0.9701% -0.6134%] (p = 0.00 < 0.05)
                        thrpt:  [+0.6172% +0.9796% +1.3561%]
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
encode/3072             time:   [1.3268 µs 1.3304 µs 1.3352 µs]
                        thrpt:  [2.1428 GiB/s 2.1504 GiB/s 2.1563 GiB/s]
                 change:
                        time:   [-3.0687% -2.7695% -2.4675%] (p = 0.00 < 0.05)
                        thrpt:  [+2.5300% +2.8484% +3.1659%]
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
encode/1048576          time:   [425.98 µs 426.80 µs 427.57 µs]
                        thrpt:  [2.2840 GiB/s 2.2881 GiB/s 2.2925 GiB/s]
                 change:
                        time:   [-2.5267% -2.2541% -1.9739%] (p = 0.00 < 0.05)
                        thrpt:  [+2.0136% +2.3060% +2.5922%]
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
encode/5242880          time:   [2.4011 ms 2.4059 ms 2.4110 ms]
                        thrpt:  [2.0252 GiB/s 2.0295 GiB/s 2.0336 GiB/s]
                 change:
                        time:   [-11.149% -6.0176% -2.6590%] (p = 0.00 < 0.05)
                        thrpt:  [+2.7316% +6.4029% +12.548%]
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  7 (7.00%) high mild
  2 (2.00%) high severe
encode/10485760         time:   [5.0205 ms 5.0300 ms 5.0399 ms]
                        thrpt:  [1.9377 GiB/s 1.9415 GiB/s 1.9452 GiB/s]
                 change:
                        time:   [-2.4759% -2.1795% -1.8817%] (p = 0.00 < 0.05)
                        thrpt:  [+1.9178% +2.2281% +2.5387%]
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) low severe
  7 (7.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe

decode/3                time:   [30.885 ns 30.905 ns 30.928 ns]
                        thrpt:  [92.506 MiB/s 92.574 MiB/s 92.633 MiB/s]
                 change:
                        time:   [-5.0108% -3.4729% -2.0920%] (p = 0.00 < 0.05)
                        thrpt:  [+2.1367% +3.5979% +5.2752%]
                        Performance has improved.
Found 24 outliers among 100 measurements (24.00%)
  7 (7.00%) low severe
  3 (3.00%) low mild
  6 (6.00%) high mild
  8 (8.00%) high severe
decode/50               time:   [62.183 ns 62.261 ns 62.373 ns]
                        thrpt:  [764.50 MiB/s 765.87 MiB/s 766.83 MiB/s]
                 change:
                        time:   [-40.088% -31.751% -22.479%] (p = 0.00 < 0.05)
                        thrpt:  [+28.998% +46.522% +66.912%]
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  8 (8.00%) high severe
decode/100              time:   [92.329 ns 92.418 ns 92.505 ns]
                        thrpt:  [1.0068 GiB/s 1.0077 GiB/s 1.0087 GiB/s]
                 change:
                        time:   [-1.7618% -1.4695% -1.1905%] (p = 0.00 < 0.05)
                        thrpt:  [+1.2048% +1.4914% +1.7934%]
                        Performance has improved.
Found 33 outliers among 100 measurements (33.00%)
  11 (11.00%) low severe
  6 (6.00%) low mild
  5 (5.00%) high mild
  11 (11.00%) high severe
decode/500              time:   [370.63 ns 370.95 ns 371.23 ns]
                        thrpt:  [1.2544 GiB/s 1.2553 GiB/s 1.2564 GiB/s]
                 change:
                        time:   [-0.1411% +0.0416% +0.2161%] (p = 0.66 > 0.05)
                        thrpt:  [-0.2156% -0.0416% +0.1413%]
                        No change in performance detected.
Found 22 outliers among 100 measurements (22.00%)
  10 (10.00%) low severe
  4 (4.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe
decode/3072             time:   [2.0358 µs 2.0375 µs 2.0392 µs]
                        thrpt:  [1.4030 GiB/s 1.4042 GiB/s 1.4053 GiB/s]
                 change:
                        time:   [+0.0720% +0.2642% +0.4544%] (p = 0.01 < 0.05)
                        thrpt:  [-0.4524% -0.2635% -0.0719%]
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe
decode/1048576          time:   [667.44 µs 668.98 µs 671.00 µs]
                        thrpt:  [1.4554 GiB/s 1.4598 GiB/s 1.4632 GiB/s]
                 change:
                        time:   [-1.2973% -0.8384% -0.3996%] (p = 0.00 < 0.05)
                        thrpt:  [+0.4012% +0.8455% +1.3144%]
                        Change within noise threshold.
Found 23 outliers among 100 measurements (23.00%)
  8 (8.00%) low severe
  6 (6.00%) low mild
  3 (3.00%) high mild
  6 (6.00%) high severe
decode/5242880          time:   [3.4985 ms 3.5017 ms 3.5046 ms]
                        thrpt:  [1.3932 GiB/s 1.3944 GiB/s 1.3957 GiB/s]
                 change:
                        time:   [-2.6815% -2.2994% -1.9647%] (p = 0.00 < 0.05)
                        thrpt:  [+2.0040% +2.3535% +2.7554%]
                        Performance has improved.
Found 22 outliers among 100 measurements (22.00%)
  4 (4.00%) low severe
  13 (13.00%) low mild
  1 (1.00%) high mild
  4 (4.00%) high severe
decode/10485760         time:   [7.0853 ms 7.0913 ms 7.0977 ms]
                        thrpt:  [1.3759 GiB/s 1.3771 GiB/s 1.3783 GiB/s]
                 change:
                        time:   [-1.7380% -1.3047% -0.9829%] (p = 0.00 < 0.05)
                        thrpt:  [+0.9926% +1.3220% +1.7688%]
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  8 (8.00%) high mild
  3 (3.00%) high severe

```
