# Profiling Report
```diff

encode/3                time:   [33.369 ns 33.429 ns 33.502 ns]
                        thrpt:  [85.400 MiB/s 85.585 MiB/s 85.740 MiB/s]
                 change:
-                        time:   [+6.4304% +6.7461% +7.0597%] (p = 0.00 < 0.05)
-                        thrpt:  [-6.5942% -6.3197% -6.0419%]
-                        Performance has regressed.

encode/50               time:   [58.911 ns 59.053 ns 59.191 ns]
                        thrpt:  [805.59 MiB/s 807.48 MiB/s 809.42 MiB/s]
                 change:
-                        time:   [+13.453% +13.779% +14.109%] (p = 0.00 < 0.05)
-                        thrpt:  [-12.364% -12.110% -11.858%]
-                        Performance has regressed.

encode/100              time:   [76.154 ns 76.364 ns 76.586 ns]
                        thrpt:  [1.2161 GiB/s 1.2196 GiB/s 1.2229 GiB/s]
                 change:
-                        time:   [+6.1302% +6.4347% +6.7408%] (p = 0.00 < 0.05)
-                        thrpt:  [-6.3151% -6.0457% -5.7761%]
-                        Performance has regressed.

encode/500              time:   [222.41 ns 223.09 ns 223.87 ns]
                        thrpt:  [2.0800 GiB/s 2.0874 GiB/s 2.0937 GiB/s]
                 change:
+                        time:   [-16.061% -15.720% -15.409%] (p = 0.00 < 0.05)
+                        thrpt:  [+18.215% +18.653% +19.134%]
+                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

encode/3072             time:   [1.0847 µs 1.0874 µs 1.0901 µs]
                        thrpt:  [2.6247 GiB/s 2.6310 GiB/s 2.6375 GiB/s]
                 change:
+                        time:   [-20.814% -20.593% -20.373%] (p = 0.00 < 0.05)
+                        thrpt:  [+25.586% +25.933% +26.285%]
+                        Performance has improved.

encode/1048576          time:   [347.11 µs 347.97 µs 348.80 µs]
                        thrpt:  [2.7998 GiB/s 2.8064 GiB/s 2.8134 GiB/s]
                 change:
+                        time:   [-20.384% -20.131% -19.865%] (p = 0.00 < 0.05)
+                        thrpt:  [+24.790% +25.206% +25.603%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

encode/5242880          time:   [2.0132 ms 2.0208 ms 2.0306 ms]
                        thrpt:  [2.4046 GiB/s 2.4163 GiB/s 2.4254 GiB/s]
                 change:
+                        time:   [-25.430% -21.063% -18.227%] (p = 0.00 < 0.05)
+                        thrpt:  [+22.289% +26.684% +34.102%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

encode/10485760         time:   [4.1761 ms 4.2125 ms 4.2696 ms]
                        thrpt:  [2.2872 GiB/s 2.3182 GiB/s 2.3385 GiB/s]
                 change:
+                        time:   [-18.823% -18.078% -17.027%] (p = 0.00 < 0.05)
+                        thrpt:  [+20.521% +22.067% +23.188%]
+                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe



decode/3                time:   [30.940 ns 30.979 ns 31.030 ns]
                        thrpt:  [92.201 MiB/s 92.353 MiB/s 92.470 MiB/s]
                 change:
                        time:   [-0.5167% -0.1974% +0.1248%] (p = 0.24 > 0.05)
                        thrpt:  [-0.1247% +0.1978% +0.5194%]
                        No change in performance detected.
Found 22 outliers among 100 measurements (22.00%)
  2 (2.00%) low mild
  5 (5.00%) high mild
  15 (15.00%) high severe

decode/50               time:   [47.984 ns 48.018 ns 48.056 ns]
                        thrpt:  [992.26 MiB/s 993.03 MiB/s 993.75 MiB/s]
                 change:
+                        time:   [-16.919% -16.735% -16.546%] (p = 0.00 < 0.05)
+                        thrpt:  [+19.827% +20.098% +20.364%]
+                        Performance has improved.
Found 20 outliers among 100 measurements (20.00%)
  3 (3.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  11 (11.00%) high severe

decode/100              time:   [64.839 ns 64.920 ns 65.013 ns]
                        thrpt:  [1.4325 GiB/s 1.4346 GiB/s 1.4364 GiB/s]
                 change:
+                        time:   [-23.262% -23.007% -22.762%] (p = 0.00 < 0.05)
+                        thrpt:  [+29.470% +29.882% +30.314%]
+                        Performance has improved.
Found 23 outliers among 100 measurements (23.00%)
  4 (4.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  14 (14.00%) high severe

decode/500              time:   [229.58 ns 229.83 ns 230.07 ns]
                        thrpt:  [2.0240 GiB/s 2.0261 GiB/s 2.0283 GiB/s]
                 change:
+                        time:   [-30.145% -29.823% -29.511%] (p = 0.00 < 0.05)
+                        thrpt:  [+41.865% +42.497% +43.153%]
+                        Performance has improved.
Found 21 outliers among 100 measurements (21.00%)
  6 (6.00%) low severe
  1 (1.00%) low mild
  7 (7.00%) high mild
  7 (7.00%) high severe

decode/3072             time:   [1.1609 µs 1.1615 µs 1.1622 µs]
                        thrpt:  [2.4617 GiB/s 2.4631 GiB/s 2.4644 GiB/s]
                 change:
+                        time:   [-32.807% -32.665% -32.526%] (p = 0.00 < 0.05)
+                        thrpt:  [+48.205% +48.512% +48.824%]
+                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) high mild
  10 (10.00%) high severe

decode/1048576          time:   [371.18 µs 371.55 µs 372.00 µs]
                        thrpt:  [2.6252 GiB/s 2.6283 GiB/s 2.6309 GiB/s]
                 change:
+                        time:   [-34.763% -34.571% -34.383%] (p = 0.00 < 0.05)
+                        thrpt:  [+52.401% +52.838% +53.288%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) high mild
  6 (6.00%) high severe

decode/5242880          time:   [2.0355 ms 2.0423 ms 2.0506 ms]
                        thrpt:  [2.3812 GiB/s 2.3908 GiB/s 2.3988 GiB/s]
                 change:
+                        time:   [-32.649% -32.093% -31.655%] (p = 0.00 < 0.05)
+                        thrpt:  [+46.317% +47.261% +48.475%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

decode/10485760         time:   [4.1104 ms 4.1186 ms 4.1290 ms]
                        thrpt:  [2.3651 GiB/s 2.3711 GiB/s 2.3758 GiB/s]
                 change:
+                        time:   [-35.686% -34.934% -34.267%] (p = 0.00 < 0.05)
+                        thrpt:  [+52.131% +53.690% +55.487%]
+                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe



```

