![image](https://github.com/user-attachments/assets/3618ccfa-03c1-42dd-a478-a81baa32f179)


```sh
➜  traefik git:(main) ✗ wrk -t2 -c500 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    48.13ms   56.58ms   1.02s    90.37%
    Req/Sec     3.52k     2.46k   15.14k    72.70%
  836582 requests in 2.00m, 281.63MB read
  Socket errors: connect 202, read 0, write 0, timeout 0
  Non-2xx or 3xx responses: 5
Requests/sec:   6966.56
Transfer/sec:      2.35MB
➜  traefik git:(main) ✗ wrk -t2 -c1000 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  2 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    43.18ms   49.86ms   1.20s    89.71%
    Req/Sec     3.54k     2.07k   12.54k    64.81%
  844137 requests in 2.00m, 284.17MB read
  Socket errors: connect 703, read 0, write 0, timeout 0
  Non-2xx or 3xx responses: 28
Requests/sec:   7029.73
Transfer/sec:      2.37MB
➜  traefik git:(main) ✗ wrk -t4 -c1000 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    41.18ms   42.05ms 601.13ms   84.97%
    Req/Sec     1.84k     1.20k    8.95k    71.78%
  875326 requests in 2.00m, 294.68MB read
  Socket errors: connect 702, read 0, write 0, timeout 0
Requests/sec:   7290.40
Transfer/sec:      2.45MB
➜  traefik git:(main) ✗ wrk -t4 -c2000 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  4 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    41.03ms   43.59ms 916.58ms   85.74%
    Req/Sec     1.83k     1.32k    9.17k    75.29%
  870508 requests in 2.00m, 293.05MB read
  Socket errors: connect 1701, read 0, write 0, timeout 0
  Non-2xx or 3xx responses: 5
Requests/sec:   7250.71
Transfer/sec:      2.44MB
```
