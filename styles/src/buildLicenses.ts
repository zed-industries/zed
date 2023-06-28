import * as fs from "fs"
import toml from "toml"
import { themes } from "./themes"
import { ThemeConfig } from "./common"

const ACCEPTED_LICENSES_FILE = `${__dirname}/../../script/licenses/zed-licenses.toml`

// Use the cargo-about configuration file as the source of truth for supported licenses.
function parseAcceptedToml(file: string): string[] {
    const buffer = fs.readFileSync(file).toString()

    const obj = toml.parse(buffer)

    if (!Array.isArray(obj.accepted)) {
        throw Error("Accepted license source is malformed")
    }

    return obj.accepted
}

function checkLicenses(themes: ThemeConfig[]) {
    for (const theme of themes) {
        if (!theme.licenseFile) {
            throw Error(`Theme ${theme.name} should have a LICENSE file`)
        }
    }
}

function generateLicenseFile(themes: ThemeConfig[]) {
    checkLicenses(themes)
    for (const theme of themes) {
        const licenseText = fs.readFileSync(theme.licenseFile).toString()
        writeLicense(theme.name, licenseText, theme.licenseUrl)
    }
}

function writeLicense(
    themeName: string,
    licenseText: string,
    licenseUrl?: string
) {
    process.stdout.write(
        licenseUrl
            ? `## [${themeName}](${licenseUrl})\n\n${licenseText}\n********************************************************************************\n\n`
            : `## ${themeName}\n\n${licenseText}\n********************************************************************************\n\n`
    )
}

const acceptedLicenses = parseAcceptedToml(ACCEPTED_LICENSES_FILE)
generateLicenseFile(themes)
