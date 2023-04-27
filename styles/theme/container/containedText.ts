import { TextStyle } from "@theme/text"
import { ContainerStyle } from "."

export interface ContainedText {
    container: ContainerStyle
    text: TextStyle
}

export interface ContainedTextProps {
    text: TextStyle,
    container: ContainerStyle,
}

// Placeholder for containedText logic
export const containedText = ({ text, container }: ContainedTextProps): ContainedText => {
    return {
        text,
        container
    }
}
