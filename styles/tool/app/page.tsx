import { Spacing } from "@/theme/properties/spacing";
import { useTheme } from "./useTheme";
import { ContainedIcon, ContainedText, InteractiveContainer, InteractiveState } from "@/theme/container";
import { Button } from "@/components/button/build";

const spacingToString = (spacing: Spacing) => {
    return `${spacing.top}px ${spacing.right}px ${spacing.bottom}px ${spacing.left}px`;
}

function buttonStyles(button: Button, state: keyof InteractiveContainer<InteractiveState>) {
    const s = button[state];

    const {
        background,
        margin,
        padding,
        borderRadius,
        border,
    } = s.container;

    let styles: React.CSSProperties = {};

    styles.backgroundColor = background;
    styles.padding = padding && spacingToString(padding);
    styles.margin = margin && spacingToString(margin);
    styles.borderRadius = borderRadius && `${borderRadius}px`;
    styles.border = border && `${border.width}px solid ${border.color} ${border.overlay ? 'inset' : ''}`;
    styles.width = (typeof s.container.width) === "number" ? `${s.container.width}px` : 'auto';
    styles.height = (typeof s.container.height) === "number" ? `${s.container.height}px` : 'auto';

    if (s.hasOwnProperty('icon')) {
        const i = s as ContainedIcon;
        styles.color = i.icon.color;
    } else if (s.hasOwnProperty('text')) {
        const t = s as ContainedText;
        styles.color = t.text.color;
        styles.fontSize = `${t.text.size}px`;
        styles.fontWeight = t.text.weight;
        // styles.fontFamily = t.text.family;
        styles.lineHeight = t.text.lineHeight;
    } else {
        styles.color = '';
    }

    return styles
}

export default function Page() {
    const theme = useTheme();

    const button = theme.ui.find.case_button;

    const b = {
        default: buttonStyles(button, 'default'),
        hovered: buttonStyles(button, 'hovered'),
        pressed: buttonStyles(button, 'pressed'),
    }

    return (
        <div>
            <div style={{ margin: 40, display: 'flex', gap: '8px' }}>
                <button type="button"
                    style={{
                        ...b.default
                    }}
                >
                    Case Default
                </button>
                <button type="button"
                    style={{
                        ...b.hovered,
                        color: b.default.color,
                    }}
                >
                    Case Hovered
                </button>
                <button type="button"
                    style={{
                        ...b.pressed,
                        color: b.default.color,
                    }}
                >
                    Case Pressed
                </button>
            </div>
        </div>
    );
}
