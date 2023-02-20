import * as fs from "fs";
import toml from "toml";
import {
  schemeMeta
} from "./colorSchemes";
import { Meta } from "./themes/common/colorScheme";
import https from "https";
import crypto from "crypto";

const accepted_licenses_file = `${__dirname}/../../script/licenses/zed-licenses.toml`

// Use the cargo-about configuration file as the source of truth for supported licenses.
function parseAcceptedToml(file: string): string[] {
  let buffer = fs.readFileSync(file).toString();

  let obj = toml.parse(buffer);

  if (!Array.isArray(obj.accepted)) {
    throw Error("Accepted license source is malformed")
  }

  return obj.accepted
}


function checkLicenses(schemeMeta: Meta[], licenses: string[]) {
  for (let meta of schemeMeta) {
    // FIXME: Add support for conjuctions and conditions
    if (licenses.indexOf(meta.license.SPDX) < 0) {
      throw Error(`License for theme ${meta.name} (${meta.license.SPDX}) is not supported`)
    }
  }
}


function getLicenseText(schemeMeta: Meta[], callback: (meta: Meta, license_text: string) => void) {
  for (let meta of schemeMeta) {
    // The following copied from the example code on nodejs.org: 
    // https://nodejs.org/api/http.html#httpgetoptions-callback
    https.get(meta.license.https_url, (res) => {
      const { statusCode } = res;

      if (statusCode < 200 || statusCode >= 300) {
        throw new Error(`Failed to fetch license for: ${meta.name}, Status Code: ${statusCode}`);
      }

      res.setEncoding('utf8');
      let rawData = '';
      res.on('data', (chunk) => { rawData += chunk; });
      res.on('end', () => {
        const hash = crypto.createHash('sha256').update(rawData).digest('hex');
        if (meta.license.license_checksum == hash) {
          callback(meta, rawData)
        } else {
          throw Error(`Checksum for ${meta.name} did not match file downloaded from ${meta.license.https_url}`)
        }
      });
    }).on('error', (e) => {
      throw e
    });
  }
}

function writeLicense(schemeMeta: Meta, text: String) {
  process.stdout.write(`## [${schemeMeta.name}](${schemeMeta.url})\n\n${text}\n********************************************************************************\n\n`)
}

const accepted_licenses = parseAcceptedToml(accepted_licenses_file);
checkLicenses(schemeMeta, accepted_licenses)

getLicenseText(schemeMeta, (meta, text) => {
  writeLicense(meta, text)
});
