import * as fs from "fs"
import toml from "toml"
import { schemeMeta } from "./colorSchemes"
import { MetaAndLicense } from "./themes/common/colorScheme"

const ACCEPTED_LICENSES_FILE = `${__dirname}/../../script/licenses/zed-licenses.toml`

// Use the cargo-about configuration file as the source of truth for supported licenses.
function parseAcceptedToml(file: string): string[] {
    let buffer = fs.readFileSync(file).toString()

    let obj = toml.parse(buffer)

    if (!Array.isArray(obj.accepted)) {
        throw Error("Accepted license source is malformed")
    }

    return obj.accepted
}

function checkLicenses(
    schemeMetaWithLicense: MetaAndLicense[],
    licenses: string[]
) {
    for (const { meta } of schemeMetaWithLicense) {
        // FIXME: Add support for conjuctions and conditions
        if (licenses.indexOf(meta.license.SPDX) < 0) {
            throw Error(
                `License for theme ${meta.name} (${meta.license.SPDX}) is not supported`
            )
        }
    }
}

function generateLicenseFile(schemeMetaWithLicense: MetaAndLicense[]) {
    for (const { meta, licenseFile } of schemeMetaWithLicense) {
        const licenseText = fs.readFileSync(licenseFile).toString()
        writeLicense(meta.name, meta.url, licenseText)
    }
}

function writeLicense(
    themeName: string,
    themeUrl: string,
    licenseText: String
) {
    process.stdout.write(
        `## [${themeName}](${themeUrl})\n\n${licenseText}\n********************************************************************************\n\n`
    )
}

const acceptedLicenses = parseAcceptedToml(ACCEPTED_LICENSES_FILE)
checkLicenses(schemeMeta, acceptedLicenses)
generateLicenseFile(schemeMeta)
