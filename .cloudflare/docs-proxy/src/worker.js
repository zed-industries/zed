export default {
  async fetch(request) {
    const url = new URL(request.url);
    const acceptHeader = request.headers.get("Accept") || "";
    const acceptsMarkdown = acceptHeader
      .split(",")
      .map((mediaType) => mediaType.split(";")[0].trim().toLowerCase())
      .includes("text/markdown");

    if (
      url.pathname === "/docs/nightly" ||
      url.pathname === "/docs/preview"
    ) {
      url.pathname += "/";
      return Response.redirect(url.toString(), 308);
    }

    const isChannelPath =
      url.pathname.startsWith("/docs/nightly/") ||
      url.pathname.startsWith("/docs/preview/");
    if (!acceptsMarkdown && isChannelPath && url.pathname.endsWith(".html")) {
      const htmlSuffix = url.pathname.endsWith("/index.html")
        ? "index.html"
        : ".html";
      url.pathname = url.pathname.slice(0, -htmlSuffix.length);
      return Response.redirect(url.toString(), 308);
    }

    if (url.pathname.startsWith("/docs/nightly/")) {
      url.hostname = "docs-nightly.pages.dev";
      url.pathname = url.pathname.replace("/docs/nightly/", "/docs/");
    } else if (url.pathname.startsWith("/docs/preview/")) {
      url.hostname = "docs-preview-5xd.pages.dev";
      url.pathname = url.pathname.replace("/docs/preview/", "/docs/");
    } else {
      url.hostname = "docs-anw.pages.dev";
    }

    if (url.pathname === "/docs.md") {
      url.pathname = "/docs/getting-started.md";
    }

    if (acceptsMarkdown) {
      url.pathname = markdownPathFor(url.pathname);
    }

    const response = await fetch(url, request);

    if (response.status === 404) {
      return fetch("https://zed.dev/404");
    }

    return response;
  },
};

/**
 * @param {string} pathname
 * @returns {string}
 */
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
