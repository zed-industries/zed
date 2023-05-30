import fs from "fs"
import path from "path"
import { ColorScheme, MetaAndLicense } from "./themes/common/colorScheme"

const THEMES_DIRECTORY = path.resolve(`${__dirname}/themes`)
const STAFF_DIRECTORY = path.resolve(`${__dirname}/themes/staff`)
const IGNORE_ITEMS = ["staff", "common", "common.ts"]
const ACCEPT_EXTENSION = ".ts"
const LICENSE_FILE_NAME = "LICENSE"

function getAllTsFiles(directoryPath: string) {
    const files = fs.readdirSync(directoryPath)
    const fileList: string[] = []

    for (const file of files) {
        if (!IGNORE_ITEMS.includes(file)) {
            const filePath = path.join(directoryPath, file)

            if (fs.statSync(filePath).isDirectory()) {
                fileList.push(...getAllTsFiles(filePath))
            } else if (path.extname(file) === ACCEPT_EXTENSION) {
                fileList.push(filePath)
            }
        }
    }

    return fileList
}

function getAllColorSchemes(directoryPath: string) {
    const files = getAllTsFiles(directoryPath)
    return files.map((filePath) => ({
        colorScheme: require(filePath),
        filePath,
        fileName: path.basename(filePath),
        licenseFile: `${path.dirname(filePath)}/${LICENSE_FILE_NAME}`,
    }))
}

function getColorSchemes(directoryPath: string) {
    const colorSchemes: ColorScheme[] = []

    for (const { colorScheme } of getAllColorSchemes(directoryPath)) {
        if (colorScheme.dark) colorSchemes.push(colorScheme.dark)
        else if (colorScheme.light) colorSchemes.push(colorScheme.light)
    }

    return colorSchemes
}

function getMetaAndLicense(directoryPath: string) {
    const meta: MetaAndLicense[] = []

    for (const { colorScheme, filePath, licenseFile } of getAllColorSchemes(
        directoryPath
    )) {
        const licenseExists = fs.existsSync(licenseFile)
        if (!licenseExists) {
            throw Error(
                `Public theme should have a LICENSE file ${licenseFile}`
            )
        }

        if (!colorScheme.meta) {
            throw Error(`Public theme ${filePath} must have a meta field`)
        }

        meta.push({
            meta: colorScheme.meta,
            licenseFile,
        })
    }

    return meta
}

export const colorSchemes = getColorSchemes(THEMES_DIRECTORY)
export const staffColorSchemes = getColorSchemes(STAFF_DIRECTORY)
export const schemeMeta = getMetaAndLicense(THEMES_DIRECTORY)
