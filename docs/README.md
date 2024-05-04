Welcome to Zed's documentation.

This is built on push to `main` and published automatically to [https://zed.dev/docs](https://zed.dev/docs).

To preview the docs locally you will need to install [mdBook](https://rust-lang.github.io/mdBook/), and then run:

```
mdbook serve docs
```

### Internal docs:

- We have a Cloudflare router called `docs-proxy`that intercepts requests to `zed.dev/docs` and forwards them to the "docs" Cloudflare Pages project.
- CI uploads a new version to the Pages project from `.github/workflows/deploy_docs.yml` on every push to `main`.
