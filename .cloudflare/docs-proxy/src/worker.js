export default {
  async fetch(request, _env, _ctx) {
    const url = new URL(request.url);
    const acceptHeader = request.headers.get("Accept") || "";
    const wantsMarkdown = acceptHeader
      .split(",")
      .map((mediaType) => mediaType.split(";")[0].trim().toLowerCase())
      .includes("text/markdown");

    if (url.pathname === "/docs/nightly") {
      url.hostname = "docs-nightly.pages.dev";
      url.pathname = "/docs/";
    } else if (url.pathname.startsWith("/docs/nightly/")) {
      url.hostname = "docs-nightly.pages.dev";
      url.pathname = url.pathname.replace("/docs/nightly/", "/docs/");
    } else if (url.pathname === "/docs/preview") {
      url.hostname = "docs-preview-5xd.pages.dev";
      url.pathname = "/docs/";
    } else if (url.pathname.startsWith("/docs/preview/")) {
      url.hostname = "docs-preview-5xd.pages.dev";
      url.pathname = url.pathname.replace("/docs/preview/", "/docs/");
    } else {
      url.hostname = "docs-anw.pages.dev";
    }

    if (url.pathname === "/docs.md") {
      url.pathname = "/docs/getting-started.md";
    }

    if (wantsMarkdown) {
      url.pathname = markdownPathFor(url.pathname);
    }

    let res = await fetch(url, request);

    if (res.status === 404) {
      res = await fetch("https://zed.dev/404");
    }

    return res;
  },
};

function markdownPathFor(pathname) {
  if (pathname === "/docs" || pathname === "/docs/") {
    return "/docs/getting-started.md";
  }

  if (pathname.endsWith("/index.md")) {
    return pathname.replace(/\/index\.md$/, "/getting-started.md");
  }

  if (pathname.endsWith(".md")) {
    return pathname;
  }

  if (pathname.endsWith(".html")) {
    return pathname.replace(/\.html$/, ".md");
  }

  if (pathname.split("/").pop().includes(".")) {
    return pathname;
  }

  if (pathname.endsWith("/")) {
    return `${pathname}getting-started.md`;
  }

  return `${pathname}.md`;
}
