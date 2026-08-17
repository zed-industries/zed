Add another route to the router.

`path` is a string of path segments separated by `/`. Each segment
can be either static, a capture, or a wildcard.

`method_router` is the [`MethodRouter`] that should receive the request if the
path matches `path`. `method_router` will commonly be a handler wrapped in a method
router like [`get`](crate::routing::get). See [`handler`](crate::handler) for
more details on handlers.

# Static paths

Examples:

- `/`
- `/foo`
- `/users/123`

If the incoming request matches the path exactly the corresponding service will
be called.

# Captures

Paths can contain segments like `/:key` which matches any single segment and
will store the value captured at `key`.

Examples:

- `/:key`
- `/users/:id`
- `/users/:id/tweets`

Captures can be extracted using [`Path`](crate::extract::Path). See its
documentation for more details.

It is not possible to create segments that only match some types like numbers or
regular expression. You must handle that manually in your handlers.

[`MatchedPath`](crate::extract::MatchedPath) can be used to extract the matched
path rather than the actual path.

# Wildcards

Paths can end in `/*key` which matches all segments and will store the segments
captured at `key`.

Examples:

- `/*key`
- `/assets/*path`
- `/:id/:repo/*tree`

Note that `/*key` doesn't match empty segments. Thus:

- `/*key` doesn't match `/` but does match `/a`, `/a/`, etc.
- `/x/*key` doesn't match `/x` or `/x/` but does match `/x/a`, `/x/a/`, etc.

Wildcard captures can also be extracted using [`Path`](crate::extract::Path).
Note that the leading slash is not included, i.e. for the route `/foo/*rest` and
the path `/foo/bar/baz` the value of `rest` will be `bar/baz`.

# Accepting multiple methods

To accept multiple methods for the same route you can add all handlers at the
same time:

```rust
use axum::{Router, routing::{get, delete}, extract::Path};

let app = Router::new().route(
    "/",
    get(get_root).post(post_root).delete(delete_root),
);

async fn get_root() {}

async fn post_root() {}

async fn delete_root() {}
# async {
# axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
# };
```

Or you can add them one by one:

```rust
# use axum::Router;
# use axum::routing::{get, post, delete};
#
let app = Router::new()
    .route("/", get(get_root))
    .route("/", post(post_root))
    .route("/", delete(delete_root));
#
# let _: Router = app;
# async fn get_root() {}
# async fn post_root() {}
# async fn delete_root() {}
```

# More examples

```rust
use axum::{Router, routing::{get, delete}, extract::Path};

let app = Router::new()
    .route("/", get(root))
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id", get(show_user))
    .route("/api/:version/users/:id/action", delete(do_users_action))
    .route("/assets/*path", get(serve_asset));

async fn root() {}

async fn list_users() {}

async fn create_user() {}

async fn show_user(Path(id): Path<u64>) {}

async fn do_users_action(Path((version, id)): Path<(String, u64)>) {}

async fn serve_asset(Path(path): Path<String>) {}
# async {
# axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
# };
```

# Panics

Panics if the route overlaps with another route:

```rust,should_panic
use axum::{routing::get, Router};

let app = Router::new()
    .route("/", get(|| async {}))
    .route("/", get(|| async {}));
# async {
# axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
# };
```

The static route `/foo` and the dynamic route `/:key` are not considered to
overlap and `/foo` will take precedence.

Also panics if `path` is empty.
