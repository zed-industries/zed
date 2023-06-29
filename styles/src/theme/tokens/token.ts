import { SingleColorToken, TokenTypes } from "@tokens-studio/types"

export function color_token(
    name: string,
    value: string,
    description?: string
): SingleColorToken {
    const token: SingleColorToken = {
        name,
        type: TokenTypes.COLOR,
        value,
        description,
    }

    if (!token.value || token.value === "")
        throw new Error("Color token must have a value")

    return token
}
