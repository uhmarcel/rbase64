# Profiling Report
```diff

encode/3                time:   [32.770 ns 32.850 ns 32.937 ns]
                        thrpt:  [86.865 MiB/s 87.092 MiB/s 87.306 MiB/s]
                 change:
+                        time:   [-2.8404% -2.5720% -2.2799%] (p = 0.00 < 0.05)
+                        thrpt:  [+2.3331% +2.6399% +2.9234%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

encode/50               time:   [47.257 ns 47.375 ns 47.487 ns]
                        thrpt:  [1004.1 MiB/s 1006.5 MiB/s 1009.0 MiB/s]
                 change:
-                        time:   [+1.1149% +1.3601% +1.5622%] (p = 0.00 < 0.05)
-                        thrpt:  [-1.5382% -1.3418% -1.1026%]
-                        Performance has regressed.

encode/100              time:   [64.621 ns 64.749 ns 64.890 ns]
                        thrpt:  [1.4352 GiB/s 1.4384 GiB/s 1.4412 GiB/s]
                 change:
                        time:   [-0.8094% -0.5885% -0.3729%] (p = 0.00 < 0.05)
                        thrpt:  [+0.3743% +0.5920% +0.8160%]
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

encode/500              time:   [221.44 ns 222.05 ns 222.70 ns]
                        thrpt:  [2.0910 GiB/s 2.0971 GiB/s 2.1029 GiB/s]
                 change:
                        time:   [-0.5849% -0.0091% +0.5641%] (p = 0.97 > 0.05)
                        thrpt:  [-0.5610% +0.0091% +0.5884%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  11 (11.00%) high mild

encode/3072             time:   [1.0874 µs 1.0904 µs 1.0935 µs]
                        thrpt:  [2.6164 GiB/s 2.6239 GiB/s 2.6311 GiB/s]
                 change:
-                        time:   [+6.2504% +6.5872% +6.9084%] (p = 0.00 < 0.05)
-                        thrpt:  [-6.4620% -6.1801% -5.8827%]
-                        Performance has regressed.

encode/51200            time:   [17.755 µs 17.797 µs 17.840 µs]
                        thrpt:  [2.6729 GiB/s 2.6794 GiB/s 2.6857 GiB/s]
                 change:
-                        time:   [+4.3477% +4.6782% +4.9829%] (p = 0.00 < 0.05)
-                        thrpt:  [-4.7464% -4.4691% -4.1665%]
-                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

encode/102400           time:   [35.520 µs 35.589 µs 35.663 µs]
                        thrpt:  [2.6741 GiB/s 2.6797 GiB/s 2.6849 GiB/s]
                 change:
-                        time:   [+6.3508% +6.6521% +6.9540%] (p = 0.00 < 0.05)
-                        thrpt:  [-6.5019% -6.2372% -5.9716%]
-                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

encode/512000           time:   [184.80 µs 185.63 µs 186.54 µs]
                        thrpt:  [2.5562 GiB/s 2.5687 GiB/s 2.5803 GiB/s]
                 change:
-                        time:   [+13.695% +14.455% +15.379%] (p = 0.00 < 0.05)
-                        thrpt:  [-13.329% -12.629% -12.045%]
-                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

encode/1048576          time:   [317.24 µs 318.87 µs 320.63 µs]
                        thrpt:  [3.0457 GiB/s 3.0626 GiB/s 3.0783 GiB/s]
                 change:
                        time:   [-3.5614% -1.2648% +1.8651%] (p = 0.35 > 0.05)
                        thrpt:  [-1.8310% +1.2810% +3.6930%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe

encode/5242880          time:   [1.6482 ms 1.6547 ms 1.6623 ms]
                        thrpt:  [2.9373 GiB/s 2.9508 GiB/s 2.9626 GiB/s]
                 change:
+                        time:   [-16.536% -16.137% -15.695%] (p = 0.00 < 0.05)
+                        thrpt:  [+18.617% +19.242% +19.812%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  9 (9.00%) high mild
  4 (4.00%) high severe

encode/10485760         time:   [3.1557 ms 3.1669 ms 3.1801 ms]
                        thrpt:  [3.0709 GiB/s 3.0836 GiB/s 3.0946 GiB/s]
                 change:
+                        time:   [-23.652% -23.347% -22.971%] (p = 0.00 < 0.05)
+                        thrpt:  [+29.821% +30.458% +30.980%]
+                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

encode/20971520         time:   [6.1868 ms 6.2046 ms 6.2270 ms]
                        thrpt:  [3.1366 GiB/s 3.1479 GiB/s 3.1569 GiB/s]
                 change:
+                        time:   [-25.886% -25.607% -25.293%] (p = 0.00 < 0.05)
+                        thrpt:  [+33.856% +34.422% +34.927%]
+                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe



decode/3                time:   [30.996 ns 31.032 ns 31.081 ns]
                        thrpt:  [92.050 MiB/s 92.197 MiB/s 92.304 MiB/s]
                 change:
-                        time:   [+1.1024% +1.3447% +1.5910%] (p = 0.00 < 0.05)
-                        thrpt:  [-1.5661% -1.3268% -1.0904%]
-                        Performance has regressed.
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe

decode/50               time:   [45.813 ns 45.836 ns 45.864 ns]
                        thrpt:  [1.0153 GiB/s 1.0159 GiB/s 1.0164 GiB/s]
                 change:
                        time:   [+0.6241% +0.7786% +0.9427%] (p = 0.00 < 0.05)
                        thrpt:  [-0.9339% -0.7725% -0.6202%]
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

decode/100              time:   [60.548 ns 60.756 ns 61.028 ns]
                        thrpt:  [1.5261 GiB/s 1.5329 GiB/s 1.5382 GiB/s]
                 change:
                        time:   [+0.1938% +0.6564% +1.1220%] (p = 0.00 < 0.05)
                        thrpt:  [-1.1096% -0.6521% -0.1935%]
                        Change within noise threshold.
Found 17 outliers among 100 measurements (17.00%)
  5 (5.00%) high mild
  12 (12.00%) high severe

decode/500              time:   [204.45 ns 204.70 ns 204.95 ns]
                        thrpt:  [2.2721 GiB/s 2.2749 GiB/s 2.2777 GiB/s]
                 change:
                        time:   [+0.0452% +0.2528% +0.4478%] (p = 0.01 < 0.05)
                        thrpt:  [-0.4458% -0.2522% -0.0451%]
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

decode/3072             time:   [989.27 ns 989.69 ns 990.16 ns]
                        thrpt:  [2.8895 GiB/s 2.8908 GiB/s 2.8921 GiB/s]
                 change:
                        time:   [+0.6206% +0.8873% +1.1451%] (p = 0.00 < 0.05)
                        thrpt:  [-1.1322% -0.8795% -0.6168%]
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe

decode/51200            time:   [16.088 µs 16.106 µs 16.126 µs]
                        thrpt:  [2.9570 GiB/s 2.9607 GiB/s 2.9640 GiB/s]
                 change:
                        time:   [+0.0626% +0.2727% +0.4981%] (p = 0.01 < 0.05)
                        thrpt:  [-0.4957% -0.2719% -0.0625%]
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

decode/102400           time:   [31.898 µs 31.931 µs 31.966 µs]
                        thrpt:  [2.9834 GiB/s 2.9867 GiB/s 2.9898 GiB/s]
                 change:
                        time:   [-0.1058% +0.1411% +0.3896%] (p = 0.26 > 0.05)
                        thrpt:  [-0.3881% -0.1409% +0.1059%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

decode/512000           time:   [152.04 µs 152.88 µs 153.90 µs]
                        thrpt:  [3.0984 GiB/s 3.1191 GiB/s 3.1362 GiB/s]
                 change:
                        time:   [-2.2833% -1.4684% -0.5322%] (p = 0.00 < 0.05)
                        thrpt:  [+0.5350% +1.4902% +2.3366%]
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

decode/1048576          time:   [253.39 µs 255.59 µs 258.55 µs]
                        thrpt:  [3.7770 GiB/s 3.8208 GiB/s 3.8540 GiB/s]
                 change:
+                        time:   [-20.838% -20.235% -19.475%] (p = 0.00 < 0.05)
+                        thrpt:  [+24.186% +25.368% +26.323%]
+                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe

decode/5242880          time:   [1.1652 ms 1.1698 ms 1.1752 ms]
                        thrpt:  [4.1549 GiB/s 4.1741 GiB/s 4.1905 GiB/s]
                 change:
+                        time:   [-33.858% -33.498% -33.127%] (p = 0.00 < 0.05)
+                        thrpt:  [+49.537% +50.372% +51.190%]
+                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

decode/10485760         time:   [2.2953 ms 2.3040 ms 2.3141 ms]
                        thrpt:  [4.2200 GiB/s 4.2385 GiB/s 4.2545 GiB/s]
                 change:
+                        time:   [-36.003% -35.647% -35.306%] (p = 0.00 < 0.05)
+                        thrpt:  [+54.573% +55.393% +56.258%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

decode/20971520         time:   [4.5451 ms 4.5624 ms 4.5819 ms]
                        thrpt:  [4.2627 GiB/s 4.2809 GiB/s 4.2972 GiB/s]
                 change:
+                        time:   [-39.342% -39.082% -38.801%] (p = 0.00 < 0.05)
+                        thrpt:  [+63.403% +64.154% +64.857%]
+                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe



```

