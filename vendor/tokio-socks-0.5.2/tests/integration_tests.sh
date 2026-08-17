#!/usr/bin/env bash
set -x

dir="$(dirname "$(which "$0")")"
SOCK="/tmp/proxy.s"
PROXY_HOST="127.0.0.1:41080"
SOCK4="/tmp/socks4_proxy.s"
SOCKS4_PROXY_HOST="127.0.0.1:41081"


#socat tcp-listen:10007,fork exec:cat &
#echo $! > /tmp/socat-test.pid

if test -z "$@"; then
    list="socks4_no_auth socks4_userid no_auth username_auth long_username_password_auth"
else
    list="$@"
fi

socat UNIX-LISTEN:${SOCK},reuseaddr,fork TCP:${PROXY_HOST} &
socat UNIX-LISTEN:${SOCK4},reuseaddr,fork TCP:${SOCKS4_PROXY_HOST} &
sleep 2

for test in ${list}; do
    3proxy ${dir}/${test}.cfg
    sleep 1
    cargo test --test ${test} --all-features -- --test-threads 1
    test_exit_code=$?

    pkill -F /tmp/3proxy-test.pid
    sleep 1

    if test "$test_exit_code" -ne 0; then
        break
    fi
done


# pkill -F /tmp/socat-test.pid
exit ${test_exit_code}
