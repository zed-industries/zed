# Contexts

GPUI makes extensive use of _context parameters_, typically named `cx` and positioned at the end of the parameter list, unless they're before a final function parameter. A context reference provides access to application state and services.

There are multiple kinds of contexts, and contexts implement the `Deref` trait so that a function taking `&mut AppContext` could be passed a `&mut WindowContext` or `&mut ViewContext` instead.

```
     AppContext
     /        \
ModelContext  WindowContext
              /
        ViewContext
```

- The `AppContext` forms the root of the hierarchy
- `ModelContext` and `WindowContext` both dereference to `AppContext`
- `ViewContext` dereferences to `WindowContext`

## `AppContext`

Provides access to the global application state. All other kinds of contexts ultimately deref to an `AppContext`. You can update a `Model<T>` by passing an `AppContext`, but you can't update a view. For that you need a `WindowContext`...

## `WindowContext`

Provides access to the state of an application window, and also derefs to an `AppContext`, so you can pass a window context reference to any method taking an app context. Obtain this context by calling `WindowHandle::update`.

## `ModelContext<T>`

Available when you create or update a `Model<T>`. It derefs to an `AppContext`, but also contains methods specific to the particular model, such as the ability to notify change observers or emit events.

## `ViewContext<V>`

Available when you create or update a `View<V>`. It derefs to a `WindowContext`, but also contains methods specific to the particular view, such as the ability to notify change observers or emit events.

## `AsyncAppContext` and `AsyncWindowContext`

Whereas the above contexts are always passed to your code as references, you can call `to_async` on the reference to create an async context, which has a static lifetime and can be held across `await` points in async code. When you interact with `Model`s or `View`s with an async context, the calls become fallible, because the context may outlive the window or even the app itself.

## `TestAppContext` and `TestVisualContext`

These are similar to the async contexts above, but they panic if you attempt to access a non-existent app or window, and they also contain other features specific to tests.
