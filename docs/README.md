# Zed Docs

Welcome to Zed's documentation.

This is built on push to `main` and published automatically to [https://zed.dev/docs](https://zed.dev/docs).

To preview the docs locally you will need to install [mdBook](https://rust-lang.github.io/mdBook/), and then run:

```
mdbook serve docs
```

## Images and videos

To add images or videos to the docs, upload them to another location (e.g., zed.dev, GitHub's asset storage) and then link out to them from the docs.

Putting binary assets such as images in the Git repository will bloat the repository size over time.

## Internal notes:

- We have a Cloudflare router called `docs-proxy`that intercepts requests to `zed.dev/docs` and forwards them to the "docs" Cloudflare Pages project.
- CI uploads a new version to the Pages project from `.github/workflows/deploy_docs.yml` on every push to `main`.
