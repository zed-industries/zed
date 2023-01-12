#!/usr/bin/env node

const fs = require('fs');

const file_name = (path) => "./licenses/".concat(path);

const writeFile = (path, data) => {
  fs.writeFile(file_name(path), data, (err) => {
    if (err) throw err;
    console.log("Saved file")
  });
}

main();

async function main() {
  console.log("Here!");

  writeFile("test.tx", "DATA DATA DATA")

  // Next steps:
  // 1a. Add wiring in Zed to check for a licenses markdown file
  // 1b. Add wiring in Zed.dev for builds to publish licenses alongside releases as well as licenses for Zed.dev itself
  // 2. Figure out how to run those commands and get the license text for each MIT and Apache licensed software
  // 3. Add in the configuration file:
  //      a. and refactor this script to have types of licenses
  //      b. add callback handlers for each type,
  //      c. check if the handler succeeds
}