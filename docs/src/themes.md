---
title: Themes
slug: themes
short_summary: Explanation of Zed's theme system, including UI and syntax themes
section: using-zed
---

Zed themes and their underlying systems are undergoing significant development. They should not be considered stable or final. Note that themes are subject to change or removal as the theme system moves towards a stable version.

A Zed Theme refers to and affects both the UI and the syntax highlighting of Zed.

We will not have separate UI and syntax themes the like Atom, but will provide a way to override syntax styles (and more) from a single theme.

---

A variety of Zed flavoured ports of various syntax themes are available in Zed. These are mostly placeholders while we develop the system and develop our own themes. It is likely the themes that exist today in Zed will be converted into community themes in the future when we ship user-creatable themes.

## Choosing a Theme

You can choose a theme in Zed a few ways:

- by pressing `CMD + K` then `CMD + T`
- by typing `theme` in the command palette
- by editing your `settings.json` with a specific theme name:

```json
{
  "theme": "Ayu Mirage"
}
```

You can also use different themes based on system preferences:

```
{
  "theme": {
    "mode": "system",
    "light": "One Light",
    "dark": "One Dark"
  }
}
```

---

## Theme Philosophy

The theme you use in your editor is deeply personal. Whether it is focused utterly on function, or just an aesthetic that you enjoy, ultimately, a theme should be a reflection of what is important to you.

We want to provide the right balance of good defaults and a powerful toolkit to make your editor your own.

In practice, this means:

- **Default Themes**: We want to provide a set of themes that are beautiful, usable and accessible out of the box. Additionally, we would love to provide default versions of prominent themes that developers are already familiar with where possible.

- **Powerful Customization**: We want to provide a way to customize your theme to your heart's content. This could mean taking a default theme and just changing a few colors, or it could mean creating your own theme from scratch.

- **Community**: We want to enable the community to create and share their themes. We will provide a way to load themes into Zed, and a toolkit for building themes.

- **Accessibility**: We want to ensure that the default themes are accessible to all users. We will also provide guardrails for you to ensure that your custom themes are accessible. We won't, however, require you to make your theme accessible. We may tag themes that don't meet our accessibility standards as such, but we won't prevent you from using or sharing them.

We want you to have the flexibility to express yourself while ensuring the editor remains a functional tool. We're working hard to create a theme system that strikes the right balance between customization and usability.

### Accessibility in Themes

Currently, many of Zed's themes are largely inaccessible. We are working on a new accessible theme system, which will launch with Zed 1.0

A11y (accessibility) in Zed will be a long project. Likely lasting far beyond 1.0. Due to GPUI being written from the ground up we don't have access to the same a11y features that Swift, Web-based apps or [insert other language] does.

Making Zed accessible will be a joint effort between things on the Zed side, and building out features in GPUI.

For now, you can join this discussion to talk further about a11y in Zed: [Accessibility (a11y) in Zed](https://github.com/zed-industries/zed/discussions/1297)
