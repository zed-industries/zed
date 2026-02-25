export default {
  async fetch(request, _env, _ctx) {
    const url = new URL(request.url);

    let hostname;
    if (url.pathname.startsWith("/docs/nightly")) {
      hostname = "docs-nightly.pages.dev";
    } else if (url.pathname.startsWith("/docs/preview")) {
      hostname = "docs-preview.pages.dev";
    } else {
      hostname = "docs-anw.pages.dev";
    }

    url.hostname = hostname;
    let res = await fetch(url, request);

    if (res.status === 404) {
      res = await fetch("https://zed.dev/404");
    }

    return res;
  },
};
