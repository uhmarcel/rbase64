# Profiling Report
```diff

encode/3                time:   [33.515 ns 33.597 ns 33.674 ns]
                        thrpt:  [84.964 MiB/s 85.158 MiB/s 85.366 MiB/s]
                 change:
-                        time:   [+5.9382% +6.2328% +6.5144%] (p = 0.00 < 0.05)
-                        thrpt:  [-6.1160% -5.8671% -5.6053%]
-                        Performance has regressed.

encode/50               time:   [58.211 ns 58.342 ns 58.488 ns]
                        thrpt:  [815.27 MiB/s 817.32 MiB/s 819.16 MiB/s]
                 change:
-                        time:   [+12.569% +12.876% +13.211%] (p = 0.00 < 0.05)
-                        thrpt:  [-11.670% -11.407% -11.165%]
-                        Performance has regressed.

encode/100              time:   [75.344 ns 75.517 ns 75.676 ns]
                        thrpt:  [1.2307 GiB/s 1.2333 GiB/s 1.2361 GiB/s]
                 change:
-                        time:   [+4.2552% +4.5360% +4.8100%] (p = 0.00 < 0.05)
-                        thrpt:  [-4.5893% -4.3392% -4.0815%]
-                        Performance has regressed.

encode/500              time:   [219.94 ns 220.47 ns 220.97 ns]
                        thrpt:  [2.1073 GiB/s 2.1122 GiB/s 2.1173 GiB/s]
                 change:
+                        time:   [-17.618% -17.338% -17.065%] (p = 0.00 < 0.05)
+                        thrpt:  [+20.576% +20.975% +21.385%]
+                        Performance has improved.

encode/3072             time:   [1.0667 µs 1.0689 µs 1.0712 µs]
                        thrpt:  [2.6708 GiB/s 2.6766 GiB/s 2.6821 GiB/s]
                 change:
+                        time:   [-22.109% -21.906% -21.698%] (p = 0.00 < 0.05)
+                        thrpt:  [+27.711% +28.050% +28.384%]
+                        Performance has improved.

encode/1048576          time:   [341.54 µs 342.30 µs 343.13 µs]
                        thrpt:  [2.8460 GiB/s 2.8530 GiB/s 2.8593 GiB/s]
                 change:
+                        time:   [-21.790% -21.555% -21.273%] (p = 0.00 < 0.05)
+                        thrpt:  [+27.021% +27.478% +27.860%]
+                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

encode/5242880          time:   [1.9835 ms 1.9890 ms 1.9958 ms]
                        thrpt:  [2.4465 GiB/s 2.4549 GiB/s 2.4618 GiB/s]
                 change:
+                        time:   [-26.556% -22.305% -19.503%] (p = 0.00 < 0.05)
+                        thrpt:  [+24.229% +28.708% +36.157%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) high mild
  6 (6.00%) high severe

encode/10485760         time:   [4.0818 ms 4.0885 ms 4.0958 ms]
                        thrpt:  [2.3843 GiB/s 2.3886 GiB/s 2.3925 GiB/s]
                 change:
+                        time:   [-20.719% -20.490% -20.254%] (p = 0.00 < 0.05)
+                        thrpt:  [+25.398% +25.771% +26.133%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe



decode/3                time:   [30.792 ns 30.850 ns 30.906 ns]
                        thrpt:  [92.572 MiB/s 92.739 MiB/s 92.916 MiB/s]
                 change:
                        time:   [-1.0133% -0.7076% -0.4067%] (p = 0.00 < 0.05)
                        thrpt:  [+0.4083% +0.7126% +1.0237%]
                        Change within noise threshold.
Found 27 outliers among 100 measurements (27.00%)
  14 (14.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  7 (7.00%) high severe

decode/50               time:   [47.574 ns 47.675 ns 47.778 ns]
                        thrpt:  [998.03 MiB/s 1000.2 MiB/s 1002.3 MiB/s]
                 change:
+                        time:   [-17.468% -17.272% -17.071%] (p = 0.00 < 0.05)
+                        thrpt:  [+20.585% +20.878% +21.165%]
+                        Performance has improved.

decode/100              time:   [64.909 ns 65.197 ns 65.502 ns]
                        thrpt:  [1.4218 GiB/s 1.4285 GiB/s 1.4348 GiB/s]
                 change:
+                        time:   [-23.427% -23.134% -22.836%] (p = 0.00 < 0.05)
+                        thrpt:  [+29.594% +30.097% +30.594%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  5 (5.00%) high severe

decode/500              time:   [226.44 ns 226.86 ns 227.23 ns]
                        thrpt:  [2.0493 GiB/s 2.0526 GiB/s 2.0564 GiB/s]
                 change:
+                        time:   [-31.013% -30.694% -30.388%] (p = 0.00 < 0.05)
+                        thrpt:  [+43.654% +44.287% +44.955%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild

decode/3072             time:   [1.1513 µs 1.1545 µs 1.1574 µs]
                        thrpt:  [2.4718 GiB/s 2.4781 GiB/s 2.4851 GiB/s]
                 change:
+                        time:   [-33.585% -33.398% -33.192%] (p = 0.00 < 0.05)
+                        thrpt:  [+49.684% +50.146% +50.569%]
+                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

decode/1048576          time:   [368.97 µs 369.97 µs 370.99 µs]
                        thrpt:  [2.6323 GiB/s 2.6396 GiB/s 2.6467 GiB/s]
                 change:
+                        time:   [-34.908% -34.664% -34.408%] (p = 0.00 < 0.05)
+                        thrpt:  [+52.457% +53.056% +53.628%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

decode/5242880          time:   [2.0457 ms 2.0554 ms 2.0654 ms]
                        thrpt:  [2.3641 GiB/s 2.3756 GiB/s 2.3868 GiB/s]
                 change:
+                        time:   [-32.274% -31.658% -31.155%] (p = 0.00 < 0.05)
+                        thrpt:  [+45.253% +46.322% +47.654%]
+                        Performance has improved.

decode/10485760         time:   [4.0950 ms 4.1065 ms 4.1190 ms]
                        thrpt:  [2.3708 GiB/s 2.3781 GiB/s 2.3848 GiB/s]
                 change:
+                        time:   [-35.882% -35.125% -34.440%] (p = 0.00 < 0.05)
+                        thrpt:  [+52.531% +54.142% +55.963%]
+                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe



```

