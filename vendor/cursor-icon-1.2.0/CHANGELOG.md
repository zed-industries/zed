# Changelog

All notable changes to cursor-icon are documented in this file.
The sections should follow the order `Packaging`, `Added`, `Changed`, `Fixed` and `Removed`.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased

## 1.2.0

- Implement `Hash` for `ParseError`.
- Add `CursorIcon::DndAsk` and `CursorIcon::AllResize` from the wayland-protocols version 1.42.

## 1.1.0

- Bump MSRV from `1.64` to `1.65`.
- Add access to alternative names for cursor icons through the `CursorIcon::alt_names` method.
