function detectOS() {
  var userAgent = navigator.userAgent;

  var platform = navigator.platform;
  var macosPlatforms = ["Macintosh", "MacIntel", "MacPPC", "Mac68K"];
  var windowsPlatforms = ["Win32", "Win64", "Windows", "WinCE"];
  var iosPlatforms = ["iPhone", "iPad", "iPod"];

  if (macosPlatforms.indexOf(platform) !== -1) {
    return "Mac";
  } else if (iosPlatforms.indexOf(platform) !== -1) {
    return "iOS";
  } else if (windowsPlatforms.indexOf(platform) !== -1) {
    return "Windows";
  } else if (/Android/.test(userAgent)) {
    return "Android";
  } else if (/Linux/.test(platform)) {
    return "Linux";
  }

  return "Unknown";
}

// Usage
var os = detectOS();
console.log("Operating System:", os);

(function updateKeybindings() {
  const os = detectOS();
  const isMac = os === "Mac" || os === "iOS";

  function processKeybinding(element) {
    const [macKeybinding, linuxKeybinding] = element.textContent.split("|");
    element.textContent = isMac ? macKeybinding : linuxKeybinding;
    element.classList.add("keybinding");
  }

  function walkDOM(node) {
    if (node.nodeType === Node.ELEMENT_NODE) {
      if (node.tagName.toLowerCase() === "kbd") {
        processKeybinding(node);
      } else {
        Array.from(node.children).forEach(walkDOM);
      }
    }
  }

  // Start the process from the body
  walkDOM(document.body);
})();

function darkModeToggle() {
  var html = document.documentElement;
  var themeToggleButton = document.getElementById("theme-toggle");
  var themePopup = document.getElementById("theme-list");
  var themePopupButtons = themePopup.querySelectorAll("button");

  function setTheme(theme) {
    html.setAttribute("data-theme", theme);
    html.setAttribute("data-color-scheme", theme);
    html.className = theme;
    localStorage.setItem("mdbook-theme", theme);

    // Force a repaint to ensure the changes take effect in the client immediately
    document.body.style.display = "none";
    document.body.offsetHeight;
    document.body.style.display = "";
  }

  themeToggleButton.addEventListener("click", function (event) {
    event.preventDefault();
    themePopup.style.display =
      themePopup.style.display === "block" ? "none" : "block";
  });

  themePopupButtons.forEach(function (button) {
    button.addEventListener("click", function () {
      setTheme(this.id);
      themePopup.style.display = "none";
    });
  });

  document.addEventListener("click", function (event) {
    if (
      !themePopup.contains(event.target) &&
      !themeToggleButton.contains(event.target)
    ) {
      themePopup.style.display = "none";
    }
  });

  // Set initial theme
  var currentTheme = localStorage.getItem("mdbook-theme");
  if (currentTheme) {
    setTheme(currentTheme);
  } else {
    // If no theme is set, use the system's preference
    var systemPreference = window.matchMedia("(prefers-color-scheme: dark)")
      .matches
      ? "dark"
      : "light";
    setTheme(systemPreference);
  }

  // Listen for system's preference changes
  const darkModeMediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
  darkModeMediaQuery.addEventListener("change", function (e) {
    if (!localStorage.getItem("mdbook-theme")) {
      setTheme(e.matches ? "dark" : "light");
    }
  });
}

const copyMarkdown = () => {
  const copyButton = document.getElementById("copy-markdown-toggle");
  if (!copyButton) return;

  // Store the original icon class, loading state, and timeout reference
  const originalIconClass = "fa fa-copy";
  let isLoading = false;
  let iconTimeoutId = null;

  const getCurrentPagePath = () => {
    const pathname = window.location.pathname;

    // Handle root docs path
    if (pathname === "/docs/" || pathname === "/docs") {
      return "getting-started.md";
    }

    // Remove /docs/ prefix and .html suffix, then add .md
    const cleanPath = pathname
      .replace(/^\/docs\//, "")
      .replace(/\.html$/, "")
      .replace(/\/$/, "");

    return cleanPath ? cleanPath + ".md" : "getting-started.md";
  };

  const showToast = (message, isSuccess = true) => {
    // Remove existing toast if any
    const existingToast = document.getElementById("copy-toast");
    existingToast?.remove();

    const toast = document.createElement("div");
    toast.id = "copy-toast";
    toast.className = `copy-toast ${isSuccess ? "success" : "error"}`;
    toast.textContent = message;

    document.body.appendChild(toast);

    // Show toast with animation
    setTimeout(() => {
      toast.classList.add("show");
    }, 10);

    // Hide and remove toast after 2 seconds
    setTimeout(() => {
      toast.classList.remove("show");
      setTimeout(() => {
        toast.parentNode?.removeChild(toast);
      }, 300);
    }, 2000);
  };

  const changeButtonIcon = (iconClass, duration = 1000) => {
    const icon = copyButton.querySelector("i");
    if (!icon) return;

    // Clear any existing timeout
    if (iconTimeoutId) {
      clearTimeout(iconTimeoutId);
      iconTimeoutId = null;
    }

    icon.className = iconClass;

    if (duration > 0) {
      iconTimeoutId = setTimeout(() => {
        icon.className = originalIconClass;
        iconTimeoutId = null;
      }, duration);
    }
  };

  const fetchAndCopyMarkdown = async () => {
    // Prevent multiple simultaneous requests
    if (isLoading) return;

    try {
      isLoading = true;
      changeButtonIcon("fa fa-spinner fa-spin", 0); // Don't auto-restore spinner

      const pagePath = getCurrentPagePath();
      const rawUrl = `https://raw.githubusercontent.com/zed-industries/zed/main/docs/src/${pagePath}`;

      const response = await fetch(rawUrl);
      if (!response.ok) {
        throw new Error(
          `Failed to fetch markdown: ${response.status} ${response.statusText}`,
        );
      }

      const markdownContent = await response.text();

      // Copy to clipboard using modern API
      if (navigator.clipboard?.writeText) {
        await navigator.clipboard.writeText(markdownContent);
      } else {
        // Fallback: throw error if clipboard API isn't available
        throw new Error("Clipboard API not supported in this browser");
      }

      changeButtonIcon("fa fa-check", 1000);
      showToast("Page content copied to clipboard!");
    } catch (error) {
      console.error("Error copying markdown:", error);
      changeButtonIcon("fa fa-exclamation-triangle", 2000);
      showToast("Failed to copy markdown. Please try again.", false);
    } finally {
      isLoading = false;
    }
  };

  copyButton.addEventListener("click", fetchAndCopyMarkdown);
};

// Initialize functionality when DOM is loaded
document.addEventListener("DOMContentLoaded", () => {
  copyMarkdown();
});
