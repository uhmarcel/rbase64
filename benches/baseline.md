# Profiling Report
```diff

encode/3                time:   [15.116 ns 15.148 ns 15.182 ns]
                        thrpt:  [188.45 MiB/s 188.87 MiB/s 189.27 MiB/s]
                 change:
-                        time:   [+5.1230% +5.4680% +5.8398%] (p = 0.00 < 0.05)
-                        thrpt:  [-5.5176% -5.1845% -4.8734%]
-                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

encode/50               time:   [33.895 ns 34.248 ns 34.701 ns]
                        thrpt:  [1.3419 GiB/s 1.3597 GiB/s 1.3738 GiB/s]
                 change:
-                        time:   [+6.6131% +7.1700% +7.9376%] (p = 0.00 < 0.05)
-                        thrpt:  [-7.3539% -6.6903% -6.2029%]
-                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

encode/100              time:   [51.996 ns 52.132 ns 52.266 ns]
                        thrpt:  [1.7819 GiB/s 1.7865 GiB/s 1.7911 GiB/s]
                 change:
-                        time:   [+9.7035% +9.9549% +10.190%] (p = 0.00 < 0.05)
-                        thrpt:  [-9.2473% -9.0536% -8.8452%]
-                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

encode/500              time:   [178.41 ns 178.89 ns 179.42 ns]
                        thrpt:  [2.5953 GiB/s 2.6030 GiB/s 2.6101 GiB/s]
                 change:
+                        time:   [-3.8960% -3.6476% -3.3757%] (p = 0.00 < 0.05)
+                        thrpt:  [+3.4937% +3.7857% +4.0539%]
+                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

encode/3072             time:   [1.0197 µs 1.0249 µs 1.0314 µs]
                        thrpt:  [2.7740 GiB/s 2.7916 GiB/s 2.8057 GiB/s]
                 change:
+                        time:   [-6.0015% -5.7201% -5.4016%] (p = 0.00 < 0.05)
+                        thrpt:  [+5.7100% +6.0671% +6.3847%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

encode/51200            time:   [16.771 µs 16.809 µs 16.846 µs]
                        thrpt:  [2.8305 GiB/s 2.8368 GiB/s 2.8433 GiB/s]
                 change:
+                        time:   [-5.8752% -5.6407% -5.3936%] (p = 0.00 < 0.05)
+                        thrpt:  [+5.7011% +5.9779% +6.2420%]
+                        Performance has improved.

encode/102400           time:   [33.890 µs 33.984 µs 34.079 µs]
                        thrpt:  [2.7984 GiB/s 2.8062 GiB/s 2.8141 GiB/s]
                 change:
+                        time:   [-5.8614% -5.5709% -5.2816%] (p = 0.00 < 0.05)
+                        thrpt:  [+5.5761% +5.8996% +6.2263%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

encode/512000           time:   [91.991 µs 94.548 µs 97.600 µs]
                        thrpt:  [4.8857 GiB/s 5.0433 GiB/s 5.1835 GiB/s]
                 change:
-                        time:   [+10.717% +13.198% +16.202%] (p = 0.00 < 0.05)
-                        thrpt:  [-13.943% -11.659% -9.6793%]
-                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

encode/1048576          time:   [129.44 µs 133.61 µs 138.59 µs]
                        thrpt:  [7.0466 GiB/s 7.3092 GiB/s 7.5442 GiB/s]
                 change:
                        time:   [+0.0822% +3.7289% +8.3565%] (p = 0.09 > 0.05)
                        thrpt:  [-7.7120% -3.5948% -0.0822%]
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

encode/5242880          time:   [494.07 µs 503.41 µs 514.28 µs]
                        thrpt:  [9.4944 GiB/s 9.6995 GiB/s 9.8828 GiB/s]
                 change:
+                        time:   [-8.9035% -6.5357% -4.3286%] (p = 0.00 < 0.05)
+                        thrpt:  [+4.5245% +6.9927% +9.7737%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe

encode/10485760         time:   [875.57 µs 886.25 µs 901.21 µs]
                        thrpt:  [10.836 GiB/s 11.019 GiB/s 11.153 GiB/s]
                 change:
                        time:   [-5.8388% -2.6162% +0.7802%] (p = 0.11 > 0.05)
                        thrpt:  [-0.7742% +2.6865% +6.2008%]
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  9 (9.00%) high mild
  8 (8.00%) high severe

encode/20971520         time:   [1.8038 ms 1.8250 ms 1.8501 ms]
                        thrpt:  [10.557 GiB/s 10.702 GiB/s 10.828 GiB/s]
                 change:
+                        time:   [-6.8473% -5.5696% -4.2083%] (p = 0.00 < 0.05)
+                        thrpt:  [+4.3932% +5.8981% +7.3506%]
+                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe



decode/3                time:   [12.140 ns 12.146 ns 12.154 ns]
                        thrpt:  [235.39 MiB/s 235.55 MiB/s 235.67 MiB/s]
                 change:
-                        time:   [+5.4347% +5.5490% +5.6620%] (p = 0.00 < 0.05)
-                        thrpt:  [-5.3586% -5.2573% -5.1546%]
-                        Performance has regressed.
Found 16 outliers among 100 measurements (16.00%)
  6 (6.00%) high mild
  10 (10.00%) high severe

decode/50               time:   [26.184 ns 26.198 ns 26.216 ns]
                        thrpt:  [1.7763 GiB/s 1.7775 GiB/s 1.7784 GiB/s]
                 change:
-                        time:   [+1.1526% +1.2904% +1.4243%] (p = 0.00 < 0.05)
-                        thrpt:  [-1.4043% -1.2740% -1.1395%]
-                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

decode/100              time:   [40.188 ns 40.226 ns 40.273 ns]
                        thrpt:  [2.3125 GiB/s 2.3152 GiB/s 2.3174 GiB/s]
                 change:
                        time:   [+0.3034% +0.4172% +0.5416%] (p = 0.00 < 0.05)
                        thrpt:  [-0.5387% -0.4155% -0.3025%]
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

decode/500              time:   [160.78 ns 160.88 ns 161.00 ns]
                        thrpt:  [2.8923 GiB/s 2.8945 GiB/s 2.8963 GiB/s]
                 change:
                        time:   [+0.4141% +0.5785% +0.7231%] (p = 0.00 < 0.05)
                        thrpt:  [-0.7179% -0.5751% -0.4124%]
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe

decode/3072             time:   [932.79 ns 933.28 ns 933.85 ns]
                        thrpt:  [3.0637 GiB/s 3.0656 GiB/s 3.0672 GiB/s]
                 change:
+                        time:   [-11.193% -7.2177% -3.6424%] (p = 0.00 < 0.05)
+                        thrpt:  [+3.7800% +7.7791% +12.603%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

decode/51200            time:   [15.107 µs 15.124 µs 15.146 µs]
                        thrpt:  [3.1483 GiB/s 3.1528 GiB/s 3.1564 GiB/s]
                 change:
                        time:   [-0.1177% +0.0381% +0.2029%] (p = 0.64 > 0.05)
                        thrpt:  [-0.2024% -0.0381% +0.1179%]
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe

decode/102400           time:   [30.422 µs 30.455 µs 30.503 µs]
                        thrpt:  [3.1265 GiB/s 3.1314 GiB/s 3.1349 GiB/s]
                 change:
                        time:   [-0.1144% +0.1157% +0.3507%] (p = 0.32 > 0.05)
                        thrpt:  [-0.3495% -0.1156% +0.1145%]
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) high mild
  11 (11.00%) high severe

decode/512000           time:   [72.939 µs 73.843 µs 75.013 µs]
                        thrpt:  [6.3567 GiB/s 6.4574 GiB/s 6.5375 GiB/s]
                 change:
                        time:   [+0.2139% +2.0645% +4.1311%] (p = 0.03 < 0.05)
                        thrpt:  [-3.9672% -2.0228% -0.2135%]
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe

decode/1048576          time:   [112.49 µs 113.29 µs 114.26 µs]
                        thrpt:  [8.5465 GiB/s 8.6202 GiB/s 8.6814 GiB/s]
                 change:
                        time:   [-1.7913% -0.5868% +0.5738%] (p = 0.34 > 0.05)
                        thrpt:  [-0.5705% +0.5902% +1.8240%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  5 (5.00%) high severe

decode/5242880          time:   [465.69 µs 469.06 µs 472.97 µs]
                        thrpt:  [10.324 GiB/s 10.410 GiB/s 10.485 GiB/s]
                 change:
                        time:   [-0.6782% +0.6698% +1.9738%] (p = 0.34 > 0.05)
                        thrpt:  [-1.9356% -0.6654% +0.6828%]
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

decode/10485760         time:   [860.09 µs 867.48 µs 876.64 µs]
                        thrpt:  [11.140 GiB/s 11.257 GiB/s 11.354 GiB/s]
                 change:
                        time:   [+0.0318% +1.8799% +3.7734%] (p = 0.05 < 0.05)
                        thrpt:  [-3.6362% -1.8452% -0.0318%]
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

decode/20971520         time:   [1.6183 ms 1.6305 ms 1.6445 ms]
                        thrpt:  [11.876 GiB/s 11.979 GiB/s 12.069 GiB/s]
                 change:
                        time:   [-2.9757% -0.3468% +1.7382%] (p = 0.81 > 0.05)
                        thrpt:  [-1.7085% +0.3480% +3.0669%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe



```

