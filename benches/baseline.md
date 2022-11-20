# Profiling Report
```diff

encode/3                time:   [34.134 ns 34.251 ns 34.394 ns]
                        thrpt:  [83.184 MiB/s 83.531 MiB/s 83.817 MiB/s]
                 change:
-                        time:   [+1.8224% +2.2457% +2.7271%] (p = 0.00 < 0.05)
-                        thrpt:  [-2.6547% -2.1964% -1.7898%]
-                        Performance has regressed.

encode/50               time:   [46.895 ns 47.000 ns 47.107 ns]
                        thrpt:  [1012.2 MiB/s 1014.5 MiB/s 1016.8 MiB/s]
                 change:
+                        time:   [-1.8591% -1.6516% -1.4260%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.4467% +1.6794% +1.8943%]
+                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

encode/100              time:   [64.502 ns 64.633 ns 64.774 ns]
                        thrpt:  [1.4378 GiB/s 1.4409 GiB/s 1.4439 GiB/s]
                 change:
+                        time:   [-2.6919% -2.4414% -2.1819%] (p = 0.00 < 0.05)
+                        thrpt:  [+2.2306% +2.5025% +2.7664%]
+                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

encode/500              time:   [217.38 ns 217.87 ns 218.33 ns]
                        thrpt:  [2.1328 GiB/s 2.1373 GiB/s 2.1422 GiB/s]
                 change:
+                        time:   [-2.1196% -1.8856% -1.6309%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.6579% +1.9219% +2.1655%]
+                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

encode/3072             time:   [1.0208 µs 1.0231 µs 1.0257 µs]
                        thrpt:  [2.7895 GiB/s 2.7964 GiB/s 2.8027 GiB/s]
                 change:
                        time:   [-0.7445% -0.5162% -0.2904%] (p = 0.00 < 0.05)
                        thrpt:  [+0.2913% +0.5189% +0.7501%]
                        Change within noise threshold.

encode/51200            time:   [16.905 µs 16.942 µs 16.982 µs]
                        thrpt:  [2.8079 GiB/s 2.8146 GiB/s 2.8207 GiB/s]
                 change:
                        time:   [-0.5160% -0.2753% -0.0226%] (p = 0.04 < 0.05)
                        thrpt:  [+0.0226% +0.2761% +0.5187%]
                        Change within noise threshold.

encode/102400           time:   [33.271 µs 33.349 µs 33.428 µs]
                        thrpt:  [2.8530 GiB/s 2.8596 GiB/s 2.8664 GiB/s]
                 change:
+                        time:   [-1.9748% -1.5076% -1.0890%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.1010% +1.5307% +2.0145%]
+                        Performance has improved.

encode/512000           time:   [189.05 µs 190.11 µs 191.40 µs]
                        thrpt:  [2.4913 GiB/s 2.5082 GiB/s 2.5222 GiB/s]
                 change:
-                        time:   [+14.231% +15.284% +16.702%] (p = 0.00 < 0.05)
-                        thrpt:  [-14.312% -13.257% -12.458%]
-                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  5 (5.00%) high severe

encode/1048576          time:   [328.99 µs 332.27 µs 336.75 µs]
                        thrpt:  [2.9000 GiB/s 2.9391 GiB/s 2.9684 GiB/s]
                 change:
+                        time:   [-3.3390% -2.4633% -1.5122%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.5354% +2.5256% +3.4543%]
+                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

encode/5242880          time:   [1.7190 ms 1.7264 ms 1.7347 ms]
                        thrpt:  [2.8148 GiB/s 2.8283 GiB/s 2.8404 GiB/s]
                 change:
+                        time:   [-15.201% -14.732% -14.264%] (p = 0.00 < 0.05)
+                        thrpt:  [+16.637% +17.278% +17.927%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe

encode/10485760         time:   [3.3211 ms 3.3324 ms 3.3444 ms]
                        thrpt:  [2.9200 GiB/s 2.9305 GiB/s 2.9405 GiB/s]
                 change:
+                        time:   [-20.681% -20.404% -20.076%] (p = 0.00 < 0.05)
+                        thrpt:  [+25.119% +25.634% +26.073%]
+                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

encode/20971520         time:   [6.6268 ms 6.7633 ms 6.9707 ms]
                        thrpt:  [2.8019 GiB/s 2.8878 GiB/s 2.9473 GiB/s]
                 change:
+                        time:   [-21.206% -19.637% -17.213%] (p = 0.00 < 0.05)
+                        thrpt:  [+20.792% +24.436% +26.913%]
+                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  9 (9.00%) high severe



decode/3                time:   [30.990 ns 31.007 ns 31.027 ns]
                        thrpt:  [92.212 MiB/s 92.270 MiB/s 92.321 MiB/s]
                 change:
                        time:   [-0.1231% +0.0463% +0.2555%] (p = 0.64 > 0.05)
                        thrpt:  [-0.2549% -0.0463% +0.1233%]
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  8 (8.00%) high mild
  8 (8.00%) high severe

decode/50               time:   [45.798 ns 45.832 ns 45.872 ns]
                        thrpt:  [1.0151 GiB/s 1.0160 GiB/s 1.0168 GiB/s]
                 change:
                        time:   [-0.2955% -0.1745% -0.0442%] (p = 0.01 < 0.05)
                        thrpt:  [+0.0442% +0.1748% +0.2964%]
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe

decode/100              time:   [60.504 ns 60.575 ns 60.662 ns]
                        thrpt:  [1.5353 GiB/s 1.5375 GiB/s 1.5393 GiB/s]
                 change:
                        time:   [-1.5436% -1.2419% -0.9161%] (p = 0.00 < 0.05)
                        thrpt:  [+0.9246% +1.2575% +1.5678%]
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  9 (9.00%) high severe

decode/500              time:   [204.76 ns 205.13 ns 205.54 ns]
                        thrpt:  [2.2655 GiB/s 2.2701 GiB/s 2.2741 GiB/s]
                 change:
                        time:   [-0.5252% -0.3248% -0.1026%] (p = 0.00 < 0.05)
                        thrpt:  [+0.1027% +0.3258% +0.5279%]
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

decode/3072             time:   [989.38 ns 990.09 ns 990.96 ns]
                        thrpt:  [2.8871 GiB/s 2.8897 GiB/s 2.8917 GiB/s]
                 change:
                        time:   [-0.2729% -0.1325% -0.0037%] (p = 0.06 > 0.05)
                        thrpt:  [+0.0037% +0.1327% +0.2737%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

decode/51200            time:   [16.086 µs 16.100 µs 16.112 µs]
                        thrpt:  [2.9595 GiB/s 2.9618 GiB/s 2.9643 GiB/s]
                 change:
                        time:   [-0.3344% -0.1458% +0.0468%] (p = 0.13 > 0.05)
                        thrpt:  [-0.0468% +0.1461% +0.3355%]
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

decode/102400           time:   [31.882 µs 31.923 µs 31.969 µs]
                        thrpt:  [2.9831 GiB/s 2.9874 GiB/s 2.9913 GiB/s]
                 change:
                        time:   [-0.5842% -0.3681% -0.1432%] (p = 0.00 < 0.05)
                        thrpt:  [+0.1434% +0.3695% +0.5877%]
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

decode/512000           time:   [132.80 µs 134.42 µs 136.52 µs]
                        thrpt:  [3.4928 GiB/s 3.5474 GiB/s 3.5906 GiB/s]
                 change:
+                        time:   [-15.144% -14.336% -13.355%] (p = 0.00 < 0.05)
+                        thrpt:  [+15.413% +16.735% +17.847%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  7 (7.00%) high severe

decode/1048576          time:   [216.35 µs 217.42 µs 218.55 µs]
                        thrpt:  [4.4684 GiB/s 4.4916 GiB/s 4.5139 GiB/s]
                 change:
+                        time:   [-32.850% -32.378% -31.830%] (p = 0.00 < 0.05)
+                        thrpt:  [+46.691% +47.880% +48.921%]
+                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe

decode/5242880          time:   [995.80 µs 1.0001 ms 1.0052 ms]
                        thrpt:  [4.8577 GiB/s 4.8822 GiB/s 4.9034 GiB/s]
                 change:
+                        time:   [-43.461% -43.013% -42.484%] (p = 0.00 < 0.05)
+                        thrpt:  [+73.866% +75.477% +76.868%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe

decode/10485760         time:   [1.9661 ms 1.9771 ms 1.9900 ms]
                        thrpt:  [4.9075 GiB/s 4.9393 GiB/s 4.9669 GiB/s]
                 change:
+                        time:   [-46.336% -45.835% -45.357%] (p = 0.00 < 0.05)
+                        thrpt:  [+83.005% +84.622% +86.344%]
+                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  6 (6.00%) high mild
  9 (9.00%) high severe

decode/20971520         time:   [3.9075 ms 3.9275 ms 3.9519 ms]
                        thrpt:  [4.9422 GiB/s 4.9729 GiB/s 4.9985 GiB/s]
                 change:
+                        time:   [-47.810% -47.521% -47.190%] (p = 0.00 < 0.05)
+                        thrpt:  [+89.358% +90.553% +91.608%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe



```

