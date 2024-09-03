![image](https://github.com/user-attachments/assets/9345d12c-922a-46de-b3bb-c4c7e2986aa4)

```sh
➜  silverwind git:(main) ✗ wrk -t2 -c500 -d120s http://localhost:6667
Running 2m test @ http://localhost:6667
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    12.67ms    7.10ms 256.53ms   79.60%
    Req/Sec    11.76k     4.31k   28.52k    72.19%
  2800594 requests in 2.00m, 213.67MB read
  Socket errors: connect 202, read 0, write 0, timeout 0
Requests/sec:  23337.37
Transfer/sec:      1.78MB
➜  silverwind git:(main) ✗ wrk -t2 -c1000 -d120s http://localhost:6667
Running 2m test @ http://localhost:6667
  2 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    14.87ms    6.77ms 103.59ms   76.95%
    Req/Sec     9.92k     2.41k   24.08k    75.01%
  2364987 requests in 2.00m, 180.43MB read
  Socket errors: connect 702, read 0, write 0, timeout 0
Requests/sec:  19694.33
Transfer/sec:      1.50MB
➜  silverwind git:(main) ✗ wrk -t4 -c1000 -d120s http://localhost:6667
Running 2m test @ http://localhost:6667
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    15.24ms    7.42ms 210.32ms   79.41%
    Req/Sec     4.87k     1.97k   16.22k    64.18%
  2323554 requests in 2.00m, 177.27MB read
  Socket errors: connect 702, read 0, write 0, timeout 0
Requests/sec:  19352.67
Transfer/sec:      1.48MB
➜  silverwind git:(main) ✗ wrk -t4 -c2000 -d120s http://localhost:6667
Running 2m test @ http://localhost:6667
  4 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    14.80ms    6.55ms 233.47ms   76.23%
    Req/Sec     5.01k     1.91k   17.40k    64.57%
  2392388 requests in 2.00m, 182.52MB read
  Socket errors: connect 1702, read 0, write 0, timeout 0
Requests/sec:  19923.96
Transfer/sec:      1.52MB
➜  silverwind git:(main) ✗ wrk -t4 -c3000 -d120s http://localhost:6667
Running 2m test @ http://localhost:6667
  4 threads and 3000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    15.42ms    7.40ms 117.76ms   79.60%
    Req/Sec     4.81k     2.66k   21.50k    57.32%
  2293804 requests in 2.00m, 175.00MB read
  Socket errors: connect 2702, read 0, write 0, timeout 0
Requests/sec:  19098.80
Transfer/sec:      1.46MB
```
