![image](https://github.com/user-attachments/assets/0df192f8-c480-4eb3-bf97-6420cfed3654)

```sh
➜  haproxy git:(main) ✗ wrk -t2 -c500 -d120s http://localhost:80  
Running 2m test @ http://localhost:80
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    16.22ms    9.74ms 254.67ms   80.18%
    Req/Sec     9.24k     2.42k   26.25k    73.03%
  2202105 requests in 2.00m, 354.92MB read
  Socket errors: connect 201, read 0, write 0, timeout 0
Requests/sec:  18336.53
Transfer/sec:      2.96MB
➜  haproxy git:(main) ✗ wrk -t2 -c1000 -d120s http://localhost:80  
Running 2m test @ http://localhost:80
  2 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    17.31ms   10.67ms 235.74ms   79.47%
    Req/Sec     8.55k     2.83k   26.42k    76.91%
  2037805 requests in 2.00m, 328.43MB read
  Socket errors: connect 701, read 0, write 0, timeout 0
Requests/sec:  16975.61
Transfer/sec:      2.74MB
➜  haproxy git:(main) ✗ wrk -t4 -c1000 -d120s http://localhost:80  
Running 2m test @ http://localhost:80
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    16.60ms    9.99ms 204.52ms   79.90%
    Req/Sec     4.49k     2.11k   16.80k    66.63%
  2140810 requests in 2.00m, 345.04MB read
  Socket errors: connect 701, read 0, write 0, timeout 0
Requests/sec:  17825.83
Transfer/sec:      2.87MB
➜  haproxy git:(main) ✗ wrk -t4 -c2000 -d120s http://localhost:80  
Running 2m test @ http://localhost:80
  4 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    17.65ms   10.47ms 254.60ms   78.06%
    Req/Sec     4.22k     1.60k   15.21k    67.76%
  2012387 requests in 2.00m, 324.34MB read
  Socket errors: connect 1701, read 0, write 0, timeout 21
Requests/sec:  16756.99
Transfer/sec:      2.70MB
```
