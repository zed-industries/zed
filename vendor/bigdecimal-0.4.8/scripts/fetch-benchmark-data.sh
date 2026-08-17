#!/bin/bash
#
# Fetch benchmark data
#
# Should this just be a makefile?
#

TEST_DATA_DIR=benches/test-data

CURL=$(command -v curl)
WGET=$(command -v wget)

GITLAB_URL_PATTERN='https://gitlab.com/akubera/bigdecimal-rs/-/raw/data-files/random-test-inputs/<FILENAME>'

function fetch_benchmark_bigdecimal_file() {
	local FILENAME="random-bigdecimals-$1.txt"
	local FILEPATH=$TEST_DATA_DIR/$FILENAME

	if [ -e "$FILEPATH" ]; then
		echo "exists: $FILEPATH"
	else
		local URL=${GITLAB_URL_PATTERN//<FILENAME>/$FILENAME}
		echo "fetching: $FILEPATH from $URL"

		if [ "$CURL" ]; then
			$CURL -s --fail -L "$URL" -o "$FILEPATH"
		elif [ "$WGET" ]; then
			"$WGET" --quiet "$URL" -O "$FILEPATH"
		else
			echo "No supported fetching program"
		fi
	fi
}

mkdir -p $TEST_DATA_DIR

fetch_benchmark_bigdecimal_file "1f633481a742923ab65855c90157bbf7"
fetch_benchmark_bigdecimal_file "9a08ddaa6ce6693cdd7b8a524e088bd0"

fetch_benchmark_bigdecimal_file "4be58192272b15fc67573b39910837d0"
fetch_benchmark_bigdecimal_file "a329e61834832d89593b29f12510bdc8"
