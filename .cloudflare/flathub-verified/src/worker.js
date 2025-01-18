export default {
  async fetch(request, env, _ctx) {
    const url = new URL(request.url);

    if (url.pathname !== "/.well-known/org.flathub.VerifiedApps.txt") {
      return await fetch("https://zed.dev/404");
    }

    return new Response(`${env.FLATHUB_VERIFICATION}`, {
      headers: {
        "content-type": "text/plain; charset=utf-8",
      },
    });
  },
};
