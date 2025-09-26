# realistic_sim.py
import time, random, serial, argparse

p = argparse.ArgumentParser()
p.add_argument('--port', default='COM3')
p.add_argument('--rate', type=float, default=10.0)
args = p.parse_args()

ser = serial.Serial(args.port, 115200, timeout=1)
i = 0
try:
    while True:
        i += 1
        t = time.time()
        value = 2.5 + random.gauss(0, 0.02)
        if random.random() < 0.01:  # occasional bad frame
            ser.write(b"BAD_FRAME\n")
        else:
            ser.write(f"{i},{t:.6f},{value:.5f}\n".encode())
        time.sleep(1.0/args.rate + random.uniform(-0.02, 0.02))
finally:
    ser.close()
