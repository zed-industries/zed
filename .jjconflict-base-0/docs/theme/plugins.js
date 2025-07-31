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
