import Theme from "../themes/common/theme";
import { text } from "./components";

export default function projectSharedNotification(theme: Theme): Object {
  const avatarSize = 12;
  return {
    ownerAvatar: {
      height: avatarSize,
      width: avatarSize,
      cornerRadius: 6,
    },
    message: {
      ...text(theme, "sans", "primary", { size: "xs" }),
    },
    joinButton: {
      ...text(theme, "sans", "primary", { size: "xs" })
    },
    dismissButton: {
      ...text(theme, "sans", "primary", { size: "xs" })
    },
  };
}
