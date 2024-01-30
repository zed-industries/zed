#!/usr/bin/env python
#
# Server that will accept connections from a Vim channel.
# Used by test_channel.vim to test LSP functionality.
#
# This requires Python 2.6 or later.

from __future__ import print_function
import json
import socket
import sys
import time
import threading

try:
    # Python 3
    import socketserver
except ImportError:
    # Python 2
    import SocketServer as socketserver

class ThreadedTCPRequestHandler(socketserver.BaseRequestHandler):

    def setup(self):
        self.request.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)

    def debuglog(self, msg):
        if self.debug:
            with open("Xlspserver.log", "a") as myfile:
                myfile.write(msg)

    def send_lsp_req(self, msgid, method, params):
        v = {'jsonrpc': '2.0', 'id': msgid, 'method': method}
        if len(params) != 0:
            v['params'] = params
        s = json.dumps(v)
        req = "Content-Length: " + str(len(s)) + "\r\n"
        req += "Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n"
        req += "\r\n"
        req += s
        if self.debug:
            self.debuglog("SEND: ({0} bytes) '{1}'\n".format(len(req), req))
        self.request.sendall(req.encode('utf-8'))

    def send_lsp_resp(self, msgid, resp_dict):
        v = {'jsonrpc': '2.0', 'result': resp_dict}
        if msgid != -1:
            v['id'] = msgid
        s = json.dumps(v)
        resp = "Content-Length: " + str(len(s)) + "\r\n"
        resp += "Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n"
        resp += "\r\n"
        resp += s
        if self.debug:
            self.debuglog("SEND: ({0} bytes) '{1}'\n".format(len(resp), resp))
        self.request.sendall(resp.encode('utf-8'))

    def send_wrong_payload(self):
        v = 'wrong-payload'
        s = json.dumps(v)
        resp = "Content-Length: " + str(len(s)) + "\r\n"
        resp += "Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n"
        resp += "\r\n"
        resp += s
        self.request.sendall(resp.encode('utf-8'))

    def send_empty_header(self, msgid, resp_dict):
        v = {'jsonrpc': '2.0', 'id': msgid, 'result': resp_dict}
        s = json.dumps(v)
        resp = "\r\n"
        resp += s
        self.request.sendall(resp.encode('utf-8'))

    def send_empty_payload(self):
        resp = "Content-Length: 0\r\n"
        resp += "Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n"
        resp += "\r\n"
        self.request.sendall(resp.encode('utf-8'))

    def send_extra_hdr_fields(self, msgid, resp_dict):
        # test for sending extra fields in the http header
        v = {'jsonrpc': '2.0', 'id': msgid, 'result': resp_dict}
        s = json.dumps(v)
        resp = "Host: abc.vim.org\r\n"
        resp += "User-Agent: Python\r\n"
        resp += "Accept-Language: en-US,en\r\n"
        resp += "Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n"
        resp += "Content-Length: " + str(len(s)) + "\r\n"
        resp += "\r\n"
        resp += s
        self.request.sendall(resp.encode('utf-8'))

    def send_delayed_payload(self, msgid, resp_dict):
        # test for sending the hdr first and then after some delay, send the
        # payload
        v = {'jsonrpc': '2.0', 'id': msgid, 'result': resp_dict}
        s = json.dumps(v)
        resp = "Content-Length: " + str(len(s)) + "\r\n"
        resp += "\r\n"
        self.request.sendall(resp.encode('utf-8'))
        time.sleep(0.05)
        resp = s
        self.request.sendall(resp.encode('utf-8'))

    def send_hdr_without_len(self, msgid, resp_dict):
        # test for sending the http header without length
        v = {'jsonrpc': '2.0', 'id': msgid, 'result': resp_dict}
        s = json.dumps(v)
        resp = "Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n"
        resp += "\r\n"
        resp += s
        self.request.sendall(resp.encode('utf-8'))

    def send_hdr_with_wrong_len(self, msgid, resp_dict):
        # test for sending the http header with wrong length
        v = {'jsonrpc': '2.0', 'id': msgid, 'result': resp_dict}
        s = json.dumps(v)
        resp = "Content-Length: 1000\r\n"
        resp += "\r\n"
        resp += s
        self.request.sendall(resp.encode('utf-8'))

    def send_hdr_with_negative_len(self, msgid, resp_dict):
        # test for sending the http header with negative length
        v = {'jsonrpc': '2.0', 'id': msgid, 'result': resp_dict}
        s = json.dumps(v)
        resp = "Content-Length: -1\r\n"
        resp += "\r\n"
        resp += s
        self.request.sendall(resp.encode('utf-8'))

    def do_ping(self, payload):
        time.sleep(0.2)
        self.send_lsp_resp(payload['id'], 'alive')

    def do_echo(self, payload):
        self.send_lsp_resp(-1, payload)

    def do_simple_rpc(self, payload):
        # test for a simple RPC request
        self.send_lsp_resp(payload['id'], 'simple-rpc')

    def do_rpc_with_notif(self, payload):
        # test for sending a notification before replying to a request message
        self.send_lsp_resp(-1, 'rpc-with-notif-notif')
        # sleep for some time to make sure the notification is delivered
        time.sleep(0.2)
        self.send_lsp_resp(payload['id'], 'rpc-with-notif-resp')

    def do_wrong_payload(self, payload):
        # test for sending a non dict payload
        self.send_wrong_payload()
        time.sleep(0.2)
        self.send_lsp_resp(-1, 'wrong-payload')

    def do_large_payload(self, payload):
        # test for sending a large (> 64K) payload
        self.send_lsp_resp(payload['id'], payload)

    def do_rpc_resp_incorrect_id(self, payload):
        self.send_lsp_resp(-1, 'rpc-resp-incorrect-id-1')
        self.send_lsp_resp(-1, 'rpc-resp-incorrect-id-2')
        self.send_lsp_resp(1, 'rpc-resp-incorrect-id-3')
        time.sleep(0.2)
        self.send_lsp_resp(payload['id'], 'rpc-resp-incorrect-id-4')

    def do_simple_notif(self, payload):
        # notification message test
        self.send_lsp_resp(-1, 'simple-notif')

    def do_multi_notif(self, payload):
        # send multiple notifications
        self.send_lsp_resp(-1, 'multi-notif1')
        self.send_lsp_resp(-1, 'multi-notif2')

    def do_msg_with_id(self, payload):
        self.send_lsp_resp(payload['id'], 'msg-with-id')

    def do_msg_specific_cb(self, payload):
        self.send_lsp_resp(payload['id'], 'msg-specific-cb')

    def do_server_req(self, payload):
        self.send_lsp_resp(201, {'method': 'checkhealth', 'params': {'a': 20}})

    def do_extra_hdr_fields(self, payload):
        self.send_extra_hdr_fields(payload['id'], 'extra-hdr-fields')

    def do_delayed_payload(self, payload):
        self.send_delayed_payload(payload['id'], 'delayed-payload')

    def do_hdr_without_len(self, payload):
        self.send_hdr_without_len(payload['id'], 'hdr-without-len')

    def do_hdr_with_wrong_len(self, payload):
        self.send_hdr_with_wrong_len(payload['id'], 'hdr-with-wrong-len')

    def do_hdr_with_negative_len(self, payload):
        self.send_hdr_with_negative_len(payload['id'], 'hdr-with-negative-len')

    def do_empty_header(self, payload):
        self.send_empty_header(payload['id'], 'empty-header')

    def do_empty_payload(self, payload):
        self.send_empty_payload()

    def do_server_req_in_middle(self, payload):
        # Send a notification message to the client in the middle of processing
        # a request message from the client
        self.send_lsp_req(-1, 'server-req-in-middle', {'text': 'server-notif'})
        # Send a request message to the client in the middle of processing a
        # request message from the client.
        self.send_lsp_req(payload['id'], 'server-req-in-middle', {'text': 'server-req'})

    def do_server_req_in_middle_resp(self, payload):
        # After receiving a response from the client send the response to the
        # client request.
        self.send_lsp_resp(payload['id'], {'text': 'server-resp'})

    def process_msg(self, msg):
        try:
            decoded = json.loads(msg)
            if 'method' in decoded:
                test_map = {
                        'ping': self.do_ping,
                        'echo': self.do_echo,
                        'simple-rpc': self.do_simple_rpc,
                        'rpc-with-notif': self.do_rpc_with_notif,
                        'wrong-payload': self.do_wrong_payload,
                        'large-payload': self.do_large_payload,
                        'rpc-resp-incorrect-id': self.do_rpc_resp_incorrect_id,
                        'simple-notif': self.do_simple_notif,
                        'multi-notif': self.do_multi_notif,
                        'msg-with-id': self.do_msg_with_id,
                        'msg-specific-cb': self.do_msg_specific_cb,
                        'server-req': self.do_server_req,
                        'extra-hdr-fields': self.do_extra_hdr_fields,
                        'delayed-payload': self.do_delayed_payload,
                        'hdr-without-len': self.do_hdr_without_len,
                        'hdr-with-wrong-len': self.do_hdr_with_wrong_len,
                        'hdr-with-negative-len': self.do_hdr_with_negative_len,
                        'empty-header': self.do_empty_header,
                        'empty-payload': self.do_empty_payload,
                        'server-req-in-middle': self.do_server_req_in_middle,
                        'server-req-in-middle-resp': self.do_server_req_in_middle_resp,
                        }
                if decoded['method'] in test_map:
                    test_map[decoded['method']](decoded)
                else:
                    self.debuglog("Error: Unsupported method - " + decoded['method'] + "\n")
            else:
                self.debuglog("Error: 'method' field is not found\n")

        except ValueError:
            self.debuglog("Error: json decoding failed\n")

    def process_msgs(self, msgbuf):
        while True:
            sidx = msgbuf.find('Content-Length: ')
            if sidx == -1:
                # partial message received
                return msgbuf
            sidx += 16
            eidx = msgbuf.find('\r\n')
            if eidx == -1:
                # partial message received
                return msgbuf
            msglen = int(msgbuf[sidx:eidx])

            hdrend = msgbuf.find('\r\n\r\n')
            if hdrend == -1:
                # partial message received
                return msgbuf

            if msglen > len(msgbuf[hdrend + 4:]):
                if self.debug:
                    self.debuglog("Partial message ({0} bytes)\n".format(len(msgbuf)))
                # partial message received
                return msgbuf

            if self.debug:
                self.debuglog("Complete message ({0} bytes) received\n".format(msglen))

            # Remove the header
            msgbuf = msgbuf[hdrend + 4:]
            payload = msgbuf[:msglen]

            self.process_msg(payload)

            # Remove the processed message
            msgbuf = msgbuf[msglen:]

    def handle(self):
        self.debug = False
        self.debuglog("=== socket opened ===\n")
        msgbuf = ''
        while True:
            try:
                received = self.request.recv(4096).decode('utf-8')
            except socket.error:
                self.debuglog("=== socket error ===\n")
                break
            except IOError:
                self.debuglog("=== socket closed ===\n")
                break
            if received == '':
                self.debuglog("=== socket closed ===\n")
                break

            # Write the received lines into the file for debugging
            if self.debug:
                self.debuglog("RECV: ({0} bytes) '{1}'\n".format(len(received), received))

            # Can receive more than one line in a response or a partial line.
            # Accumulate all the received characters and process one line at
            # a time.
            msgbuf += received
            msgbuf = self.process_msgs(msgbuf)

class ThreadedTCPServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
    pass

def writePortInFile(port):
    # Write the port number in Xportnr, so that the test knows it.
    f = open("Xportnr", "w")
    f.write("{0}".format(port))
    f.close()

def main(host, port, server_class=ThreadedTCPServer):
    # Wait half a second before opening the port to test waittime in ch_open().
    # We do want to get the port number, get that first.  We cannot open the
    # socket, guess a port is free.
    if len(sys.argv) >= 2 and sys.argv[1] == 'delay':
        port = 13684
        writePortInFile(port)
        time.sleep(0.5)

    addrs = socket.getaddrinfo(host, port, 0, 0, socket.IPPROTO_TCP)
    # Each addr is a (family, type, proto, canonname, sockaddr) tuple
    sockaddr = addrs[0][4]
    server_class.address_family = addrs[0][0]

    server = server_class(sockaddr[0:2], ThreadedTCPRequestHandler)
    ip, port = server.server_address[0:2]

    # Start a thread with the server.  That thread will then start a new thread
    # for each connection.
    server_thread = threading.Thread(target=server.serve_forever)
    server_thread.start()

    writePortInFile(port)

    # Main thread terminates, but the server continues running
    # until server.shutdown() is called.
    try:
        while server_thread.is_alive():
            server_thread.join(1)
    except (KeyboardInterrupt, SystemExit):
        server.shutdown()

if __name__ == "__main__":
    main("localhost", 0)
