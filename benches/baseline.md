# Profiling Report
```diff

encode/3                time:   [33.538 ns 33.626 ns 33.727 ns]
                        thrpt:  [84.830 MiB/s 85.084 MiB/s 85.308 MiB/s]
                 change:
                        time:   [-0.8900% -0.6116% -0.2926%] (p = 0.00 < 0.05)
                        thrpt:  [+0.2935% +0.6154% +0.8980%]
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

encode/50               time:   [58.352 ns 58.490 ns 58.637 ns]
                        thrpt:  [813.20 MiB/s 815.24 MiB/s 817.17 MiB/s]
                 change:
                        time:   [-1.1272% -0.8774% -0.6427%] (p = 0.00 < 0.05)
                        thrpt:  [+0.6468% +0.8851% +1.1400%]
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

encode/100              time:   [74.960 ns 75.106 ns 75.275 ns]
                        thrpt:  [1.2372 GiB/s 1.2400 GiB/s 1.2424 GiB/s]
                 change:
                        time:   [-0.9910% -0.7815% -0.5546%] (p = 0.00 < 0.05)
                        thrpt:  [+0.5577% +0.7877% +1.0009%]
                        Change within noise threshold.

encode/500              time:   [219.73 ns 220.25 ns 220.75 ns]
                        thrpt:  [2.1094 GiB/s 2.1142 GiB/s 2.1192 GiB/s]
                 change:
+                        time:   [-2.2033% -1.9038% -1.6263%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.6531% +1.9407% +2.2529%]
+                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

encode/3072             time:   [1.0716 µs 1.0761 µs 1.0824 µs]
                        thrpt:  [2.6432 GiB/s 2.6586 GiB/s 2.6700 GiB/s]
                 change:
                        time:   [-1.0267% -0.7870% -0.4710%] (p = 0.00 < 0.05)
                        thrpt:  [+0.4733% +0.7932% +1.0373%]
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

encode/1048576          time:   [343.10 µs 344.10 µs 345.03 µs]
                        thrpt:  [2.8304 GiB/s 2.8380 GiB/s 2.8463 GiB/s]
                 change:
                        time:   [-1.0872% -0.7844% -0.4280%] (p = 0.00 < 0.05)
                        thrpt:  [+0.4299% +0.7906% +1.0991%]
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

encode/5242880          time:   [1.9936 ms 1.9986 ms 2.0038 ms]
                        thrpt:  [2.4368 GiB/s 2.4431 GiB/s 2.4492 GiB/s]
                 change:
                        time:   [-1.4849% -1.1372% -0.8000%] (p = 0.00 < 0.05)
                        thrpt:  [+0.8065% +1.1502% +1.5073%]
                        Change within noise threshold.

encode/10485760         time:   [4.0803 ms 4.0869 ms 4.0941 ms]
                        thrpt:  [2.3853 GiB/s 2.3895 GiB/s 2.3934 GiB/s]
                 change:
+                        time:   [-1.5443% -1.2906% -1.0378%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.0487% +1.3075% +1.5685%]
+                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild



decode/3                time:   [30.665 ns 30.689 ns 30.712 ns]
                        thrpt:  [93.158 MiB/s 93.228 MiB/s 93.299 MiB/s]
                 change:
                        time:   [-1.0012% -0.8312% -0.6729%] (p = 0.00 < 0.05)
                        thrpt:  [+0.6775% +0.8382% +1.0113%]
                        Change within noise threshold.
Found 21 outliers among 100 measurements (21.00%)
  7 (7.00%) low severe
  3 (3.00%) high mild
  11 (11.00%) high severe

decode/50               time:   [45.478 ns 45.507 ns 45.539 ns]
                        thrpt:  [1.0226 GiB/s 1.0233 GiB/s 1.0239 GiB/s]
                 change:
+                        time:   [-5.4341% -5.3153% -5.1984%] (p = 0.00 < 0.05)
+                        thrpt:  [+5.4834% +5.6136% +5.7463%]
+                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  3 (3.00%) low severe
  2 (2.00%) low mild
  13 (13.00%) high severe

decode/100              time:   [60.366 ns 60.488 ns 60.623 ns]
                        thrpt:  [1.5363 GiB/s 1.5397 GiB/s 1.5428 GiB/s]
                 change:
+                        time:   [-6.5171% -6.3238% -6.1317%] (p = 0.00 < 0.05)
+                        thrpt:  [+6.5323% +6.7507% +6.9714%]
+                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

decode/500              time:   [204.10 ns 204.35 ns 204.64 ns]
                        thrpt:  [2.2756 GiB/s 2.2787 GiB/s 2.2816 GiB/s]
                 change:
+                        time:   [-10.938% -10.654% -10.369%] (p = 0.00 < 0.05)
+                        thrpt:  [+11.568% +11.925% +12.281%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe

decode/3072             time:   [992.64 ns 994.64 ns 996.86 ns]
                        thrpt:  [2.8700 GiB/s 2.8764 GiB/s 2.8822 GiB/s]
                 change:
+                        time:   [-13.163% -12.973% -12.772%] (p = 0.00 < 0.05)
+                        thrpt:  [+14.642% +14.906% +15.158%]
+                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

decode/1048576          time:   [321.77 µs 322.06 µs 322.40 µs]
                        thrpt:  [3.0291 GiB/s 3.0322 GiB/s 3.0350 GiB/s]
                 change:
+                        time:   [-12.295% -12.111% -11.926%] (p = 0.00 < 0.05)
+                        thrpt:  [+13.540% +13.780% +14.019%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  3 (3.00%) high severe

decode/5242880          time:   [1.7669 ms 1.7690 ms 1.7712 ms]
                        thrpt:  [2.7567 GiB/s 2.7602 GiB/s 2.7634 GiB/s]
                 change:
+                        time:   [-11.926% -11.800% -11.674%] (p = 0.00 < 0.05)
+                        thrpt:  [+13.216% +13.378% +13.541%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  10 (10.00%) high mild
  3 (3.00%) high severe

decode/10485760         time:   [3.5955 ms 3.6021 ms 3.6094 ms]
                        thrpt:  [2.7056 GiB/s 2.7111 GiB/s 2.7161 GiB/s]
                 change:
+                        time:   [-11.655% -11.439% -11.217%] (p = 0.00 < 0.05)
+                        thrpt:  [+12.634% +12.917% +13.193%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe



```

