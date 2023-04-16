import fs from "fs"
export const EXPORT_PATH = "./target"

export function writeToDisk(name: string, json: string, path: string): void {
    const slug = name.toLowerCase().replace(/ /g, "_")
    path = `${path}/${slug}.json`

    fs.writeFile(path, json, (err) => {
        if (err) {
            console.error(err)
            return
        }
        console.log(`Wrote ${name} to ${path}`)
    })
}
