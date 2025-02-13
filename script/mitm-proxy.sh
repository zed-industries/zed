#!/usr/bin/env bash

set -e

CONTAINER_ID=$(docker run -d --rm -it -v ~/.mitmproxy:/home/mitmproxy/.mitmproxy -p 9876:8080 mitmproxy/mitmproxy mitmdump)

trap 'docker stop '"$CONTAINER_ID"' 1> /dev/null || true; exit 1' SIGINT

echo "Add the root certificate created in ~/.mitmproxy to your certificate chain for HTTP"
echo "on macOS:"
echo "sudo security add-trusted-cert -d -p ssl -p basic -k /Library/Keychains/System.keychain ~/.mitmproxy/mitmproxy-ca-cert.pem"
echo "Press enter to continue"
read

http_proxy=http://localhost:9876 cargo run

# Clean up detached proxy after running
docker stop "$CONTAINER_ID" 2>/dev/null || true
