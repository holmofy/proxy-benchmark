![image](https://github.com/user-attachments/assets/cd77a459-4e0b-45bb-a818-8c6e7cca9fd2)

```sh
➜  caddy git:(main) wrk -t4 -c1000 -d60s http://localhost:8050
Running 1m test @ http://localhost:8050
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   163.59ms  266.87ms   2.00s    89.39%
    Req/Sec   500.76    656.71     4.29k    86.12%
  115152 requests in 1.00m, 31.62MB read
  Socket errors: connect 702, read 0, write 0, timeout 206
  Non-2xx or 3xx responses: 3592
Requests/sec:   1919.51
Transfer/sec:    539.71KB
➜  caddy git:(main) wrk -t4 -c2000 -d60s http://localhost:8050
Running 1m test @ http://localhost:8050
  4 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   130.04ms  163.64ms   1.98s    87.95%
    Req/Sec   573.45    659.01     6.43k    87.66%
  134141 requests in 1.00m, 37.41MB read
  Socket errors: connect 1702, read 0, write 0, timeout 165
  Non-2xx or 3xx responses: 1082
Requests/sec:   2232.15
Transfer/sec:    637.44KB
➜  caddy git:(main) wrk -t4 -c1000 -d120s http://localhost:8050
Running 2m test @ http://localhost:8050
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   157.81ms  234.20ms   2.00s    90.11%
    Req/Sec   508.04    651.24     6.15k    88.13%
  233971 requests in 2.00m, 65.09MB read
  Socket errors: connect 702, read 0, write 0, timeout 486
  Non-2xx or 3xx responses: 2771
Requests/sec:   1948.20
Transfer/sec:    554.95KB
➜  caddy git:(main) wrk -t4 -c2000 -d120s http://localhost:8050
Running 2m test @ http://localhost:8050
  4 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   139.79ms  181.59ms   2.00s    90.55%
    Req/Sec   543.15    615.71     5.59k    87.82%
  252066 requests in 2.00m, 70.29MB read
  Socket errors: connect 1702, read 0, write 0, timeout 195
  Non-2xx or 3xx responses: 2058
Requests/sec:   2099.30
Transfer/sec:    599.47KB
➜  caddy git:(main) wrk -t4 -c500 -d120s http://localhost:8050
Running 2m test @ http://localhost:8050
  4 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   197.39ms  268.60ms   2.00s    87.30%
    Req/Sec   498.11    599.05     5.10k    86.87%
  229706 requests in 2.00m, 63.96MB read
  Socket errors: connect 202, read 0, write 0, timeout 233
  Non-2xx or 3xx responses: 2391
Requests/sec:   1912.78
Transfer/sec:    545.38KB
➜  caddy git:(main) wrk -t2 -c500 -d120s http://localhost:8050
Running 2m test @ http://localhost:8050
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   183.17ms  243.73ms   2.00s    87.26%
    Req/Sec     1.02k     1.26k    9.93k    88.18%
  238095 requests in 2.00m, 66.17MB read
  Socket errors: connect 202, read 0, write 0, timeout 390
  Non-2xx or 3xx responses: 3175
Requests/sec:   1982.55
Transfer/sec:    564.17KB
```
