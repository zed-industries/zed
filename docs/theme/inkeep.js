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
  };

  function initInkeep() {
    if (typeof Inkeep !== "undefined") {
      Inkeep.ChatButton({ ...config, label: "Ask Zed" });

      var searchContainer = document.getElementById("inkeep-search");
      if (searchContainer) {
        Inkeep.SearchBar("#inkeep-search", {
          baseSettings: config.baseSettings,
          searchSettings: {
            placeholder: "Search docs...",
          },
        });
      }
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
