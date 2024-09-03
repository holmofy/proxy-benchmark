![image](https://github.com/user-attachments/assets/221974ae-ca81-44b8-b6cb-edb23800139d)

```sh
➜  nginx git:(main) ✗ wrk -t2 -c500 -d120s http://localhost:80  
Running 2m test @ http://localhost:80
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    94.04ms  169.29ms   1.71s    87.81%
    Req/Sec     3.21k     3.15k   26.62k    86.07%
  765757 requests in 2.00m, 154.50MB read
  Socket errors: connect 211, read 0, write 0, timeout 64
  Non-2xx or 3xx responses: 39478
Requests/sec:   6376.61
Transfer/sec:      1.29MB
➜  nginx git:(main) ✗ wrk -t2 -c1000 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  2 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    69.66ms  157.61ms   2.00s    89.50%
    Req/Sec     3.09k     2.18k   26.83k    74.63%
  737823 requests in 2.00m, 147.77MB read
  Socket errors: connect 711, read 0, write 0, timeout 26
  Non-2xx or 3xx responses: 27358
Requests/sec:   6145.06
Transfer/sec:      1.23MB
➜  nginx git:(main) ✗ wrk -t4 -c1000 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    67.82ms  155.33ms   1.99s    90.30%
    Req/Sec     1.65k     1.31k   15.67k    78.11%
  789082 requests in 2.00m, 158.06MB read
  Socket errors: connect 711, read 0, write 0, timeout 61
  Non-2xx or 3xx responses: 29520
Requests/sec:   6570.55
Transfer/sec:      1.32MB
➜  nginx git:(main) ✗ wrk -t4 -c2000 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  4 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    51.82ms  149.44ms   1.95s    93.56%
    Req/Sec     1.85k     1.05k   12.02k    71.68%
  882066 requests in 2.00m, 175.11MB read
  Socket errors: connect 1710, read 0, write 0, timeout 155
  Non-2xx or 3xx responses: 17742
Requests/sec:   7345.03
Transfer/sec:      1.46MB
```
