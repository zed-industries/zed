export default {
  async fetch(request, _env, _ctx) {
    const url = new URL(request.url);
    url.hostname = "docs-anw.pages.dev";

    let res = await fetch(url, request);

    if (res.status === 404) {
      res = await fetch("https://zed.dev/404");
    }

    return res;
  },
};
