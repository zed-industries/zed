#!/usr/bin/env python
#
# Server that will accept connections from a Vim channel.
# Used by test_channel.vim.
#
# This requires Python 2.6 or later.

from __future__ import print_function
from test_channel import ThreadedTCPServer, TestingRequestHandler, \
    writePortInFile
import socket
import threading
import os

try:
    FileNotFoundError
except NameError:
    # Python 2
    FileNotFoundError = (IOError, OSError)

if not hasattr(socket, "AF_UNIX"):
    raise NotImplementedError("Unix sockets are not supported on this platform")

class ThreadedUnixServer(ThreadedTCPServer):
    address_family = socket.AF_UNIX

class ThreadedUnixRequestHandler(TestingRequestHandler):
    pass

def main(path):
    server = ThreadedUnixServer(path, ThreadedUnixRequestHandler)

    # Start a thread with the server.  That thread will then start a new thread
    # for each connection.
    server_thread = threading.Thread(target=server.serve_forever)
    server_thread.start()

    # Signal the test harness we're ready, the port value has no meaning.
    writePortInFile(1234)

    print("Listening on {0}".format(server.server_address))

    # Main thread terminates, but the server continues running
    # until server.shutdown() is called.
    try:
        while server_thread.is_alive():
            server_thread.join(1)
    except (KeyboardInterrupt, SystemExit):
        server.shutdown()

if __name__ == "__main__":
    try:
        os.remove("Xtestsocket")
    except FileNotFoundError:
        pass
    main("Xtestsocket")
