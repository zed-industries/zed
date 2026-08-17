# This is just a convenience script to filter the important facts out of the criterion report
cargo bench --bench mutex | grep -E "cont|time" | grep -v -E "Warming|Analyzing|Benchmarking|Warning"