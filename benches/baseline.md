# Profiling Report
```diff

encode/3                time:   [14.337 ns 14.353 ns 14.370 ns]
                        thrpt:  [199.09 MiB/s 199.34 MiB/s 199.56 MiB/s]
                 change:
                        time:   [-0.2099% -0.0960% +0.0288%] (p = 0.13 > 0.05)
                        thrpt:  [-0.0288% +0.0961% +0.2103%]
                        No change in performance detected.
Found 23 outliers among 100 measurements (23.00%)
  6 (6.00%) low severe
  2 (2.00%) low mild
  3 (3.00%) high mild
  12 (12.00%) high severe

encode/50               time:   [31.719 ns 31.754 ns 31.790 ns]
                        thrpt:  [1.4648 GiB/s 1.4665 GiB/s 1.4681 GiB/s]
                 change:
                        time:   [-0.4254% -0.2249% -0.0329%] (p = 0.03 < 0.05)
                        thrpt:  [+0.0330% +0.2254% +0.4273%]
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

encode/100              time:   [47.547 ns 47.618 ns 47.709 ns]
                        thrpt:  [1.9521 GiB/s 1.9558 GiB/s 1.9588 GiB/s]
                 change:
                        time:   [-0.2634% -0.0773% +0.1218%] (p = 0.43 > 0.05)
                        thrpt:  [-0.1217% +0.0774% +0.2641%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  4 (4.00%) high severe

encode/500              time:   [185.67 ns 185.82 ns 185.97 ns]
                        thrpt:  [2.5040 GiB/s 2.5060 GiB/s 2.5080 GiB/s]
                 change:
                        time:   [-0.1874% -0.0152% +0.1432%] (p = 0.86 > 0.05)
                        thrpt:  [-0.1430% +0.0152% +0.1878%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe

encode/3072             time:   [1.0872 µs 1.0880 µs 1.0888 µs]
                        thrpt:  [2.6276 GiB/s 2.6296 GiB/s 2.6314 GiB/s]
                 change:
                        time:   [-0.3924% -0.2233% -0.0558%] (p = 0.01 < 0.05)
                        thrpt:  [+0.0558% +0.2238% +0.3940%]
                        Change within noise threshold.
Found 18 outliers among 100 measurements (18.00%)
  3 (3.00%) low severe
  6 (6.00%) high mild
  9 (9.00%) high severe

encode/51200            time:   [17.719 µs 17.734 µs 17.750 µs]
                        thrpt:  [2.6863 GiB/s 2.6889 GiB/s 2.6911 GiB/s]
                 change:
                        time:   [-0.3813% -0.2249% -0.0724%] (p = 0.00 < 0.05)
                        thrpt:  [+0.0725% +0.2254% +0.3827%]
                        Change within noise threshold.
Found 23 outliers among 100 measurements (23.00%)
  4 (4.00%) low severe
  3 (3.00%) low mild
  6 (6.00%) high mild
  10 (10.00%) high severe

encode/102400           time:   [36.010 µs 36.043 µs 36.078 µs]
                        thrpt:  [2.6434 GiB/s 2.6459 GiB/s 2.6484 GiB/s]
                 change:
                        time:   [-0.5525% -0.3592% -0.1795%] (p = 0.00 < 0.05)
                        thrpt:  [+0.1799% +0.3605% +0.5556%]
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

encode/512000           time:   [86.165 µs 91.543 µs 98.820 µs]
                        thrpt:  [4.8253 GiB/s 5.2089 GiB/s 5.5340 GiB/s]
                 change:
                        time:   [-3.4157% +0.5382% +5.4026%] (p = 0.82 > 0.05)
                        thrpt:  [-5.1256% -0.5354% +3.5365%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  8 (8.00%) high mild
  4 (4.00%) high severe

encode/1048576          time:   [131.73 µs 133.67 µs 135.96 µs]
                        thrpt:  [7.1829 GiB/s 7.3060 GiB/s 7.4132 GiB/s]
                 change:
                        time:   [-2.9508% +0.1403% +3.2755%] (p = 0.92 > 0.05)
                        thrpt:  [-3.1716% -0.1401% +3.0405%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

encode/5242880          time:   [531.64 µs 535.19 µs 539.09 µs]
                        thrpt:  [9.0575 GiB/s 9.1234 GiB/s 9.1844 GiB/s]
                 change:
                        time:   [-4.1480% -1.7392% +0.3116%] (p = 0.15 > 0.05)
                        thrpt:  [-0.3106% +1.7700% +4.3276%]
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

encode/10485760         time:   [965.84 µs 981.22 µs 999.75 µs]
                        thrpt:  [9.7681 GiB/s 9.9525 GiB/s 10.111 GiB/s]
                 change:
                        time:   [-2.1496% +1.3401% +4.8379%] (p = 0.49 > 0.05)
                        thrpt:  [-4.6147% -1.3224% +2.1968%]
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe

encode/20971520         time:   [1.9407 ms 1.9570 ms 1.9752 ms]
                        thrpt:  [9.8881 GiB/s 9.9804 GiB/s 10.064 GiB/s]
                 change:
                        time:   [-3.8047% -1.6402% +0.3331%] (p = 0.13 > 0.05)
                        thrpt:  [-0.3320% +1.6676% +3.9552%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe


decode/3                time:   [12.145 ns 12.155 ns 12.166 ns]
                        thrpt:  [235.16 MiB/s 235.39 MiB/s 235.58 MiB/s]

                 change:
-                        time:   [+5.4166% +5.5655% +5.7113%] (p = 0.00 < 0.05)
-                        thrpt:  [-5.4028% -5.2721% -5.1383%]
-                        Performance has regressed.
Found 17 outliers among 100 measurements (17.00%)
  8 (8.00%) high mild
  9 (9.00%) high severe

decode/50               time:   [26.187 ns 26.204 ns 26.223 ns]
                        thrpt:  [1.7758 GiB/s 1.7770 GiB/s 1.7782 GiB/s]
                 change:
                        time:   [+0.9544% +1.0927% +1.2358%] (p = 0.00 < 0.05)
                        thrpt:  [-1.2208% -1.0809% -0.9453%]
                        Change within noise threshold.
Found 17 outliers among 100 measurements (17.00%)
  8 (8.00%) high mild
  9 (9.00%) high severe

decode/100              time:   [40.203 ns 40.226 ns 40.251 ns]
                        thrpt:  [2.3138 GiB/s 2.3152 GiB/s 2.3166 GiB/s]
                 change:
                        time:   [+0.4981% +0.7443% +0.9406%] (p = 0.00 < 0.05)
                        thrpt:  [-0.9319% -0.7388% -0.4957%]
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) high mild
  11 (11.00%) high severe

decode/500              time:   [160.20 ns 160.32 ns 160.45 ns]
                        thrpt:  [2.9022 GiB/s 2.9046 GiB/s 2.9067 GiB/s]
                 change:
                        time:   [-0.1808% +0.0110% +0.1857%] (p = 0.91 > 0.05)
                        thrpt:  [-0.1853% -0.0110% +0.1811%]
                        No change in performance detected.
Found 19 outliers among 100 measurements (19.00%)
  10 (10.00%) high mild
  9 (9.00%) high severe

decode/3072             time:   [934.77 ns 935.38 ns 936.12 ns]
                        thrpt:  [3.0562 GiB/s 3.0587 GiB/s 3.0607 GiB/s]
                 change:
+                        time:   [-4.6618% -4.4940% -4.3268%] (p = 0.00 < 0.05)
+                        thrpt:  [+4.5225% +4.7055% +4.8897%]
+                        Performance has improved.
Found 17 outliers among 100 measurements (17.00%)
  10 (10.00%) high mild
  7 (7.00%) high severe

decode/51200            time:   [15.106 µs 15.116 µs 15.128 µs]
                        thrpt:  [3.1520 GiB/s 3.1544 GiB/s 3.1565 GiB/s]
                 change:
+                        time:   [-5.4585% -5.3087% -5.1576%] (p = 0.00 < 0.05)
+                        thrpt:  [+5.4381% +5.6063% +5.7736%]
+                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe

decode/102400           time:   [30.360 µs 30.382 µs 30.407 µs]
                        thrpt:  [3.1364 GiB/s 3.1390 GiB/s 3.1413 GiB/s]
                 change:
+                        time:   [-6.4617% -6.2312% -5.9893%] (p = 0.00 < 0.05)
+                        thrpt:  [+6.3709% +6.6453% +6.9081%]
+                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

decode/512000           time:   [72.583 µs 72.958 µs 73.368 µs]
                        thrpt:  [6.4992 GiB/s 6.5358 GiB/s 6.5696 GiB/s]
                 change:
                        time:   [-1.4523% -0.0292% +1.4024%] (p = 0.97 > 0.05)
                        thrpt:  [-1.3830% +0.0292% +1.4737%]
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe

decode/1048576          time:   [114.52 µs 115.81 µs 117.37 µs]
                        thrpt:  [8.3202 GiB/s 8.4321 GiB/s 8.5275 GiB/s]
                 change:
                        time:   [-3.5055% -1.4226% +0.8566%] (p = 0.20 > 0.05)
                        thrpt:  [-0.8493% +1.4432% +3.6328%]
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

decode/5242880          time:   [468.75 µs 471.76 µs 474.84 µs]
                        thrpt:  [10.283 GiB/s 10.350 GiB/s 10.417 GiB/s]
                 change:
+                        time:   [-7.2317% -4.6685% -2.0495%] (p = 0.00 < 0.05)
+                        thrpt:  [+2.0924% +4.8972% +7.7955%]
+                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

decode/10485760         time:   [879.57 µs 884.95 µs 890.70 µs]
                        thrpt:  [10.964 GiB/s 11.035 GiB/s 11.103 GiB/s]
                 change:
+                        time:   [-14.018% -11.631% -9.3755%] (p = 0.00 < 0.05)
+                        thrpt:  [+10.345% +13.162% +16.304%]
+                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

decode/20971520         time:   [1.6359 ms 1.6474 ms 1.6599 ms]
                        thrpt:  [11.766 GiB/s 11.856 GiB/s 11.939 GiB/s]
                 change:
                        time:   [-2.9030% -0.5720% +1.4162%] (p = 0.62 > 0.05)
                        thrpt:  [-1.3964% +0.5753% +2.9898%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe



```

