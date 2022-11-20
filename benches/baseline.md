# Profiling Report
```diff

encode/3                time:   [32.822 ns 32.909 ns 32.995 ns]
                        thrpt:  [86.711 MiB/s 86.939 MiB/s 87.167 MiB/s]
                 change:
+                        time:   [-2.9327% -2.7108% -2.4942%] (p = 0.00 < 0.05)
+                        thrpt:  [+2.5580% +2.7863% +3.0213%]
+                        Performance has improved.

encode/50               time:   [46.908 ns 47.022 ns 47.147 ns]
                        thrpt:  [1011.4 MiB/s 1014.1 MiB/s 1016.5 MiB/s]
                 change:
                        time:   [+0.6613% +0.9092% +1.1396%] (p = 0.00 < 0.05)
                        thrpt:  [-1.1267% -0.9010% -0.6569%]
                        Change within noise threshold.

encode/100              time:   [64.656 ns 64.775 ns 64.900 ns]
                        thrpt:  [1.4350 GiB/s 1.4378 GiB/s 1.4404 GiB/s]
                 change:
                        time:   [-0.5186% -0.3070% -0.0948%] (p = 0.01 < 0.05)
                        thrpt:  [+0.0949% +0.3079% +0.5213%]
                        Change within noise threshold.

encode/500              time:   [220.36 ns 222.16 ns 223.97 ns]
                        thrpt:  [2.0791 GiB/s 2.0961 GiB/s 2.1132 GiB/s]
                 change:
                        time:   [-1.3576% -0.6646% +0.0221%] (p = 0.06 > 0.05)
                        thrpt:  [-0.0221% +0.6690% +1.3763%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  11 (11.00%) high mild
  1 (1.00%) high severe

encode/3072             time:   [1.0918 µs 1.0948 µs 1.0978 µs]
                        thrpt:  [2.6062 GiB/s 2.6132 GiB/s 2.6205 GiB/s]
                 change:
-                        time:   [+6.3420% +6.6727% +7.0063%] (p = 0.00 < 0.05)
-                        thrpt:  [-6.5475% -6.2553% -5.9637%]
-                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

encode/51200            time:   [17.755 µs 17.811 µs 17.877 µs]
                        thrpt:  [2.6673 GiB/s 2.6772 GiB/s 2.6857 GiB/s]
                 change:
-                        time:   [+4.3829% +4.7380% +5.1323%] (p = 0.00 < 0.05)
-                        thrpt:  [-4.8817% -4.5237% -4.1988%]
-                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

encode/102400           time:   [35.812 µs 36.070 µs 36.460 µs]
                        thrpt:  [2.6156 GiB/s 2.6439 GiB/s 2.6630 GiB/s]
                 change:
-                        time:   [+6.9476% +7.3791% +7.9414%] (p = 0.00 < 0.05)
-                        thrpt:  [-7.3572% -6.8720% -6.4963%]
-                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

encode/512000           time:   [185.41 µs 187.42 µs 189.99 µs]
                        thrpt:  [2.5098 GiB/s 2.5442 GiB/s 2.5717 GiB/s]
                 change:
-                        time:   [+14.295% +15.656% +17.076%] (p = 0.00 < 0.05)
-                        thrpt:  [-14.585% -13.536% -12.507%]
-                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

encode/1048576          time:   [316.04 µs 317.87 µs 319.76 µs]
                        thrpt:  [3.0540 GiB/s 3.0722 GiB/s 3.0900 GiB/s]
                 change:
+                        time:   [-5.6030% -4.9228% -4.1911%] (p = 0.00 < 0.05)
+                        thrpt:  [+4.3744% +5.1777% +5.9355%]
+                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

encode/5242880          time:   [1.6437 ms 1.6544 ms 1.6680 ms]
                        thrpt:  [2.9273 GiB/s 2.9514 GiB/s 2.9706 GiB/s]
                 change:
+                        time:   [-16.736% -16.154% -15.430%] (p = 0.00 < 0.05)
+                        thrpt:  [+18.245% +19.266% +20.100%]
+                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe

encode/10485760         time:   [3.1504 ms 3.1635 ms 3.1781 ms]
                        thrpt:  [3.0728 GiB/s 3.0869 GiB/s 3.0998 GiB/s]
                 change:
+                        time:   [-23.759% -23.429% -23.039%] (p = 0.00 < 0.05)
+                        thrpt:  [+29.936% +30.597% +31.164%]
+                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

encode/20971520         time:   [6.1696 ms 6.1951 ms 6.2243 ms]
                        thrpt:  [3.1379 GiB/s 3.1527 GiB/s 3.1657 GiB/s]
                 change:
+                        time:   [-26.035% -25.721% -25.363%] (p = 0.00 < 0.05)
+                        thrpt:  [+33.981% +34.627% +35.198%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  9 (9.00%) high mild
  4 (4.00%) high severe



decode/3                time:   [30.988 ns 31.023 ns 31.074 ns]
                        thrpt:  [92.070 MiB/s 92.222 MiB/s 92.326 MiB/s]
                 change:
-                        time:   [+1.0416% +1.2609% +1.4928%] (p = 0.00 < 0.05)
-                        thrpt:  [-1.4709% -1.2452% -1.0309%]
-                        Performance has regressed.
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe

decode/50               time:   [45.830 ns 45.871 ns 45.917 ns]
                        thrpt:  [1.0141 GiB/s 1.0152 GiB/s 1.0161 GiB/s]
                 change:
                        time:   [+0.7279% +0.9084% +1.0900%] (p = 0.00 < 0.05)
                        thrpt:  [-1.0782% -0.9002% -0.7226%]
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

decode/100              time:   [60.502 ns 60.557 ns 60.632 ns]
                        thrpt:  [1.5360 GiB/s 1.5379 GiB/s 1.5393 GiB/s]
                 change:
                        time:   [-0.0975% +0.2574% +0.5658%] (p = 0.14 > 0.05)
                        thrpt:  [-0.5626% -0.2568% +0.0976%]
                        No change in performance detected.
Found 19 outliers among 100 measurements (19.00%)
  2 (2.00%) low mild
  7 (7.00%) high mild
  10 (10.00%) high severe

decode/500              time:   [206.07 ns 206.69 ns 207.44 ns]
                        thrpt:  [2.2448 GiB/s 2.2530 GiB/s 2.2598 GiB/s]
                 change:
-                        time:   [+1.9200% +2.3505% +2.7822%] (p = 0.00 < 0.05)
-                        thrpt:  [-2.7069% -2.2965% -1.8838%]
-                        Performance has regressed.

decode/3072             time:   [989.59 ns 990.13 ns 990.76 ns]
                        thrpt:  [2.8877 GiB/s 2.8895 GiB/s 2.8911 GiB/s]
                 change:
                        time:   [+0.6561% +0.9185% +1.1689%] (p = 0.00 < 0.05)
                        thrpt:  [-1.1554% -0.9102% -0.6518%]
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe

decode/51200            time:   [16.126 µs 16.139 µs 16.152 µs]
                        thrpt:  [2.9523 GiB/s 2.9546 GiB/s 2.9570 GiB/s]
                 change:
                        time:   [+0.1951% +0.4036% +0.6263%] (p = 0.00 < 0.05)
                        thrpt:  [-0.6224% -0.4020% -0.1947%]
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe

decode/102400           time:   [31.898 µs 32.094 µs 32.474 µs]
                        thrpt:  [2.9368 GiB/s 2.9715 GiB/s 2.9897 GiB/s]
                 change:
                        time:   [-0.0035% +0.3915% +0.9650%] (p = 0.12 > 0.05)
                        thrpt:  [-0.9558% -0.3899% +0.0035%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

decode/512000           time:   [154.53 µs 156.12 µs 158.11 µs]
                        thrpt:  [3.0159 GiB/s 3.0543 GiB/s 3.0857 GiB/s]
                 change:
                        time:   [-1.0059% -0.1302% +0.8451%] (p = 0.80 > 0.05)
                        thrpt:  [-0.8380% +0.1304% +1.0162%]
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

decode/1048576          time:   [257.80 µs 258.79 µs 259.97 µs]
                        thrpt:  [3.7564 GiB/s 3.7735 GiB/s 3.7880 GiB/s]
                 change:
+                        time:   [-19.209% -18.224% -17.225%] (p = 0.00 < 0.05)
+                        thrpt:  [+20.809% +22.285% +23.776%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe

decode/5242880          time:   [1.1998 ms 1.2100 ms 1.2229 ms]
                        thrpt:  [3.9928 GiB/s 4.0354 GiB/s 4.0696 GiB/s]
                 change:
+                        time:   [-32.222% -31.610% -30.868%] (p = 0.00 < 0.05)
+                        thrpt:  [+44.650% +46.220% +47.540%]
+                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  8 (8.00%) high mild
  6 (6.00%) high severe

decode/10485760         time:   [2.3781 ms 2.3900 ms 2.4042 ms]
                        thrpt:  [4.0619 GiB/s 4.0861 GiB/s 4.1065 GiB/s]
                 change:
+                        time:   [-33.661% -33.246% -32.790%] (p = 0.00 < 0.05)
+                        thrpt:  [+48.786% +49.805% +50.741%]
+                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

decode/20971520         time:   [4.6632 ms 4.6912 ms 4.7264 ms]
                        thrpt:  [4.1323 GiB/s 4.1634 GiB/s 4.1884 GiB/s]
                 change:
+                        time:   [-37.751% -37.363% -36.893%] (p = 0.00 < 0.05)
+                        thrpt:  [+58.461% +59.650% +60.645%]
+                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe



```

