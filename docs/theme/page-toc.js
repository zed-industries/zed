let scrollTimeout;

const listenActive = () => {
  const elems = document.querySelector(".pagetoc").children;
  [...elems].forEach((el) => {
    el.addEventListener("click", (_) => {
      clearTimeout(scrollTimeout);
      [...elems].forEach((el) => el.classList.remove("active"));
      el.classList.add("active");

      scrollTimeout = setTimeout(() => {
        scrollTimeout = null;
      }, 100);
    });
  });
};

const autoCreatePagetoc = () => {
  const main = document.querySelector("#content > main");
  const content = Object.assign(document.createElement("div"), {
    className: "content-wrap",
  });
  content.append(...main.childNodes);
  main.prepend(content);
  main.insertAdjacentHTML(
    "afterbegin",
    '<div class="toc-container"><nav class="pagetoc"></nav></div>',
  );
  return document.querySelector(".pagetoc");
};

const getPagetoc = () =>
  document.querySelector(".pagetoc") || autoCreatePagetoc();

const updateFunction = () => {
  if (scrollTimeout) return;

  const headers = [...document.getElementsByClassName("header")];
  if (headers.length === 0) return;

  const threshold = 100;
  let activeHeader = null;

  for (const header of headers) {
    const rect = header.getBoundingClientRect();

    if (rect.top <= threshold) {
      activeHeader = header;
    }
  }

  if (!activeHeader && headers.length > 0) {
    activeHeader = headers[0];
  }

  const pagetocLinks = [...document.querySelector(".pagetoc").children];
  pagetocLinks.forEach((link) => link.classList.remove("active"));

  if (activeHeader) {
    const activeLink = pagetocLinks.find(
      (link) => activeHeader.href === link.href,
    );
    if (activeLink) activeLink.classList.add("active");
  }
};

document.addEventListener("DOMContentLoaded", () => {
  const pagetoc = getPagetoc();
  const headers = [...document.getElementsByClassName("header")];

  const nonH1Headers = headers.filter(
    (header) => !header.parentElement.tagName.toLowerCase().startsWith("h1"),
  );
  const tocContainer = document.querySelector(".toc-container");

  if (nonH1Headers.length === 0) {
    if (tocContainer) {
      tocContainer.classList.add("no-toc");
    }
    return;
  }

  if (tocContainer) {
    tocContainer.classList.add("has-toc");
  }

  const tocTitle = Object.assign(document.createElement("p"), {
    className: "toc-title",
    textContent: "On This Page",
  });

  pagetoc.appendChild(tocTitle);

  headers.forEach((header) => {
    const link = Object.assign(document.createElement("a"), {
      textContent: header.text,
      href: header.href,
      className: `pagetoc-${header.parentElement.tagName}`,
    });
    pagetoc.appendChild(link);
  });

  updateFunction();
  listenActive();

  const pageElement = document.querySelector(".page");
  if (pageElement) {
    pageElement.addEventListener("scroll", updateFunction);
  } else {
    window.addEventListener("scroll", updateFunction);
  }
});
