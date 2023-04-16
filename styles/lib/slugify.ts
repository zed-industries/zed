export function slugify(text: string): string {
    const specialChars = /[^\w\s]/gi // Match any character that is not a word character (\w) or whitespace (\s)
    const spaces = /\s+/g // Match one or more consecutive whitespace characters

    // Replace special characters with an empty string
    const cleanedText = text.replace(specialChars, "")

    // Replace spaces with underscores
    const slug = cleanedText.replace(spaces, "_")

    return slug.toLowerCase()
}
