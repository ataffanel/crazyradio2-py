import crazyradio2
import time
import threading

n_thread = 2
n_call = 1000

radio = crazyradio2.Crazyradio2()

start = time.time()

threads = []
call_per_thread = int(n_call / n_thread)
total_calls = call_per_thread * n_thread

def thread():
   for i in range(call_per_thread):
      print(".")
      _version =  radio.rpc_call_py("version", None)

for t in range(n_thread):
   threads.append(threading.Thread(target = thread))
   threads[-1].start()

for thread in threads:
   thread.join()

end = time.time()
elapsed = end - start
rate = total_calls / elapsed

print(f"{total_calls} calls in {elapsed} seconds ({rate} calls/second)")