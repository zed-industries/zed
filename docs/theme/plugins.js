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

const copyPageActions = () => {
  const headerCopyMarkdownButton = document.getElementById(
    "copy-markdown-toggle",
  );

  const actionButtons = new Map();
  let isLoading = false;

  const showToast = (message, isSuccess = true) => {
    const existingToast = document.getElementById("copy-toast");
    existingToast?.remove();

    const toast = document.createElement("div");
    toast.id = "copy-toast";
    toast.className = `copy-toast ${isSuccess ? "success" : "error"}`;
    toast.textContent = message;

    document.body.appendChild(toast);

    setTimeout(() => {
      toast.classList.add("show");
    }, 10);

    setTimeout(() => {
      toast.classList.remove("show");
      setTimeout(() => {
        toast.parentNode?.removeChild(toast);
      }, 300);
    }, 2000);
  };

  const registerButton = (button, originalIconClass) => {
    if (!button) return;
    actionButtons.set(button, {
      originalIconClass,
      iconTimeoutId: null,
    });
  };

  registerButton(headerCopyMarkdownButton, "fa fa-copy");

  const changeButtonIcon = (button, iconClass, duration = 1000) => {
    const state = actionButtons.get(button);
    if (!state) return;

    const icon = button.querySelector("i");
    if (!icon) return;

    if (state.iconTimeoutId) {
      clearTimeout(state.iconTimeoutId);
      state.iconTimeoutId = null;
    }

    icon.className = iconClass;

    if (duration > 0) {
      state.iconTimeoutId = setTimeout(() => {
        icon.className = state.originalIconClass;
        state.iconTimeoutId = null;
      }, duration);
    }
  };

  const copyText = async (text) => {
    if (!navigator.clipboard?.writeText) {
      throw new Error("Clipboard API not supported in this browser");
    }

    await navigator.clipboard.writeText(text);
  };

  const getContentRoot = () =>
    document.querySelector("#content main .content-wrap") ||
    document.querySelector("#content > main");

  const markdownAlternateHref = () =>
    document
      .querySelector('link[rel="alternate"][type="text/markdown"]')
      ?.getAttribute("href");

  const insertPageActionButtons = () => {
    const contentRoot = getContentRoot();
    if (
      !contentRoot ||
      contentRoot.querySelector(".page-actions") ||
      !headerCopyMarkdownButton ||
      !markdownAlternateHref()
    ) {
      return;
    }

    const firstHeading = contentRoot.querySelector("h1");
    if (!firstHeading) return;

    const header = document.createElement("div");
    header.className = "page-header";

    const actions = document.createElement("div");
    actions.className = "page-actions";

    firstHeading.insertAdjacentElement("beforebegin", header);
    headerCopyMarkdownButton.classList.remove(
      "icon-button",
      "ib-hidden-mobile",
    );
    headerCopyMarkdownButton.classList.add("page-action", "page-action-icon");
    actions.append(headerCopyMarkdownButton);
    header.append(firstHeading, actions);
  };

  const markdownUrl = () => {
    const alternateHref = markdownAlternateHref();
    if (!alternateHref) return null;

    const url = new URL(alternateHref, window.location.href);
    if (
      url.origin === window.location.origin &&
      url.pathname.startsWith("/docs/") &&
      !window.location.pathname.startsWith("/docs/")
    ) {
      return url.pathname.replace(/^\/docs/, "") || "/";
    }

    if (url.origin === window.location.origin) {
      return `${url.pathname}${url.search}${url.hash}`;
    }

    return url.pathname;
  };

  const fetchAndCopyMarkdown = async (button) => {
    // Prevent multiple simultaneous requests
    if (isLoading) return;

    try {
      isLoading = true;
      changeButtonIcon(button, "fa fa-spinner fa-spin", 0); // Don't auto-restore spinner

      const url = markdownUrl();
      if (!url) {
        throw new Error("Markdown alternate link not found");
      }

      const response = await fetch(url);
      if (!response.ok) {
        throw new Error(
          `Failed to fetch markdown: ${response.status} ${response.statusText}`,
        );
      }

      const markdownContent = await response.text();

      await copyText(markdownContent);

      changeButtonIcon(button, "fa fa-check", 1000);
      showToast("Markdown copied to clipboard!");
    } catch (error) {
      console.error("Error copying markdown:", error);
      changeButtonIcon(button, "fa fa-exclamation-triangle", 2000);
      showToast("Failed to copy markdown. Please try again.", false);
    } finally {
      isLoading = false;
    }
  };

  headerCopyMarkdownButton?.addEventListener("click", () =>
    fetchAndCopyMarkdown(headerCopyMarkdownButton),
  );

  insertPageActionButtons();
};

const initializePlugins = () => {
  darkModeToggle();
  requestAnimationFrame(copyPageActions);
};

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initializePlugins);
} else {
  initializePlugins();
}

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
