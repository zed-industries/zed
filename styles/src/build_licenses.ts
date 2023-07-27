import * as fs from "fs"
import toml from "toml"
import { themes } from "./themes"
import { ThemeConfig } from "./common"

const ACCEPTED_LICENSES_FILE = `${__dirname}/../../script/licenses/zed-licenses.toml`

// Use the cargo-about configuration file as the source of truth for supported licenses.
function parse_accepted_toml(file: string): string[] {
    const buffer = fs.readFileSync(file).toString()

    const obj = toml.parse(buffer)

    if (!Array.isArray(obj.accepted)) {
        throw Error("Accepted license source is malformed")
    }

    return obj.accepted
}

function check_licenses(themes: ThemeConfig[]) {
    for (const theme of themes) {
        if (!theme.license_file) {
            throw Error(`Theme ${theme.name} should have a LICENSE file`)
        }
    }
}

function generate_license_file(themes: ThemeConfig[]) {
    check_licenses(themes)
    for (const theme of themes) {
        const license_text = fs.readFileSync(theme.license_file).toString()
        write_license(theme.name, license_text, theme.license_url)
    }
}

function write_license(
    theme_name: string,
    license_text: string,
    license_url?: string
) {
    process.stdout.write(
        license_url
            ? `## [${theme_name}](${license_url})\n\n${license_text}\n********************************************************************************\n\n`
            : `## ${theme_name}\n\n${license_text}\n********************************************************************************\n\n`
    )
}

const accepted_licenses = parse_accepted_toml(ACCEPTED_LICENSES_FILE)
generate_license_file(themes)
