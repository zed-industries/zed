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
