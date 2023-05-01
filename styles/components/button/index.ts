import { Theme } from "@theme/config";
import { Button, buildButton } from "./build";
import { ContainedIcon, ContainedText } from "@theme/container";

export const iconButton = (theme: Theme) => buildButton({
    theme,
    name: "iconButton",
    kind: "icon"
}) as Button<ContainedIcon>;

export const labelButton = (theme: Theme) => buildButton({
    theme,
    name: "labelButton",
    kind: "label"
}) as Button<ContainedText>;
