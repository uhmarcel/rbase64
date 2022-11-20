# Profiling Report
```diff

encode/3                time:   [34.154 ns 34.247 ns 34.338 ns]
                        thrpt:  [83.320 MiB/s 83.541 MiB/s 83.767 MiB/s]
                 change:
-                        time:   [+1.1840% +1.4127% +1.6531%] (p = 0.00 < 0.05)
-                        thrpt:  [-1.6262% -1.3930% -1.1702%]
-                        Performance has regressed.

encode/50               time:   [47.027 ns 47.140 ns 47.255 ns]
                        thrpt:  [1009.1 MiB/s 1011.5 MiB/s 1014.0 MiB/s]
                 change:
-                        time:   [+1.2134% +1.4895% +1.7578%] (p = 0.00 < 0.05)
-                        thrpt:  [-1.7275% -1.4676% -1.1988%]
-                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

encode/100              time:   [64.871 ns 65.014 ns 65.158 ns]
                        thrpt:  [1.4293 GiB/s 1.4325 GiB/s 1.4357 GiB/s]
                 change:
                        time:   [-0.1115% +0.0997% +0.3067%] (p = 0.34 > 0.05)
                        thrpt:  [-0.3058% -0.0996% +0.1116%]
                        No change in performance detected.

encode/500              time:   [218.83 ns 219.68 ns 220.54 ns]
                        thrpt:  [2.1114 GiB/s 2.1198 GiB/s 2.1279 GiB/s]
                 change:
                        time:   [-1.1315% -0.7089% -0.2531%] (p = 0.00 < 0.05)
                        thrpt:  [+0.2537% +0.7140% +1.1445%]
                        Change within noise threshold.

encode/3072             time:   [1.0202 µs 1.0225 µs 1.0251 µs]
                        thrpt:  [2.7911 GiB/s 2.7981 GiB/s 2.8044 GiB/s]
                 change:
                        time:   [-0.7654% -0.5503% -0.3401%] (p = 0.00 < 0.05)
                        thrpt:  [+0.3413% +0.5533% +0.7713%]
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) low mild
  3 (3.00%) high mild

encode/1048576          time:   [327.91 µs 330.26 µs 333.51 µs]
                        thrpt:  [2.9281 GiB/s 2.9569 GiB/s 2.9782 GiB/s]
                 change:
+                        time:   [-3.3106% -2.4132% -1.3988%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.4187% +2.4729% +3.4239%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

encode/5242880          time:   [1.8177 ms 1.9400 ms 2.1327 ms]
                        thrpt:  [2.2895 GiB/s 2.5170 GiB/s 2.6862 GiB/s]
                 change:
                        time:   [-8.7803% -2.5049% +7.6388%] (p = 0.66 > 0.05)
                        thrpt:  [-7.0967% +2.5692% +9.6254%]
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  5 (5.00%) high mild
  13 (13.00%) high severe

encode/10485760         time:   [3.3043 ms 3.3192 ms 3.3382 ms]
                        thrpt:  [2.9254 GiB/s 2.9421 GiB/s 2.9555 GiB/s]
                 change:
+                        time:   [-20.610% -20.243% -19.794%] (p = 0.00 < 0.05)
+                        thrpt:  [+24.680% +25.381% +25.960%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe



decode/3                time:   [31.064 ns 31.081 ns 31.101 ns]
                        thrpt:  [91.991 MiB/s 92.050 MiB/s 92.101 MiB/s]
                 change:
-                        time:   [+1.0599% +1.2425% +1.4254%] (p = 0.00 < 0.05)
-                        thrpt:  [-1.4054% -1.2273% -1.0488%]
-                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe

decode/50               time:   [45.814 ns 45.846 ns 45.882 ns]
                        thrpt:  [1.0149 GiB/s 1.0157 GiB/s 1.0164 GiB/s]
                 change:
                        time:   [+0.5674% +0.6626% +0.7555%] (p = 0.00 < 0.05)
                        thrpt:  [-0.7499% -0.6582% -0.5642%]
                        Change within noise threshold.
Found 21 outliers among 100 measurements (21.00%)
  4 (4.00%) high mild
  17 (17.00%) high severe

decode/100              time:   [60.197 ns 60.256 ns 60.342 ns]
                        thrpt:  [1.5434 GiB/s 1.5456 GiB/s 1.5471 GiB/s]
                 change:
                        time:   [-0.3572% -0.0779% +0.2118%] (p = 0.60 > 0.05)
                        thrpt:  [-0.2113% +0.0780% +0.3584%]
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  7 (7.00%) high mild
  7 (7.00%) high severe

decode/500              time:   [208.75 ns 210.18 ns 211.65 ns]
                        thrpt:  [2.2001 GiB/s 2.2155 GiB/s 2.2307 GiB/s]
                 change:
-                        time:   [+1.8345% +2.4966% +3.1240%] (p = 0.00 < 0.05)
-                        thrpt:  [-3.0293% -2.4358% -1.8015%]
-                        Performance has regressed.

decode/3072             time:   [989.32 ns 990.00 ns 990.82 ns]
                        thrpt:  [2.8875 GiB/s 2.8899 GiB/s 2.8919 GiB/s]
                 change:
                        time:   [+0.0191% +0.1318% +0.2523%] (p = 0.03 < 0.05)
                        thrpt:  [-0.2516% -0.1317% -0.0191%]
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

decode/1048576          time:   [249.13 µs 250.40 µs 251.83 µs]
                        thrpt:  [3.8779 GiB/s 3.9000 GiB/s 3.9199 GiB/s]
                 change:
+                        time:   [-22.873% -22.317% -21.647%] (p = 0.00 < 0.05)
+                        thrpt:  [+27.627% +28.728% +29.657%]
+                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

decode/5242880          time:   [1.1650 ms 1.1697 ms 1.1761 ms]
                        thrpt:  [4.1518 GiB/s 4.1743 GiB/s 4.1914 GiB/s]
                 change:
+                        time:   [-34.046% -33.490% -32.830%] (p = 0.00 < 0.05)
+                        thrpt:  [+48.877% +50.354% +51.621%]
+                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

decode/10485760         time:   [2.2965 ms 2.3054 ms 2.3157 ms]
                        thrpt:  [4.2171 GiB/s 4.2359 GiB/s 4.2523 GiB/s]
                 change:
+                        time:   [-36.494% -36.219% -35.929%] (p = 0.00 < 0.05)
+                        thrpt:  [+56.077% +56.785% +57.467%]
+                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) high mild
  10 (10.00%) high severe



```

