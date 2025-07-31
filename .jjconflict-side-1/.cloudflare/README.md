We have two cloudflare workers that let us serve some assets of this repo
from Cloudflare.

- `open-source-website-assets` is used for `install.sh`
- `docs-proxy` is used for `https://zed.dev/docs`

On push to `main`, both of these (and the files they depend on) are uploaded to Cloudflare.

### Deployment

These functions are deployed on push to main by the deploy_cloudflare.yml workflow. Worker Rules in Cloudflare intercept requests to zed.dev and proxy them to the appropriate workers.

### Testing

You can use [wrangler](https://developers.cloudflare.com/workers/cli-wrangler/install-update) to test these workers locally, or to deploy custom versions.
