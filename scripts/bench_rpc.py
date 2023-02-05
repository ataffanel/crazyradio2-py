import crazyradio2
import time

radio = crazyradio2.Crazyradio2()

start = time.time()

for i in range(10000):
   version =  radio.rpc_call_py("version", None)

end = time.time()
elapsed = end - start
rate = 10000 / elapsed

print(f"10000 calls in {elapsed} seconds ({rate} calls/second)")