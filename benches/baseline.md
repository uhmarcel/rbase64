# Profiling Report
```diff

encode/3                time:   [32.562 ns 32.591 ns 32.621 ns]
                        thrpt:  [87.704 MiB/s 87.786 MiB/s 87.863 MiB/s]
                 change:
+                        time:   [-1.9780% -1.7935% -1.6123%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.6387% +1.8263% +2.0179%]
+                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

encode/50               time:   [46.759 ns 46.811 ns 46.873 ns]
                        thrpt:  [1017.3 MiB/s 1018.6 MiB/s 1019.8 MiB/s]
                 change:
+                        time:   [-2.0444% -1.8142% -1.5807%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.6061% +1.8477% +2.0871%]
+                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

encode/100              time:   [64.396 ns 64.449 ns 64.505 ns]
                        thrpt:  [1.4438 GiB/s 1.4451 GiB/s 1.4462 GiB/s]
                 change:
+                        time:   [-2.0885% -1.9594% -1.8225%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.8563% +1.9986% +2.1331%]
+                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

encode/500              time:   [217.08 ns 217.23 ns 217.39 ns]
                        thrpt:  [2.1421 GiB/s 2.1437 GiB/s 2.1451 GiB/s]
                 change:
+                        time:   [-4.8176% -4.4157% -4.0071%] (p = 0.00 < 0.05)
+                        thrpt:  [+4.1744% +4.6197% +5.0614%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) high mild
  6 (6.00%) high severe

encode/3072             time:   [1.0798 µs 1.0810 µs 1.0825 µs]
                        thrpt:  [2.6430 GiB/s 2.6467 GiB/s 2.6496 GiB/s]
                 change:
+                        time:   [-1.9872% -1.8303% -1.6712%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.6996% +1.8644% +2.0275%]
+                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe

encode/51200            time:   [17.627 µs 17.640 µs 17.654 µs]
                        thrpt:  [2.7009 GiB/s 2.7031 GiB/s 2.7052 GiB/s]
                 change:
+                        time:   [-1.9149% -1.7801% -1.6396%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.6669% +1.8124% +1.9523%]
+                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

encode/102400           time:   [35.343 µs 35.372 µs 35.404 µs]
                        thrpt:  [2.6937 GiB/s 2.6962 GiB/s 2.6983 GiB/s]
                 change:
+                        time:   [-1.6437% -1.4620% -1.2594%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.2754% +1.4837% +1.6712%]
+                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

encode/512000           time:   [183.93 µs 185.20 µs 186.88 µs]
                        thrpt:  [2.5516 GiB/s 2.5747 GiB/s 2.5925 GiB/s]
                 change:
                        time:   [-1.1794% +0.0284% +1.2515%] (p = 0.96 > 0.05)
                        thrpt:  [-1.2360% -0.0284% +1.1935%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe

encode/1048576          time:   [314.32 µs 316.99 µs 320.67 µs]
                        thrpt:  [3.0454 GiB/s 3.0807 GiB/s 3.1069 GiB/s]
                 change:
                        time:   [-2.0021% -0.6735% +0.5748%] (p = 0.32 > 0.05)
                        thrpt:  [-0.5715% +0.6781% +2.0430%]
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

encode/5242880          time:   [1.6368 ms 1.6416 ms 1.6477 ms]
                        thrpt:  [2.9633 GiB/s 2.9743 GiB/s 2.9832 GiB/s]
                 change:
                        time:   [-1.1881% -0.6082% -0.0421%] (p = 0.04 < 0.05)
                        thrpt:  [+0.0421% +0.6119% +1.2024%]
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

encode/10485760         time:   [3.1422 ms 3.1548 ms 3.1709 ms]
                        thrpt:  [3.0798 GiB/s 3.0955 GiB/s 3.1079 GiB/s]
                 change:
                        time:   [-0.3535% +0.1628% +0.7479%] (p = 0.58 > 0.05)
                        thrpt:  [-0.7423% -0.1625% +0.3548%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe

encode/20971520         time:   [6.1521 ms 6.1784 ms 6.2096 ms]
                        thrpt:  [3.1453 GiB/s 3.1612 GiB/s 3.1747 GiB/s]
                 change:
                        time:   [-0.3138% +0.2221% +0.7538%] (p = 0.45 > 0.05)
                        thrpt:  [-0.7482% -0.2216% +0.3148%]
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe



decode/3                time:   [30.986 ns 31.036 ns 31.122 ns]
                        thrpt:  [91.929 MiB/s 92.185 MiB/s 92.332 MiB/s]
                 change:
                        time:   [-0.0510% +0.1096% +0.2851%] (p = 0.20 > 0.05)
                        thrpt:  [-0.2843% -0.1095% +0.0510%]
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  8 (8.00%) high mild
  8 (8.00%) high severe

decode/50               time:   [45.800 ns 45.830 ns 45.866 ns]
                        thrpt:  [1.0153 GiB/s 1.0161 GiB/s 1.0167 GiB/s]
                 change:
                        time:   [-0.0462% +0.0976% +0.2548%] (p = 0.19 > 0.05)
                        thrpt:  [-0.2541% -0.0975% +0.0462%]
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe

decode/100              time:   [60.531 ns 60.689 ns 60.878 ns]
                        thrpt:  [1.5298 GiB/s 1.5346 GiB/s 1.5386 GiB/s]
                 change:
                        time:   [-0.2959% +0.0162% +0.3082%] (p = 0.92 > 0.05)
                        thrpt:  [-0.3073% -0.0162% +0.2968%]
                        No change in performance detected.
Found 20 outliers among 100 measurements (20.00%)
  2 (2.00%) low mild
  7 (7.00%) high mild
  11 (11.00%) high severe

decode/500              time:   [205.60 ns 206.46 ns 207.43 ns]
                        thrpt:  [2.2450 GiB/s 2.2555 GiB/s 2.2649 GiB/s]
                 change:
                        time:   [+0.2010% +0.5517% +0.8987%] (p = 0.00 < 0.05)
                        thrpt:  [-0.8907% -0.5486% -0.2006%]
                        Change within noise threshold.
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe

decode/3072             time:   [989.33 ns 989.87 ns 990.50 ns]
                        thrpt:  [2.8885 GiB/s 2.8903 GiB/s 2.8919 GiB/s]
                 change:
                        time:   [-0.0879% +0.0340% +0.1629%] (p = 0.61 > 0.05)
                        thrpt:  [-0.1627% -0.0340% +0.0880%]
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe

decode/51200            time:   [16.082 µs 16.100 µs 16.119 µs]
                        thrpt:  [2.9583 GiB/s 2.9617 GiB/s 2.9650 GiB/s]
                 change:
                        time:   [-0.2770% -0.1040% +0.0556%] (p = 0.23 > 0.05)
                        thrpt:  [-0.0556% +0.1041% +0.2778%]
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) low mild
  4 (4.00%) high mild

decode/102400           time:   [31.893 µs 31.926 µs 31.961 µs]
                        thrpt:  [2.9839 GiB/s 2.9871 GiB/s 2.9903 GiB/s]
                 change:
                        time:   [-0.4732% -0.2816% -0.0733%] (p = 0.00 < 0.05)
                        thrpt:  [+0.0733% +0.2824% +0.4754%]
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

decode/512000           time:   [153.70 µs 154.28 µs 154.91 µs]
                        thrpt:  [3.0781 GiB/s 3.0907 GiB/s 3.1024 GiB/s]
                 change:
+                        time:   [-5.1536% -3.8720% -2.7486%] (p = 0.00 < 0.05)
+                        thrpt:  [+2.8263% +4.0280% +5.4336%]
+                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe

decode/1048576          time:   [257.05 µs 258.18 µs 259.48 µs]
                        thrpt:  [3.7635 GiB/s 3.7825 GiB/s 3.7991 GiB/s]
                 change:
+                        time:   [-8.8280% -7.0183% -5.1265%] (p = 0.00 < 0.05)
+                        thrpt:  [+5.4035% +7.5481% +9.6828%]
+                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

decode/5242880          time:   [1.1936 ms 1.1998 ms 1.2074 ms]
                        thrpt:  [4.0442 GiB/s 4.0696 GiB/s 4.0907 GiB/s]
                 change:
                        time:   [-1.5858% -0.5263% +0.4324%] (p = 0.33 > 0.05)
                        thrpt:  [-0.4306% +0.5291% +1.6113%]
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe

decode/10485760         time:   [2.3603 ms 2.3709 ms 2.3842 ms]
                        thrpt:  [4.0960 GiB/s 4.1189 GiB/s 4.1374 GiB/s]
                 change:
                        time:   [-0.1435% +0.5346% +1.1859%] (p = 0.12 > 0.05)
                        thrpt:  [-1.1720% -0.5318% +0.1437%]
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

decode/20971520         time:   [4.6860 ms 4.7041 ms 4.7241 ms]
                        thrpt:  [4.1344 GiB/s 4.1520 GiB/s 4.1680 GiB/s]
                 change:
                        time:   [-1.3647% -0.4536% +0.3371%] (p = 0.32 > 0.05)
                        thrpt:  [-0.3360% +0.4556% +1.3836%]
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe



```

