![image](https://github.com/user-attachments/assets/41915033-6d3a-4742-ad10-af78df03fe7b)

```sh
➜  envoy git:(main) wrk -t2 -c500 -d120s http://localhost:8050
Running 2m test @ http://localhost:8050
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    23.13ms   17.48ms 283.88ms   77.47%
    Req/Sec     6.49k     1.86k   18.22k    70.24%
  1546616 requests in 2.00m, 494.54MB read
  Socket errors: connect 203, read 0, write 0, timeout 0
Requests/sec:  12879.39
Transfer/sec:      4.12MB
➜  envoy git:(main) wrk -t2 -c1000 -d120s http://localhost:8050
Running 2m test @ http://localhost:8050
  2 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    21.43ms   16.93ms 220.33ms   78.43%
    Req/Sec     6.94k     1.61k   14.60k    77.25%
  1654648 requests in 2.00m, 529.02MB read
  Socket errors: connect 702, read 0, write 0, timeout 0
Requests/sec:  13779.34
Transfer/sec:      4.41MB
➜  envoy git:(main) wrk -t4 -c1000 -d120s http://localhost:8050
Running 2m test @ http://localhost:8050
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    21.99ms   16.95ms 223.84ms   78.02%
    Req/Sec     3.40k     1.23k   10.59k    70.88%
  1622901 requests in 2.00m, 518.90MB read
  Socket errors: connect 701, read 0, write 0, timeout 0
Requests/sec:  13517.39
Transfer/sec:      4.32MB
➜  envoy git:(main) wrk -t4 -c2000 -d120s http://localhost:8050
Running 2m test @ http://localhost:8050
  4 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    22.60ms   17.19ms 222.68ms   77.68%
    Req/Sec     3.30k     0.89k    8.79k    75.26%
  1573538 requests in 2.00m, 503.15MB read
  Socket errors: connect 1702, read 0, write 0, timeout 0
Requests/sec:  13105.01
Transfer/sec:      4.19MB
➜  envoy git:(main) docker compose restart
[+] Restarting 2/2
 ✔ Container envoy-backend-1  Started                                                                                                                              1.1s 
 ✔ Container envoy-envoy-1    Started                                                                                                                              1.2s 
➜  envoy git:(main) docker compose restart
➜  envoy git:(main) wrk -t4 -c2000 -d120s http://localhost:8050
Running 2m test @ http://localhost:8050
  4 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    21.15ms   16.64ms 232.21ms   78.40%
    Req/Sec     3.51k     1.43k   12.10k    71.36%
  1675456 requests in 2.00m, 535.70MB read
  Socket errors: connect 1703, read 0, write 0, timeout 0
Requests/sec:  13951.65
Transfer/sec:      4.46MB
➜  envoy git:(main) wrk -t4 -c1000 -d120s http://localhost:8050
Running 2m test @ http://localhost:8050
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    24.08ms   18.74ms 270.34ms   78.91%
    Req/Sec     3.10k     1.99k   14.10k    74.58%
  1477257 requests in 2.00m, 472.37MB read
  Socket errors: connect 702, read 0, write 0, timeout 0
Requests/sec:  12301.21
Transfer/sec:      3.93MB
➜  envoy git:(main) ✗ wrk -t4 -c500 -d120s http://localhost:8050
Running 2m test @ http://localhost:8050
  4 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    23.98ms   18.49ms 302.92ms   78.57%
    Req/Sec     3.17k     1.15k    9.38k    70.02%
  1508088 requests in 2.00m, 482.22MB read
  Socket errors: connect 202, read 0, write 0, timeout 0
Requests/sec:  12561.34
Transfer/sec:      4.02MB
```
