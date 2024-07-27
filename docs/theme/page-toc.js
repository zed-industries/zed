let scrollTimeout;

const listenActive = () => {
  const elems = document.querySelector(".pagetoc").children;
  [...elems].forEach(el => {
    el.addEventListener("click", (event) => {
      clearTimeout(scrollTimeout);
      [...elems].forEach(el => el.classList.remove("active"));
      el.classList.add("active");
      // Prevent scroll updates for a short period
      scrollTimeout = setTimeout(() => {
        scrollTimeout = null;
      }, 100); // Adjust timing as needed
    });
  });
};

const getPagetoc = () => document.querySelector(".pagetoc") || autoCreatePagetoc();

const autoCreatePagetoc = () => {
  const main = document.querySelector("#content > main");
  const content = Object.assign(document.createElement("div"), {
    className: "content-wrap"
  });
  content.append(...main.childNodes);
  main.prepend(content);
  main.insertAdjacentHTML("afterbegin", '<div class="sidetoc"><nav class="pagetoc"></nav></div>');
  return document.querySelector(".pagetoc");
};
const updateFunction = () => {
  if (scrollTimeout) return; // Skip updates if within the cooldown period from a click
  const headers = [...document.getElementsByClassName("header")];
  const scrolledY = window.scrollY;
  let lastHeader = null;

  // Find the last header that is above the current scroll position
  for (let i = headers.length - 1; i >= 0; i--) {
    if (scrolledY >= headers[i].offsetTop) {
      lastHeader = headers[i];
      break;
    }
  }

  const pagetocLinks = [...document.querySelector(".pagetoc").children];
  pagetocLinks.forEach(link => link.classList.remove("active"));

  if (lastHeader) {
    const activeLink = pagetocLinks.find(link => lastHeader.href === link.href);
    if (activeLink) activeLink.classList.add("active");
  }
};

window.addEventListener('load', () => {
  const pagetoc = getPagetoc();
  const headers = [...document.getElementsByClassName("header")];
  headers.forEach(header => {
    const link = Object.assign(document.createElement("a"), {
      textContent: header.text,
      href: header.href,
      className: `pagetoc-${header.parentElement.tagName}`
    });
    pagetoc.appendChild(link);
  });
  updateFunction();
  listenActive();
  window.addEventListener("scroll", updateFunction);
});

