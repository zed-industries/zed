(function () {
  var config = {
    baseSettings: {
      // I believe this can be exposed publicly, but just to be sure I’ve
      // shared the API key with you on 1Password
      apiKey: "YOUR_INKEEP_API_KEY",
      organizationDisplayName: "Zed",
      primaryBrandColor: "#4F46E5",
    },
    aiChatSettings: {
      aiAssistantName: "Zed Assistant",
    },
    label: "Ask Zed",
  };

  function initInkeep() {
    if (typeof Inkeep !== "undefined") {
      Inkeep.ChatButton(config);
    } else {
      setTimeout(initInkeep, 100);
    }
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", initInkeep);
  } else {
    initInkeep();
  }
})();
