import Theme from "../themes/common/theme";
import { text } from "./components";

export default function incomingCallNotification(theme: Theme): Object {
  const avatarSize = 12;
  return {
    callerAvatar: {
      height: avatarSize,
      width: avatarSize,
      cornerRadius: 6,
    },
    callerUsername: {
      ...text(theme, "sans", "primary", { size: "xs" }),
    },
    acceptButton: {
      ...text(theme, "sans", "primary", { size: "xs" })
    },
    declineButton: {
      ...text(theme, "sans", "primary", { size: "xs" })
    },
  };
}
