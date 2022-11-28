# Profiling Report
```diff

encode/3                time:   [16.456 ns 16.494 ns 16.528 ns]
                        thrpt:  [173.10 MiB/s 173.46 MiB/s 173.86 MiB/s]
                 change:
-                        time:   [+4.4885% +4.7129% +4.9515%] (p = 0.00 < 0.05)
-                        thrpt:  [-4.7179% -4.5008% -4.2957%]
-                        Performance has regressed.

encode/50               time:   [34.108 ns 34.187 ns 34.259 ns]
                        thrpt:  [1.3592 GiB/s 1.3621 GiB/s 1.3652 GiB/s]
                 change:
                        time:   [-1.4238% -1.1800% -0.9359%] (p = 0.00 < 0.05)
                        thrpt:  [+0.9447% +1.1941% +1.4443%]
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

encode/100              time:   [54.533 ns 54.655 ns 54.792 ns]
                        thrpt:  [1.6997 GiB/s 1.7040 GiB/s 1.7078 GiB/s]
                 change:
                        time:   [-1.0111% -0.7870% -0.5747%] (p = 0.00 < 0.05)
                        thrpt:  [+0.5781% +0.7933% +1.0214%]
                        Change within noise threshold.

encode/500              time:   [179.02 ns 179.48 ns 179.95 ns]
                        thrpt:  [2.5878 GiB/s 2.5945 GiB/s 2.6011 GiB/s]
                 change:
                        time:   [-1.2171% -0.9906% -0.7346%] (p = 0.00 < 0.05)
                        thrpt:  [+0.7400% +1.0005% +1.2320%]
                        Change within noise threshold.

encode/3072             time:   [1.0194 µs 1.0216 µs 1.0239 µs]
                        thrpt:  [2.7941 GiB/s 2.8005 GiB/s 2.8065 GiB/s]
                 change:
                        time:   [-0.8454% -0.6297% -0.4378%] (p = 0.00 < 0.05)
                        thrpt:  [+0.4397% +0.6337% +0.8526%]
                        Change within noise threshold.

encode/51200            time:   [16.643 µs 16.678 µs 16.716 µs]
                        thrpt:  [2.8526 GiB/s 2.8591 GiB/s 2.8650 GiB/s]
                 change:
                        time:   [-0.9556% -0.7178% -0.5073%] (p = 0.00 < 0.05)
                        thrpt:  [+0.5098% +0.7230% +0.9648%]
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

encode/102400           time:   [33.531 µs 33.601 µs 33.667 µs]
                        thrpt:  [2.8327 GiB/s 2.8382 GiB/s 2.8441 GiB/s]
                 change:
                        time:   [-1.1709% -0.9768% -0.7508%] (p = 0.00 < 0.05)
                        thrpt:  [+0.7565% +0.9864% +1.1847%]
                        Change within noise threshold.

encode/512000           time:   [77.039 µs 77.515 µs 78.016 µs]
                        thrpt:  [6.1120 GiB/s 6.1515 GiB/s 6.1895 GiB/s]
                 change:
                        time:   [-0.0394% +1.1245% +2.3684%] (p = 0.07 > 0.05)
                        thrpt:  [-2.3136% -1.1120% +0.0395%]
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe

encode/1048576          time:   [121.00 µs 121.71 µs 122.50 µs]
                        thrpt:  [7.9722 GiB/s 8.0236 GiB/s 8.0708 GiB/s]
                 change:
                        time:   [-0.3161% +0.9008% +2.1738%] (p = 0.16 > 0.05)
                        thrpt:  [-2.1275% -0.8927% +0.3171%]
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe

encode/5242880          time:   [483.52 µs 485.32 µs 487.18 µs]
                        thrpt:  [10.023 GiB/s 10.061 GiB/s 10.098 GiB/s]
                 change:
                        time:   [-0.4425% +0.8796% +2.0624%] (p = 0.18 > 0.05)
                        thrpt:  [-2.0207% -0.8719% +0.4445%]
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

encode/10485760         time:   [868.23 µs 872.71 µs 877.73 µs]
                        thrpt:  [11.126 GiB/s 11.190 GiB/s 11.248 GiB/s]
                 change:
                        time:   [-0.2014% +0.6031% +1.3685%] (p = 0.15 > 0.05)
                        thrpt:  [-1.3500% -0.5995% +0.2018%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

encode/20971520         time:   [1.7922 ms 1.8032 ms 1.8160 ms]
                        thrpt:  [10.755 GiB/s 10.831 GiB/s 10.898 GiB/s]
                 change:
                        time:   [-0.3838% +0.3308% +1.0717%] (p = 0.39 > 0.05)
                        thrpt:  [-1.0604% -0.3297% +0.3853%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe


decode/3                time:   [12.138 ns 12.144 ns 12.152 ns]
                        thrpt:  [235.45 MiB/s 235.59 MiB/s 235.71 MiB/s]
                 change:
                        time:   [-0.1182% -0.0176% +0.0875%] (p = 0.75 > 0.05)
                        thrpt:  [-0.0874% +0.0176% +0.1184%]
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe

decode/50               time:   [24.163 ns 24.179 ns 24.197 ns]
                        thrpt:  [1.9244 GiB/s 1.9259 GiB/s 1.9272 GiB/s]
                 change:
+                        time:   [-7.7921% -7.6623% -7.5489%] (p = 0.00 < 0.05)
+                        thrpt:  [+8.1653% +8.2981% +8.4506%]
+                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

decode/100              time:   [36.024 ns 36.043 ns 36.063 ns]
                        thrpt:  [2.5825 GiB/s 2.5839 GiB/s 2.5853 GiB/s]
                 change:
+                        time:   [-10.402% -10.303% -10.205%] (p = 0.00 < 0.05)
+                        thrpt:  [+11.365% +11.486% +11.610%]
+                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

decode/500              time:   [140.25 ns 140.33 ns 140.43 ns]
                        thrpt:  [3.3160 GiB/s 3.3184 GiB/s 3.3203 GiB/s]
                 change:
+                        time:   [-12.264% -12.163% -12.056%] (p = 0.00 < 0.05)
+                        thrpt:  [+13.709% +13.847% +13.979%]
+                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

decode/3072             time:   [819.20 ns 821.59 ns 824.53 ns]
                        thrpt:  [3.4699 GiB/s 3.4823 GiB/s 3.4924 GiB/s]
                 change:
+                        time:   [-12.566% -12.417% -12.246%] (p = 0.00 < 0.05)
+                        thrpt:  [+13.955% +14.177% +14.373%]
+                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

decode/51200            time:   [12.768 µs 12.780 µs 12.793 µs]
                        thrpt:  [3.7274 GiB/s 3.7312 GiB/s 3.7348 GiB/s]
                 change:
+                        time:   [-15.360% -15.167% -14.906%] (p = 0.00 < 0.05)
+                        thrpt:  [+17.518% +17.879% +18.148%]
+                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

decode/102400           time:   [25.915 µs 25.960 µs 26.003 µs]
                        thrpt:  [3.6675 GiB/s 3.6737 GiB/s 3.6801 GiB/s]
                 change:
+                        time:   [-14.805% -14.632% -14.474%] (p = 0.00 < 0.05)
+                        thrpt:  [+16.923% +17.140% +17.378%]
+                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild

decode/512000           time:   [68.084 µs 68.577 µs 69.154 µs]
                        thrpt:  [6.8953 GiB/s 6.9533 GiB/s 7.0037 GiB/s]
                 change:
+                        time:   [-6.0504% -4.7809% -3.5395%] (p = 0.00 < 0.05)
+                        thrpt:  [+3.6694% +5.0209% +6.4400%]
+                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

decode/1048576          time:   [103.26 µs 103.72 µs 104.24 µs]
                        thrpt:  [9.3686 GiB/s 9.4154 GiB/s 9.4573 GiB/s]
                 change:
+                        time:   [-8.2282% -7.3591% -6.5279%] (p = 0.00 < 0.05)
+                        thrpt:  [+6.9838% +7.9437% +8.9660%]
+                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe

decode/5242880          time:   [400.00 µs 402.02 µs 404.23 µs]
                        thrpt:  [12.079 GiB/s 12.146 GiB/s 12.207 GiB/s]
                 change:
+                        time:   [-10.080% -9.1619% -8.2020%] (p = 0.00 < 0.05)
+                        thrpt:  [+8.9348% +10.086% +11.210%]
+                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

decode/10485760         time:   [720.71 µs 724.50 µs 728.69 µs]
                        thrpt:  [13.402 GiB/s 13.479 GiB/s 13.550 GiB/s]
                 change:
+                        time:   [-13.295% -12.204% -11.000%] (p = 0.00 < 0.05)
+                        thrpt:  [+12.359% +13.901% +15.334%]
+                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

decode/20971520         time:   [1.3687 ms 1.3771 ms 1.3864 ms]
                        thrpt:  [14.087 GiB/s 14.183 GiB/s 14.270 GiB/s]
                 change:
+                        time:   [-12.109% -11.395% -10.617%] (p = 0.00 < 0.05)
+                        thrpt:  [+11.879% +12.861% +13.777%]
+                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe


```

