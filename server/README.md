# Zed Server

This crate is what we run at https://zed.dev.

It contains our web presence as well as the backend logic for collaboration, to which we connect from the Zed client via a websocket.

## Templates

We use handlebars templates that are interpreted at runtime. When running in debug mode, you can change templates and see the latest content without restarting the server. This is enabled by the `rust-embed` crate, which we use to access the contents of the `/templates` folder at runtime. In debug mode it reads contents from the file system, but in release the templates will be embedded in the server binary.

## Static assets

We also use `rust-embed` to access the contents of the `/static` folder via the `/static/*` route. The app will pick up changes to the contents of this folder when running in debug mode.

## CSS

This site uses Tailwind CSS, which means our stylesheets don't need to change very frequently. We check `static/styles.css` into the repository, but it's actually compiled from `/styles.css` via `script/build-css`. This script runs the Tailwind compilation flow to regenerate `static/styles.css` via PostCSS.
