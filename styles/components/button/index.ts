import { Theme } from "@theme/config";
import { buildButton } from "./build";

export const iconButton = (theme: Theme) => buildButton({
    theme,
    name: "iconButton",
    kind: "icon"
});

export const labelButton = (theme: Theme) => buildButton({
    theme,
    name: "labelButton",
    kind: "label"
});
