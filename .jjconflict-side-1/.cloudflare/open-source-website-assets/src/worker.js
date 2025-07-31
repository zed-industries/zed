export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    const key = url.pathname.slice(1);

    const object = await env.OPEN_SOURCE_WEBSITE_ASSETS_BUCKET.get(key);
    if (!object) {
      return await fetch("https://zed.dev/404");
    }

    const headers = new Headers();
    object.writeHttpMetadata(headers);
    headers.set("etag", object.httpEtag);

    return new Response(object.body, {
      headers,
    });
  },
};
