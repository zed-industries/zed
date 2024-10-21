export default {
  async fetch(request, _env, _ctx) {
    const url = new URL(request.url);
    url.hostname = "docs-anw.pages.dev";

    // These pages were removed, but may still be served due to Cloudflare's
    // [asset retention](https://developers.cloudflare.com/pages/configuration/serving-pages/#asset-retention).
    if (
      url.pathname === "/docs/assistant/context-servers" ||
      url.pathname === "/docs/assistant/model-context-protocol"
    ) {
      return await fetch("https://zed.dev/404");
    }

    let res = await fetch(url, request);

    if (res.status === 404) {
      res = await fetch("https://zed.dev/404");
    }

    return res;
  },
};
