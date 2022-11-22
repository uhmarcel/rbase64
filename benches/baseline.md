# Profiling Report
```diff
encode/3                time:   [33.121 ns 33.157 ns 33.196 ns]
                        thrpt:  [86.186 MiB/s 86.287 MiB/s 86.380 MiB/s]
                 change:
                        time:   [-0.3017% -0.1409% +0.0153%] (p = 0.09 > 0.05)
                        thrpt:  [-0.0153% +0.1411% +0.3027%]
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

encode/50               time:   [47.790 ns 47.835 ns 47.883 ns]
                        thrpt:  [995.84 MiB/s 996.85 MiB/s 997.78 MiB/s]
                 change:
                        time:   [+0.1093% +0.3075% +0.4975%] (p = 0.00 < 0.05)
                        thrpt:  [-0.4950% -0.3065% -0.1091%]
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

encode/100              time:   [65.539 ns 65.570 ns 65.606 ns]
                        thrpt:  [1.4196 GiB/s 1.4204 GiB/s 1.4210 GiB/s]
                 change:
                        time:   [-0.3224% -0.1855% -0.0513%] (p = 0.01 < 0.05)
                        thrpt:  [+0.0514% +0.1858% +0.3234%]
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

encode/500              time:   [221.86 ns 222.34 ns 222.94 ns]
                        thrpt:  [2.0887 GiB/s 2.0943 GiB/s 2.0989 GiB/s]

                 change:
+                        time:   [-2.6294% -2.1876% -1.6980%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.7273% +2.2366% +2.7004%]
+                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

encode/3072             time:   [1.0993 µs 1.0999 µs 1.1006 µs]
                        thrpt:  [2.5995 GiB/s 2.6011 GiB/s 2.6026 GiB/s]
                 change:
                        time:   [-0.2303% -0.1011% +0.0200%] (p = 0.11 > 0.05)
                        thrpt:  [-0.0200% +0.1012% +0.2308%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) high mild
  3 (3.00%) high severe

encode/51200            time:   [17.977 µs 17.999 µs 18.022 µs]
                        thrpt:  [2.6458 GiB/s 2.6492 GiB/s 2.6525 GiB/s]
                 change:
                        time:   [-0.0155% +0.1116% +0.2430%] (p = 0.10 > 0.05)
                        thrpt:  [-0.2424% -0.1115% +0.0155%]
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

encode/102400           time:   [36.021 µs 36.044 µs 36.067 µs]
                        thrpt:  [2.6442 GiB/s 2.6459 GiB/s 2.6476 GiB/s]
                 change:
                        time:   [+0.0666% +0.1675% +0.2703%] (p = 0.00 < 0.05)
                        thrpt:  [-0.2696% -0.1672% -0.0665%]
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe

encode/512000           time:   [86.979 µs 87.512 µs 88.098 µs]
                        thrpt:  [5.4126 GiB/s 5.4488 GiB/s 5.4822 GiB/s]
                 change:
+                        time:   [-53.417% -52.844% -52.261%] (p = 0.00 < 0.05)
+                        thrpt:  [+109.47% +112.06% +114.67%]
+                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe

encode/1048576          time:   [129.89 µs 131.14 µs 132.63 µs]
                        thrpt:  [7.3633 GiB/s 7.4466 GiB/s 7.5187 GiB/s]
                 change:
+                        time:   [-59.433% -58.786% -58.148%] (p = 0.00 < 0.05)
+                        thrpt:  [+138.94% +142.63% +146.50%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

encode/5242880          time:   [833.25 µs 837.25 µs 841.87 µs]
                        thrpt:  [5.7999 GiB/s 5.8320 GiB/s 5.8599 GiB/s]
                 change:
+                        time:   [-49.378% -48.994% -48.550%] (p = 0.00 < 0.05)
+                        thrpt:  [+94.362% +96.056% +97.544%]
+                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

encode/10485760         time:   [1.5284 ms 1.5353 ms 1.5434 ms]
                        thrpt:  [6.3272 GiB/s 6.3607 GiB/s 6.3896 GiB/s]
                 change:
+                        time:   [-51.544% -51.255% -50.968%] (p = 0.00 < 0.05)
+                        thrpt:  [+103.95% +105.15% +106.37%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) high mild
  9 (9.00%) high severe

encode/20971520         time:   [2.9371 ms 2.9426 ms 2.9487 ms]
                        thrpt:  [6.6237 GiB/s 6.6373 GiB/s 6.6498 GiB/s]
                 change:
+                        time:   [-52.437% -52.266% -52.098%] (p = 0.00 < 0.05)
+                        thrpt:  [+108.76% +109.50% +110.25%]
+                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe



decode/3                time:   [31.047 ns 31.064 ns 31.084 ns]
                        thrpt:  [92.043 MiB/s 92.100 MiB/s 92.150 MiB/s]
                 change:
                        time:   [+0.0958% +0.2309% +0.3567%] (p = 0.00 < 0.05)
                        thrpt:  [-0.3555% -0.2304% -0.0957%]
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe

decode/50               time:   [45.584 ns 45.652 ns 45.756 ns]
                        thrpt:  [1.0177 GiB/s 1.0200 GiB/s 1.0216 GiB/s]
                 change:
                        time:   [-0.4541% +0.2538% +1.5121%] (p = 0.78 > 0.05)
                        thrpt:  [-1.4896% -0.2531% +0.4562%]
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

decode/100              time:   [59.453 ns 59.577 ns 59.759 ns]
                        thrpt:  [1.5585 GiB/s 1.5632 GiB/s 1.5665 GiB/s]
                 change:
+                        time:   [-2.0493% -1.7408% -1.4391%] (p = 0.00 < 0.05)
+                        thrpt:  [+1.4601% +1.7716% +2.0922%]
+                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  8 (8.00%) high severe

decode/500              time:   [206.29 ns 206.49 ns 206.69 ns]
                        thrpt:  [2.2530 GiB/s 2.2552 GiB/s 2.2574 GiB/s]
                 change:
                        time:   [+0.7530% +0.9254% +1.0894%] (p = 0.00 < 0.05)
                        thrpt:  [-1.0777% -0.9169% -0.7473%]
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low severe
  5 (5.00%) high mild
  7 (7.00%) high severe

decode/3072             time:   [948.23 ns 948.76 ns 949.36 ns]
                        thrpt:  [3.0136 GiB/s 3.0155 GiB/s 3.0172 GiB/s]
                 change:
+                        time:   [-4.3035% -4.2059% -4.1059%] (p = 0.00 < 0.05)
+                        thrpt:  [+4.2817% +4.3906% +4.4970%]
+                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

decode/51200            time:   [15.568 µs 15.603 µs 15.641 µs]
                        thrpt:  [3.0486 GiB/s 3.0560 GiB/s 3.0629 GiB/s]
                 change:
+                        time:   [-3.5945% -3.3231% -3.0502%] (p = 0.00 < 0.05)
+                        thrpt:  [+3.1461% +3.4373% +3.7286%]
+                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

decode/102400           time:   [31.029 µs 31.131 µs 31.229 µs]
                        thrpt:  [3.0538 GiB/s 3.0635 GiB/s 3.0735 GiB/s]
                 change:
+                        time:   [-3.3724% -3.1084% -2.8340%] (p = 0.00 < 0.05)
+                        thrpt:  [+2.9167% +3.2082% +3.4901%]
+                        Performance has improved.

decode/512000           time:   [71.603 µs 72.167 µs 72.819 µs]
                        thrpt:  [6.5482 GiB/s 6.6074 GiB/s 6.6595 GiB/s]
                 change:
+                        time:   [-55.776% -55.090% -54.450%] (p = 0.00 < 0.05)
+                        thrpt:  [+119.54% +122.67% +126.12%]
+                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  6 (6.00%) high severe

decode/1048576          time:   [111.90 µs 112.31 µs 112.77 µs]
                        thrpt:  [8.6600 GiB/s 8.6949 GiB/s 8.7273 GiB/s]
                 change:
+                        time:   [-60.393% -59.606% -58.741%] (p = 0.00 < 0.05)
+                        thrpt:  [+142.37% +147.56% +152.48%]
+                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  5 (5.00%) high severe

decode/5242880          time:   [596.82 µs 598.86 µs 601.36 µs]
                        thrpt:  [8.1197 GiB/s 8.1534 GiB/s 8.1814 GiB/s]
                 change:
+                        time:   [-50.188% -49.478% -48.586%] (p = 0.00 < 0.05)
+                        thrpt:  [+94.500% +97.933% +100.76%]
+                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  9 (9.00%) high mild
  9 (9.00%) high severe

decode/10485760         time:   [1.1953 ms 1.2062 ms 1.2201 ms]
                        thrpt:  [8.0041 GiB/s 8.0960 GiB/s 8.1701 GiB/s]
                 change:
+                        time:   [-49.341% -48.852% -48.060%] (p = 0.00 < 0.05)
+                        thrpt:  [+92.528% +95.512% +97.397%]
+                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe

decode/20971520         time:   [2.2433 ms 2.2533 ms 2.2644 ms]
                        thrpt:  [8.6255 GiB/s 8.6680 GiB/s 8.7066 GiB/s]
                 change:
+                        time:   [-52.758% -52.317% -51.922%] (p = 0.00 < 0.05)
+                        thrpt:  [+108.00% +109.72% +111.67%]
+                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe
```

