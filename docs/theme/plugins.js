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

var os = detectOS();
console.log("Operating System:", os);

// Defer keybinding processing to avoid blocking initial render
function updateKeybindings() {
  const os = detectOS();
  const isMac = os === "Mac" || os === "iOS";

  function processKeybinding(element) {
    const [macKeybinding, linuxKeybinding] = element.textContent.split("|");
    element.textContent = isMac ? macKeybinding : linuxKeybinding;
    element.classList.add("keybinding");
  }

  // Process all kbd elements at once (more efficient than walking entire DOM)
  const kbdElements = document.querySelectorAll("kbd");
  kbdElements.forEach(processKeybinding);
}

// Use requestIdleCallback if available, otherwise requestAnimationFrame
if (typeof requestIdleCallback === "function") {
  requestIdleCallback(updateKeybindings);
} else {
  requestAnimationFrame(updateKeybindings);
}

function darkModeToggle() {
  var html = document.documentElement;

  function setTheme(theme) {
    html.setAttribute("data-theme", theme);
    html.setAttribute("data-color-scheme", theme);
    html.className = theme;
    localStorage.setItem("mdbook-theme", theme);
  }

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
  darkModeToggle();
  copyMarkdown();
});

// Collapsible sidebar navigation for entire sections
// Note: Initial collapsed state is applied in index.hbs to prevent flicker
function initCollapsibleSidebar() {
  var sidebar = document.getElementById("sidebar");
  if (!sidebar) return;

  var chapterList = sidebar.querySelector("ol.chapter");
  if (!chapterList) return;

  var partTitles = Array.from(chapterList.querySelectorAll("li.part-title"));

  partTitles.forEach(function (partTitle) {
    // Get all sibling elements that belong to this section
    var sectionItems = getSectionItems(partTitle);

    if (sectionItems.length > 0) {
      setupCollapsibleSection(partTitle, sectionItems);
    }
  });
}

// Saves the list of collapsed section names to sessionStorage
// This gets reset when the tab is closed and opened again
function saveCollapsedSections() {
  var collapsedSections = [];
  var partTitles = document.querySelectorAll(
    "#sidebar li.part-title.collapsible",
  );

  partTitles.forEach(function (partTitle) {
    if (!partTitle.classList.contains("expanded")) {
      collapsedSections.push(partTitle._sectionName);
    }
  });

  try {
    sessionStorage.setItem(
      "sidebar-collapsed-sections",
      JSON.stringify(collapsedSections),
    );
  } catch (e) {
    // sessionStorage might not be available
  }
}

function getSectionItems(partTitle) {
  var items = [];
  var sibling = partTitle.nextElementSibling;

  while (sibling) {
    // Stop when we hit another part-title
    if (sibling.classList.contains("part-title")) {
      break;
    }
    items.push(sibling);
    sibling = sibling.nextElementSibling;
  }

  return items;
}

function setupCollapsibleSection(partTitle, sectionItems) {
  partTitle.classList.add("collapsible");
  partTitle.setAttribute("role", "button");
  partTitle.setAttribute("tabindex", "0");
  partTitle._sectionItems = sectionItems;

  var isCurrentlyCollapsed = partTitle._isCollapsed;
  if (isCurrentlyCollapsed) {
    partTitle.setAttribute("aria-expanded", "false");
  } else {
    partTitle.classList.add("expanded");
    partTitle.setAttribute("aria-expanded", "true");
  }

  partTitle.addEventListener("click", function (e) {
    e.preventDefault();
    toggleSection(partTitle);
  });

  // a11y: Add keyboard support (Enter and Space)
  partTitle.addEventListener("keydown", function (e) {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      toggleSection(partTitle);
    }
  });
}

function toggleSection(partTitle) {
  var isExpanded = partTitle.classList.contains("expanded");
  var sectionItems = partTitle._sectionItems;
  var spacerAfter = partTitle._spacerAfter;

  if (isExpanded) {
    partTitle.classList.remove("expanded");
    partTitle.setAttribute("aria-expanded", "false");
    sectionItems.forEach(function (item) {
      item.classList.add("section-hidden");
    });
    if (spacerAfter) {
      spacerAfter.classList.add("section-hidden");
    }
  } else {
    partTitle.classList.add("expanded");
    partTitle.setAttribute("aria-expanded", "true");
    sectionItems.forEach(function (item) {
      item.classList.remove("section-hidden");
    });
    if (spacerAfter) {
      spacerAfter.classList.remove("section-hidden");
    }
  }

  saveCollapsedSections();
}

document.addEventListener("DOMContentLoaded", function () {
  initCollapsibleSidebar();
});
