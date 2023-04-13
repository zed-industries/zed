import { Theme } from "@/theme/config";

export default function tabBarStyle(theme: Theme) {
    return {
        tabBar: {
            backgroundColor: theme.colors.neutral[100],
        },
    };
}
