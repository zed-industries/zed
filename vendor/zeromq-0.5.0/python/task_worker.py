# encoding: utf-8
#
#   Task worker - design 2
#   Adds pub-sub flow to receive and respond to kill signal
#
#   Author: Jeremy Avnet (brainsik) <spork(dash)zmq(at)theory(dot)org>
#

import sys
import time
import zmq

context = zmq.Context()

# Socket to receive messages on
receiver = context.socket(zmq.PULL)
receiver.connect("tcp://localhost:5557")

# Socket to send messages to
sender = context.socket(zmq.PUSH)
sender.connect("tcp://localhost:5558")

# Socket for control input
controller = context.socket(zmq.SUB)
controller.connect("tcp://localhost:5559")
controller.setsockopt(zmq.SUBSCRIBE, b"")

# Process messages from receiver and controller
poller = zmq.Poller()
poller.register(receiver, zmq.POLLIN)
poller.register(controller, zmq.POLLIN)
# Process messages from both sockets
while True:
    socks = dict(poller.poll())

    if socks.get(receiver) == zmq.POLLIN:
        message = receiver.recv_string()

        # Process task
        workload = int(message)  # Workload in msecs

        # Do the work
        time.sleep(workload / 1000.0)

        # Send results to sink
        sender.send_string(message)

        # Simple progress indicator for the viewer
        sys.stdout.write(".")
        sys.stdout.flush()

    # Any waiting controller command acts as 'KILL'
    if socks.get(controller) == zmq.POLLIN:
        break
print("Done")
# Finished
receiver.close()
sender.close()
controller.close()
context.term()
