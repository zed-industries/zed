target/debug/examples/parse_dump_yaml test-conf/fonts.conf > test-conf/fonts.yaml

for f in test-conf/conf.d/*.conf; do
    target/debug/examples/parse_dump_yaml $f > "${f%.conf}.yaml"
done
