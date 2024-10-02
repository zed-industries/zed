# Theme

This crate provides the theme system for Zed.

## Overview

A theme is a collection of colors used to build a consistent appearance for UI components across the application.
To produce a theme in Zed,

A theme is made of two parts: A [ThemeFamily] and one or more [Theme]s.

//
A [ThemeFamily] contains metadata like theme name, author, and theme-specific [ColorScales] as well as a series of themes.

- [ThemeColors] - A set of colors that are used to style the UI. Refer to the [ThemeColors] documentation for more information.
