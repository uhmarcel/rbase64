# Profiling Report
```diff

encode/3                time:   [33.819 ns 33.847 ns 33.874 ns]
                        thrpt:  [84.460 MiB/s 84.528 MiB/s 84.597 MiB/s]
                 change:
                        time:   [+0.4183% +0.6705% +0.9191%] (p = 0.00 < 0.05)
                        thrpt:  [-0.9107% -0.6660% -0.4166%]
                        Change within noise threshold.
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) low severe
  2 (2.00%) low mild
  7 (7.00%) high mild
  3 (3.00%) high severe

encode/50               time:   [46.523 ns 46.592 ns 46.660 ns]
                        thrpt:  [1021.9 MiB/s 1023.4 MiB/s 1.0009 GiB/s]
                 change:
+                        time:   [-20.575% -20.383% -20.196%] (p = 0.00 < 0.05)
+                        thrpt:  [+25.307% +25.601% +25.905%]
+                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild

encode/100              time:   [65.344 ns 65.477 ns 65.636 ns]
                        thrpt:  [1.4189 GiB/s 1.4224 GiB/s 1.4253 GiB/s]
                 change:
+                        time:   [-13.333% -13.047% -12.780%] (p = 0.00 < 0.05)
+                        thrpt:  [+14.653% +15.005% +15.384%]
+                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

encode/500              time:   [220.04 ns 221.31 ns 222.76 ns]
                        thrpt:  [2.0904 GiB/s 2.1041 GiB/s 2.1163 GiB/s]
                 change:
+                        time:   [-3.1167% -2.5568% -1.9598%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.9990% +2.6238% +3.2170%]
+                        Performance has improved.
Found 20 outliers among 100 measurements (20.00%)
  9 (9.00%) low mild
  7 (7.00%) high mild
  4 (4.00%) high severe

encode/3072             time:   [1.0243 µs 1.0266 µs 1.0289 µs]
                        thrpt:  [2.7808 GiB/s 2.7869 GiB/s 2.7930 GiB/s]
                 change:
+                        time:   [-4.1981% -3.9159% -3.6448%] (p = 0.00 < 0.05)
+                        thrpt:  [+3.7827% +4.0755% +4.3821%]
+                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

encode/1048576          time:   [339.25 µs 339.75 µs 340.27 µs]
                        thrpt:  [2.8699 GiB/s 2.8743 GiB/s 2.8786 GiB/s]
                 change:
+                        time:   [-2.2247% -1.7897% -1.4088%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.4289% +1.8223% +2.2753%]
+                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) low severe
  8 (8.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

encode/5242880          time:   [1.9869 ms 1.9930 ms 2.0006 ms]
                        thrpt:  [2.4406 GiB/s 2.4500 GiB/s 2.4575 GiB/s]
                 change:
                        time:   [-1.0978% -0.6490% -0.1858%] (p = 0.01 < 0.05)
                        thrpt:  [+0.1862% +0.6532% +1.1100%]
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

encode/10485760         time:   [4.1619 ms 4.1677 ms 4.1734 ms]
                        thrpt:  [2.3399 GiB/s 2.3432 GiB/s 2.3464 GiB/s]
                 change:
                        time:   [+0.8847% +1.1634% +1.4275%] (p = 0.00 < 0.05)
                        thrpt:  [-1.4074% -1.1500% -0.8769%]
                        Change within noise threshold.



decode/3                time:   [30.721 ns 30.739 ns 30.758 ns]
                        thrpt:  [93.016 MiB/s 93.076 MiB/s 93.129 MiB/s]
                 change:
                        time:   [-0.7527% -0.5785% -0.3978%] (p = 0.00 < 0.05)
                        thrpt:  [+0.3994% +0.5819% +0.7584%]
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

decode/50               time:   [45.579 ns 45.609 ns 45.640 ns]
                        thrpt:  [1.0203 GiB/s 1.0210 GiB/s 1.0217 GiB/s]
                 change:
+                        time:   [-5.8084% -5.3462% -5.0350%] (p = 0.00 < 0.05)
+                        thrpt:  [+5.3019% +5.6482% +6.1666%]
+                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe

decode/100              time:   [60.543 ns 60.745 ns 61.003 ns]
                        thrpt:  [1.5267 GiB/s 1.5332 GiB/s 1.5383 GiB/s]
                 change:
+                        time:   [-7.4295% -6.9413% -6.4610%] (p = 0.00 < 0.05)
+                        thrpt:  [+6.9073% +7.4590% +8.0258%]
+                        Performance has improved.
Found 17 outliers among 100 measurements (17.00%)
  7 (7.00%) high mild
  10 (10.00%) high severe

decode/500              time:   [205.07 ns 205.34 ns 205.62 ns]
                        thrpt:  [2.2647 GiB/s 2.2677 GiB/s 2.2707 GiB/s]
                 change:
+                        time:   [-10.905% -10.754% -10.600%] (p = 0.00 < 0.05)
+                        thrpt:  [+11.857% +12.050% +12.240%]
+                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe

decode/3072             time:   [988.01 ns 989.24 ns 990.89 ns]
                        thrpt:  [2.8873 GiB/s 2.8921 GiB/s 2.8957 GiB/s]
                 change:
+                        time:   [-15.152% -14.993% -14.827%] (p = 0.00 < 0.05)
+                        thrpt:  [+17.409% +17.638% +17.857%]
+                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  2 (2.00%) low severe
  4 (4.00%) low mild
  6 (6.00%) high mild
  6 (6.00%) high severe

decode/1048576          time:   [322.66 µs 323.88 µs 326.30 µs]
                        thrpt:  [2.9928 GiB/s 3.0152 GiB/s 3.0266 GiB/s]
                 change:
+                        time:   [-13.349% -12.933% -12.471%] (p = 0.00 < 0.05)
+                        thrpt:  [+14.248% +14.854% +15.406%]
+                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  5 (5.00%) high severe

decode/5242880          time:   [1.7719 ms 1.7778 ms 1.7868 ms]
                        thrpt:  [2.7327 GiB/s 2.7465 GiB/s 2.7556 GiB/s]
                 change:
+                        time:   [-12.407% -12.070% -11.644%] (p = 0.00 < 0.05)
+                        thrpt:  [+13.178% +13.727% +14.164%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) high mild
  9 (9.00%) high severe

decode/10485760         time:   [3.5972 ms 3.6043 ms 3.6119 ms]
                        thrpt:  [2.7038 GiB/s 2.7095 GiB/s 2.7148 GiB/s]
                 change:
+                        time:   [-12.009% -11.800% -11.586%] (p = 0.00 < 0.05)
+                        thrpt:  [+13.104% +13.379% +13.648%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild



```

