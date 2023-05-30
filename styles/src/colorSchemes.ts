import fs from "fs"
import path from "path"
import { ColorScheme, Meta } from "./themes/common/colorScheme"

const THEMES_DIRECTORY = path.resolve(`${__dirname}/themes`)
const STAFF_DIRECTORY = path.resolve(`${__dirname}/themes/staff`)
const IGNORE_ITEMS = ["staff", "common", "common.ts"]
const ACCEPT_EXTENSION = ".ts"

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
        filePath: path.basename(filePath),
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

function getMeta(directoryPath: string) {
    const meta: Meta[] = []

    for (const { colorScheme, filePath } of getAllColorSchemes(directoryPath)) {
        if (colorScheme.meta) {
            meta.push(colorScheme.meta)
        } else {
            throw Error(`Public theme ${filePath} must have a meta field`)
        }
    }

    return meta
}

export const colorSchemes = getColorSchemes(THEMES_DIRECTORY)
export const staffColorSchemes = getColorSchemes(STAFF_DIRECTORY)
export const schemeMeta = getMeta(THEMES_DIRECTORY)
