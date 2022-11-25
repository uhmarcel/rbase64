# Profiling Report
```diff

encode/3                time:   [14.648 ns 14.754 ns 14.889 ns]
                        thrpt:  [192.16 MiB/s 193.91 MiB/s 195.32 MiB/s]
                 change:
+                        time:   [-56.854% -56.669% -56.452%] (p = 0.00 < 0.05)
+                        thrpt:  [+129.63% +130.78% +131.77%]
+                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

encode/50               time:   [32.246 ns 32.374 ns 32.510 ns]
                        thrpt:  [1.4324 GiB/s 1.4384 GiB/s 1.4441 GiB/s]
                 change:
+                        time:   [-34.350% -33.896% -33.511%] (p = 0.00 < 0.05)
+                        thrpt:  [+50.401% +51.277% +52.322%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  11 (11.00%) high mild
  2 (2.00%) high severe

encode/100              time:   [48.261 ns 48.364 ns 48.480 ns]
                        thrpt:  [1.9211 GiB/s 1.9257 GiB/s 1.9298 GiB/s]
                 change:
+                        time:   [-27.812% -27.603% -27.386%] (p = 0.00 < 0.05)
+                        thrpt:  [+37.715% +38.127% +38.527%]
+                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

encode/500              time:   [187.11 ns 187.46 ns 187.85 ns]
                        thrpt:  [2.4789 GiB/s 2.4840 GiB/s 2.4887 GiB/s]
                 change:
+                        time:   [-17.046% -16.595% -16.185%] (p = 0.00 < 0.05)
+                        thrpt:  [+19.311% +19.896% +20.548%]
+                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

encode/3072             time:   [1.0921 µs 1.0943 µs 1.0967 µs]
                        thrpt:  [2.6088 GiB/s 2.6145 GiB/s 2.6196 GiB/s]
                 change:
                        time:   [-0.8570% -0.6570% -0.4360%] (p = 0.00 < 0.05)
                        thrpt:  [+0.4379% +0.6614% +0.8644%]
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  11 (11.00%) high mild
  3 (3.00%) high severe

encode/51200            time:   [17.761 µs 17.793 µs 17.829 µs]
                        thrpt:  [2.6745 GiB/s 2.6800 GiB/s 2.6847 GiB/s]
                 change:
                        time:   [-1.3001% -1.0902% -0.8854%] (p = 0.00 < 0.05)
                        thrpt:  [+0.8933% +1.1022% +1.3172%]
                        Change within noise threshold.
Found 18 outliers among 100 measurements (18.00%)
  4 (4.00%) high mild
  14 (14.00%) high severe

encode/102400           time:   [36.107 µs 36.171 µs 36.250 µs]
                        thrpt:  [2.6308 GiB/s 2.6366 GiB/s 2.6412 GiB/s]
                 change:
-                        time:   [+1.3870% +2.4016% +3.5761%] (p = 0.00 < 0.05)
-                        thrpt:  [-3.4526% -2.3453% -1.3680%]
-                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

encode/512000           time:   [86.020 µs 87.044 µs 88.301 µs]
                        thrpt:  [5.4002 GiB/s 5.4781 GiB/s 5.5433 GiB/s]
                 change:
                        time:   [-3.4090% -0.7912% +1.9072%] (p = 0.57 > 0.05)
                        thrpt:  [-1.8715% +0.7975% +3.5293%]
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  9 (9.00%) high severe

encode/1048576          time:   [132.10 µs 134.11 µs 136.61 µs]
                        thrpt:  [7.1487 GiB/s 7.2816 GiB/s 7.3924 GiB/s]
                 change:
                        time:   [-7.5664% -1.2481% +3.5855%] (p = 0.73 > 0.05)
                        thrpt:  [-3.4614% +1.2639% +8.1857%]
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

encode/5242880          time:   [572.64 µs 589.76 µs 612.09 µs]
                        thrpt:  [7.9773 GiB/s 8.2793 GiB/s 8.5268 GiB/s]
                 change:
                        time:   [-22.508% -6.8984% +17.577%] (p = 0.58 > 0.05)
                        thrpt:  [-14.949% +7.4095% +29.045%]
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  8 (8.00%) high mild
  5 (5.00%) high severe

encode/10485760         time:   [988.34 µs 1.0120 ms 1.0428 ms]
                        thrpt:  [9.3644 GiB/s 9.6503 GiB/s 9.8809 GiB/s]
                 change:
+                        time:   [-36.431% -31.070% -22.099%] (p = 0.00 < 0.05)
+                        thrpt:  [+28.369% +45.074% +57.308%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe

encode/20971520         time:   [1.9848 ms 2.0287 ms 2.0801 ms]
                        thrpt:  [9.3898 GiB/s 9.6276 GiB/s 9.8403 GiB/s]
                 change:
+                        time:   [-34.245% -32.706% -31.123%] (p = 0.00 < 0.05)
+                        thrpt:  [+45.186% +48.601% +52.080%]
+                        Performance has improved.
Found 17 outliers among 100 measurements (17.00%)
  2 (2.00%) high mild
  15 (15.00%) high severe



decode/3                time:   [11.521 ns 11.534 ns 11.552 ns]
                        thrpt:  [247.67 MiB/s 248.04 MiB/s 248.33 MiB/s]
                 change:
+                        time:   [-63.004% -62.820% -62.591%] (p = 0.00 < 0.05)
+                        thrpt:  [+167.31% +168.96% +170.30%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe

decode/50               time:   [25.978 ns 26.010 ns 26.047 ns]
                        thrpt:  [1.7878 GiB/s 1.7903 GiB/s 1.7925 GiB/s]
                 change:
+                        time:   [-43.123% -43.032% -42.948%] (p = 0.00 < 0.05)
+                        thrpt:  [+75.278% +75.539% +75.819%]
+                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  8 (8.00%) high mild
  7 (7.00%) high severe

decode/100              time:   [39.933 ns 39.966 ns 40.004 ns]
                        thrpt:  [2.3281 GiB/s 2.3303 GiB/s 2.3322 GiB/s]
                 change:
+                        time:   [-33.123% -32.995% -32.867%] (p = 0.00 < 0.05)
+                        thrpt:  [+48.959% +49.242% +49.528%]
+                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

decode/500              time:   [160.72 ns 161.72 ns 162.73 ns]
                        thrpt:  [2.8615 GiB/s 2.8794 GiB/s 2.8973 GiB/s]
                 change:
+                        time:   [-22.699% -22.428% -22.141%] (p = 0.00 < 0.05)
+                        thrpt:  [+28.438% +28.913% +29.364%]
+                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) high mild
  12 (12.00%) high severe

decode/3072             time:   [983.07 ns 984.40 ns 985.86 ns]
                        thrpt:  [2.9021 GiB/s 2.9064 GiB/s 2.9103 GiB/s]
                 change:
-                        time:   [+3.0342% +3.3521% +3.6500%] (p = 0.00 < 0.05)
-                        thrpt:  [-3.5215% -3.2434% -2.9449%]
-                        Performance has regressed.
Found 25 outliers among 100 measurements (25.00%)
  12 (12.00%) low severe
  1 (1.00%) low mild
  9 (9.00%) high mild
  3 (3.00%) high severe

decode/51200            time:   [15.980 µs 15.997 µs 16.016 µs]
                        thrpt:  [2.9772 GiB/s 2.9808 GiB/s 2.9839 GiB/s]
                 change:
-                        time:   [+1.1583% +1.6095% +2.0147%] (p = 0.00 < 0.05)
-                        thrpt:  [-1.9749% -1.5840% -1.1451%]
-                        Performance has regressed.
Found 18 outliers among 100 measurements (18.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  12 (12.00%) high severe

decode/102400           time:   [32.138 µs 32.215 µs 32.296 µs]
                        thrpt:  [2.9529 GiB/s 2.9604 GiB/s 2.9674 GiB/s]
                 change:
-                        time:   [+3.4872% +3.9039% +4.3138%] (p = 0.00 < 0.05)
-                        thrpt:  [-4.1354% -3.7572% -3.3697%]
-                        Performance has regressed.

decode/512000           time:   [72.648 µs 73.348 µs 74.234 µs]
                        thrpt:  [6.4234 GiB/s 6.5011 GiB/s 6.5636 GiB/s]
                 change:
                        time:   [-1.0063% +1.1548% +3.4007%] (p = 0.34 > 0.05)
                        thrpt:  [-3.2888% -1.1416% +1.0165%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

decode/1048576          time:   [117.08 µs 122.81 µs 130.58 µs]
                        thrpt:  [7.4785 GiB/s 7.9518 GiB/s 8.3409 GiB/s]
                 change:
-                        time:   [+1.2322% +4.6764% +8.6586%] (p = 0.01 < 0.05)
-                        thrpt:  [-7.9686% -4.4675% -1.2172%]
-                        Performance has regressed.
Found 15 outliers among 100 measurements (15.00%)
  9 (9.00%) high mild
  6 (6.00%) high severe

decode/5242880          time:   [470.18 µs 477.18 µs 485.90 µs]
                        thrpt:  [10.049 GiB/s 10.233 GiB/s 10.385 GiB/s]
                 change:
+                        time:   [-26.131% -24.223% -22.248%] (p = 0.00 < 0.05)
+                        thrpt:  [+28.615% +31.967% +35.376%]
+                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  9 (9.00%) high mild
  5 (5.00%) high severe

decode/10485760         time:   [872.03 µs 883.41 µs 898.21 µs]
                        thrpt:  [10.872 GiB/s 11.054 GiB/s 11.199 GiB/s]
                 change:
+                        time:   [-31.442% -29.737% -27.813%] (p = 0.00 < 0.05)
+                        thrpt:  [+38.529% +42.323% +45.863%]
+                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

decode/20971520         time:   [1.6261 ms 1.6428 ms 1.6613 ms]
                        thrpt:  [11.757 GiB/s 11.889 GiB/s 12.011 GiB/s]
                 change:
+                        time:   [-28.748% -27.867% -27.005%] (p = 0.00 < 0.05)
+                        thrpt:  [+36.995% +38.634% +40.346%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  11 (11.00%) high mild
  2 (2.00%) high severe



```

