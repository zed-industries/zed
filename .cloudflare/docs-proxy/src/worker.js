export default {
  async fetch(request, _env, _ctx) {
    const url = new URL(request.url);

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

    let res = await fetch(url, request);

    if (res.status === 404) {
      res = await fetch("https://zed.dev/404");
    }

    return res;
  },
};
