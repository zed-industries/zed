# Hello World

Let's work through the prototypical "Build a todo app" example to showcase how we might build a simple component from scratch.

## Setup

We'll create a headline, a list of todo items, and a form to add new items.

~~~rust
struct TodoList<V: 'static> {
    headline: SharedString,
    items: Vec<TodoItem>,
    submit_form: ClickHandler<V>
}

struct TodoItem<V: 'static> {
    text: SharedString,
    completed: bool,
    delete: ClickHandler<V>
}

impl<V: 'static> TodoList<V> {
    pub fn new(
        // Here we impl Into<SharedString>
        headline: impl Into<SharedString>,
        items: Vec<TodoItem>,
        submit_form: ClickHandler<V>
    ) -> Self {
        Self {
            // and here we call .into() so we can simply pass a string
            // when creating the headline. This pattern is used throughout
            // outr components
            headline: headline.into(),
            items: Vec::new(),
            submit_form,
        }
    }
}
~~~

All of this is relatively straightforward.

We use [gpui::SharedString] in components instead of [std::string::String]. This allows us to efficiently handle shared string data across multiple components and threads without the performance overhead of copying strings.

When we want to pass an action we pass a `ClickHandler`. Whenever we want to add an action, the struct it belongs to needs to be generic over the view type `V`.

~~~rust
use gpui::hsla

impl<V: 'static> TodoList<V> {
    // ...
    fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        div().size_4().bg(hsla(50.0/360.0, 1.0, 0.5, 1.0))
    }
}
~~~

Every component needs a render method, and it should return `impl Element<V>`. This basic component will render a 16x16px yellow square on the screen.

A couple of questions might come to mind:

**Why is `size_4()` 16px, not 4px?**

gpui's style system is based on conventions created by [Tailwind CSS](https://tailwindcss.com/). Here is an example of the list of sizes for `width`: [Width - TailwindCSS Docs](https://tailwindcss.com/docs/width).

I'll quote from the Tailwind [Core Concepts](https://tailwindcss.com/docs/utility-first) docs here:

> Now I know what you’re thinking, “this is an atrocity, what a horrible mess!”
> and you’re right, it’s kind of ugly. In fact it’s just about impossible to
> think this is a good idea the first time you see it —
> you have to actually try it.

As you start using the Tailwind-style conventions you will be surprised how quick it makes it to build out UIs.

**Why `50.0/360.0` in `hsla()`?**

gpui [gpui::Hsla] use `0.0-1.0` for all its values, but it is common for tools to use `0-360` for hue.

This may change in the future, but this is a little trick that let's you use familiar looking values.

## Building out the container

Let's grab our [theme::colors::ThemeColors] from the theme and start building out a basic container.

We can access the current theme's colors like this:

~~~rust
impl<V: 'static> TodoList<V> {
    // ...
    fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        let color = cx.theme().colors()

        div().size_4().hsla(50.0/360.0, 1.0, 0.5, 1.0)
    }
}
~~~

Now we have access to the complete set of colors defined in the theme.

~~~rust
use gpui::hsla

impl<V: 'static> TodoList<V> {
    // ...
    fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        let color = cx.theme().colors()

        div().size_4().bg(color.surface)
    }
}
~~~

Let's finish up some basic styles for the container then move on to adding the other elements.

~~~rust
use gpui::hsla

impl<V: 'static> TodoList<V> {
    // ...
    fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        let color = cx.theme().colors()

        div()
            // Flex properties
            .flex()
            .flex_col()             // Stack elements vertically
            .gap_2()                // Add 8px of space between elements
            // Size properties
            .w_96()                 // Set width to 384px
            .p_4()                  // Add 16px of padding on all sides
            // Color properties
            .bg(color.surface)      // Set background color
            .text_color(color.text) // Set text color
            // Border properties
            .rounded_md()           // Add 4px of border radius
            .border_1()             // Add a 1px border
            .border_color(color.border)
            .child(
                "Hello, world!"
            )
    }
}
~~~

### Headline

TODO

### List of todo items

TODO

### Input

TODO


### End result

TODO
