export type ColorToken = {
    value: string
    type: "color"
    description?: string
}

export type TokenFamily =
    | {
          [key: string]: object
      }
    | {
          [key: string]: Token
      }

export type Token = ColorToken | TokenFamily

export type Tokens = {
    addToken: (name: string, token: Token) => void
    addToToken: (name: string, token: Token) => void
    colorToken: (value: string, description?: string) => ColorToken
    values: Token
}

function createTokens(): Tokens {
    const values: Token = {}

    const tokens: Tokens = {
        /** Creates a new token for ${name} */
        addToken: function (name: string, token: Token) {
            values[name] = token
        },
        /** Adds a token to an existing token family ${name} */
        addToToken: function (name: string, token: Token) {
            if (values[name]) {
                values[name] = {
                    ...values[name],
                    ...token,
                }
            } else {
                tokens.addToken(name, token)
            }
        },
        colorToken: function (value: string, description?: string) {
            const token: ColorToken = {
                type: "color",
                value: value,
                description: description,
            }

            if (!token.value) throw new Error("Color token must have a value")

            return token
        },
        values: values,
    }

    return tokens
}

export const tokens = createTokens()
