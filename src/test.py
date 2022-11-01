import socket
import sys 
import time

with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as client:
    client.connect("/tmp/pluggable_timer.sock")
    i = 0
    while i < 10:
        client.send(b"Client 1: hi\n")
        time.sleep(1)
        i += 1

    client.close()