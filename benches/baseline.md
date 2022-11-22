# Profiling Report
```diff

encode/3                time:   [32.914 ns 33.037 ns 33.217 ns]
                        thrpt:  [86.130 MiB/s 86.601 MiB/s 86.925 MiB/s]
                 change:
-                        time:   [+2.1153% +3.9641% +6.3489%] (p = 0.00 < 0.05)
-                        thrpt:  [-5.9699% -3.8129% -2.0715%]
-                        Performance has regressed.
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe

encode/50               time:   [46.908 ns 46.972 ns 47.043 ns]
                        thrpt:  [1013.6 MiB/s 1015.1 MiB/s 1016.5 MiB/s]
                 change:
                        time:   [-0.3823% -0.0658% +0.3724%] (p = 0.74 > 0.05)
                        thrpt:  [-0.3710% +0.0658% +0.3838%]
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe

encode/100              time:   [64.616 ns 64.686 ns 64.757 ns]
                        thrpt:  [1.4382 GiB/s 1.4398 GiB/s 1.4413 GiB/s]
                 change:
                        time:   [-1.3551% -1.1364% -0.9161%] (p = 0.00 < 0.05)
                        thrpt:  [+0.9245% +1.1495% +1.3737%]
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

encode/500              time:   [217.78 ns 218.37 ns 219.33 ns]
                        thrpt:  [2.1231 GiB/s 2.1325 GiB/s 2.1382 GiB/s]
                 change:
+                        time:   [-1.8834% -1.5536% -1.2441%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.2598% +1.5781% +1.9196%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe

encode/3072             time:   [1.0794 µs 1.0807 µs 1.0822 µs]
                        thrpt:  [2.6438 GiB/s 2.6474 GiB/s 2.6505 GiB/s]
                 change:
                        time:   [-1.3591% -0.8781% -0.4761%] (p = 0.00 < 0.05)
                        thrpt:  [+0.4784% +0.8859% +1.3778%]
                        Change within noise threshold.
Found 19 outliers among 100 measurements (19.00%)
  12 (12.00%) high mild
  7 (7.00%) high severe

encode/51200            time:   [17.665 µs 17.681 µs 17.696 µs]
                        thrpt:  [2.6945 GiB/s 2.6970 GiB/s 2.6994 GiB/s]
                 change:
                        time:   [-0.6563% -0.4157% -0.1736%] (p = 0.00 < 0.05)
                        thrpt:  [+0.1739% +0.4175% +0.6606%]
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

encode/102400           time:   [35.389 µs 35.422 µs 35.461 µs]
                        thrpt:  [2.6894 GiB/s 2.6923 GiB/s 2.6948 GiB/s]
                 change:
                        time:   [-1.0823% -0.8242% -0.5663%] (p = 0.00 < 0.05)
                        thrpt:  [+0.5695% +0.8311% +1.0942%]
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

encode/512000           time:   [90.109 µs 92.017 µs 94.150 µs]
                        thrpt:  [5.0646 GiB/s 5.1821 GiB/s 5.2918 GiB/s]
                 change:
                        time:   [-0.9075% +1.2513% +3.6167%] (p = 0.28 > 0.05)
                        thrpt:  [-3.4905% -1.2359% +0.9159%]
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  9 (9.00%) high severe

encode/1048576          time:   [130.32 µs 131.35 µs 132.50 µs]
                        thrpt:  [7.3700 GiB/s 7.4348 GiB/s 7.4935 GiB/s]
                 change:
                        time:   [-0.1584% +1.3937% +2.9381%] (p = 0.08 > 0.05)
                        thrpt:  [-2.8543% -1.3746% +0.1587%]
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  9 (9.00%) high mild
  5 (5.00%) high severe

encode/5242880          time:   [836.25 µs 844.56 µs 855.20 µs]
                        thrpt:  [5.7096 GiB/s 5.7815 GiB/s 5.8390 GiB/s]
                 change:
                        time:   [-3.5489% -0.4059% +2.5482%] (p = 0.81 > 0.05)
                        thrpt:  [-2.4849% +0.4076% +3.6795%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

encode/10485760         time:   [1.5256 ms 1.5355 ms 1.5469 ms]
                        thrpt:  [6.3130 GiB/s 6.3599 GiB/s 6.4010 GiB/s]
                 change:
                        time:   [-0.9159% -0.0140% +0.8283%] (p = 0.98 > 0.05)
                        thrpt:  [-0.8215% +0.0140% +0.9243%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) high mild
  11 (11.00%) high severe

encode/20971520         time:   [2.9660 ms 2.9913 ms 3.0248 ms]
                        thrpt:  [6.4571 GiB/s 6.5294 GiB/s 6.5850 GiB/s]
                 change:
                        time:   [-0.5013% +0.6144% +2.0107%] (p = 0.33 > 0.05)
                        thrpt:  [-1.9711% -0.6106% +0.5038%]
                        No change in performance detected.
Found 19 outliers among 100 measurements (19.00%)
  8 (8.00%) high mild
  11 (11.00%) high severe



decode/3                time:   [31.052 ns 31.082 ns 31.113 ns]
                        thrpt:  [91.955 MiB/s 92.048 MiB/s 92.138 MiB/s]
                 change:
                        time:   [-0.3367% -0.1641% +0.0049%] (p = 0.07 > 0.05)
                        thrpt:  [-0.0049% +0.1644% +0.3379%]
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  1 (1.00%) low severe
  7 (7.00%) low mild
  5 (5.00%) high mild
  5 (5.00%) high severe

decode/50               time:   [45.627 ns 45.688 ns 45.759 ns]
                        thrpt:  [1.0176 GiB/s 1.0192 GiB/s 1.0206 GiB/s]
                 change:
                        time:   [+0.0285% +0.2053% +0.3755%] (p = 0.02 < 0.05)
                        thrpt:  [-0.3741% -0.2048% -0.0285%]
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

decode/100              time:   [59.578 ns 59.838 ns 60.178 ns]
                        thrpt:  [1.5476 GiB/s 1.5564 GiB/s 1.5632 GiB/s]
                 change:
                        time:   [-0.0672% +0.3950% +0.9375%] (p = 0.11 > 0.05)
                        thrpt:  [-0.9288% -0.3934% +0.0672%]
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  11 (11.00%) high severe

decode/500              time:   [199.31 ns 199.94 ns 200.66 ns]
                        thrpt:  [2.3206 GiB/s 2.3290 GiB/s 2.3364 GiB/s]
                 change:
+                        time:   [-3.4414% -3.1322% -2.8314%] (p = 0.00 < 0.05)
+                        thrpt:  [+2.9139% +3.2334% +3.5640%]
+                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  13 (13.00%) high mild
  1 (1.00%) high severe

decode/3072             time:   [945.41 ns 946.93 ns 948.71 ns]
                        thrpt:  [3.0157 GiB/s 3.0214 GiB/s 3.0262 GiB/s]
                 change:
                        time:   [-0.0110% +0.1316% +0.2922%] (p = 0.11 > 0.05)
                        thrpt:  [-0.2914% -0.1314% +0.0110%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe

decode/51200            time:   [15.558 µs 15.611 µs 15.662 µs]
                        thrpt:  [3.0446 GiB/s 3.0545 GiB/s 3.0649 GiB/s]
                 change:
                        time:   [-0.1928% +0.1764% +0.5486%] (p = 0.36 > 0.05)
                        thrpt:  [-0.5456% -0.1761% +0.1931%]
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

decode/102400           time:   [30.866 µs 30.976 µs 31.085 µs]
                        thrpt:  [3.0679 GiB/s 3.0787 GiB/s 3.0897 GiB/s]
                 change:
                        time:   [-0.9775% -0.6184% -0.2538%] (p = 0.00 < 0.05)
                        thrpt:  [+0.2545% +0.6223% +0.9871%]
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

decode/512000           time:   [71.555 µs 72.078 µs 72.684 µs]
                        thrpt:  [6.5604 GiB/s 6.6156 GiB/s 6.6639 GiB/s]
                 change:
                        time:   [-0.3378% +0.9551% +2.3631%] (p = 0.17 > 0.05)
                        thrpt:  [-2.3085% -0.9460% +0.3389%]
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  9 (9.00%) high severe

decode/1048576          time:   [113.73 µs 115.00 µs 116.52 µs]
                        thrpt:  [8.3812 GiB/s 8.4921 GiB/s 8.5867 GiB/s]
                 change:
                        time:   [+0.7656% +2.2177% +3.8938%] (p = 0.00 < 0.05)
                        thrpt:  [-3.7479% -2.1696% -0.7598%]
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)

  1 (1.00%) low mild
  4 (4.00%) high mild
  9 (9.00%) high severe

decode/5242880          time:   [603.94 µs 610.15 µs 617.51 µs]
                        thrpt:  [7.9072 GiB/s 8.0027 GiB/s 8.0850 GiB/s]
                 change:
                        time:   [-0.8367% +1.1746% +3.1325%] (p = 0.26 > 0.05)
                        thrpt:  [-3.0374% -1.1610% +0.8437%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

decode/10485760         time:   [1.2049 ms 1.2114 ms 1.2187 ms]
                        thrpt:  [8.0130 GiB/s 8.0614 GiB/s 8.1053 GiB/s]
                 change:
                        time:   [+0.4271% +1.2029% +2.0284%] (p = 0.00 < 0.05)
                        thrpt:  [-1.9881% -1.1886% -0.4253%]
                        Change within noise threshold.
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) high mild
  11 (11.00%) high severe

decode/20971520         time:   [2.2643 ms 2.2847 ms 2.3098 ms]
                        thrpt:  [8.4557 GiB/s 8.5488 GiB/s 8.6258 GiB/s]
                 change:
                        time:   [+0.4473% +1.4790% +2.5370%] (p = 0.00 < 0.05)
                        thrpt:  [-2.4742% -1.4575% -0.4453%]
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe



```

