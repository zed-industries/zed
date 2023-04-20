import { TokenFamily, tokens } from "@/theme/tokens"

const buttonTokens: TokenFamily = {
    sm: {
        container: {
            width: {
                value: 15,
                type: "sizing",
            },
            height: {
                value: 15,
                type: "sizing",
            },
        },
    },
}

tokens.addToToken("button", {
    ...buttonTokens,
})
