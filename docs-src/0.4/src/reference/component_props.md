# Component Props

Just like you can pass arguments to a function or attributes to an element, you can pass props to a component that customize its behavior! The components we've seen so far didn't accept any props – so let's write some components that do.

## derive(Props)

Component props are a single struct annotated with `#[derive(Props)]`. For a component to accept props, the type of its argument must be `Scope<YourPropsStruct>`. Then, you can access the value of the props using `cx.props`.

There are 2 flavors of Props structs:

- Owned props:
  - Don't have an associated lifetime
  - Implement `PartialEq`, allow for memoization (if the props don't change, Dioxus won't re-render the component)
- Borrowed props:
  - [Borrow](https://doc.rust-lang.org/beta/rust-by-example/scope/borrow.html) from a parent component
  - Cannot be memoized due to lifetime constraints

### Owned Props

Owned Props are very simple – they don't borrow anything. Example:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_owned_props.rs:Likes}}
```

You can then pass prop values to the component the same way you would pass attributes to an element:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_owned_props.rs:App}}
```

```inject-dioxus
DemoFrame {
    component_owned_props::App {}
}
```

### Borrowed Props

Owned props work well if your props are easy to copy around – like a single number. But what if we need to pass a larger data type, like a String from an `App` Component to a `TitleCard` subcomponent? A naive solution might be to [`.clone()`](https://doc.rust-lang.org/std/clone/trait.Clone.html) the String, creating a copy of it for the subcomponent – but this would be inefficient, especially for larger Strings.

Rust allows for something more efficient – borrowing the String as a `&str` – this is what Borrowed Props are for!

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_borrowed_props.rs:TitleCard}}
```

We can then use the component like this:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_borrowed_props.rs:App}}
```

```inject-dioxus
DemoFrame {
    // original: component_borrowed_props::App {}
    __interactive_04::component_borrowed_props {}
}
```

Borrowed props can be very useful, but they do not allow for memorization so they will _always_ rerun when the parent scope is rerendered. Because of this Borrowed Props should be reserved for components that are cheap to rerun or places where cloning data is an issue. Using Borrowed Props everywhere will result in large parts of your app rerunning every interaction.

## Prop Options

The `#[derive(Props)]` macro has some features that let you customize the behavior of props.

### Optional Props

You can create optional fields by using the `Option<…>` type for a field:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_props_options.rs:OptionalProps}}
```

Then, you can choose to either provide them or not:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_props_options.rs:OptionalProps_usage}}
```

### Explicitly Required `Option`s

If you want to explicitly require an `Option`, and not an optional prop, you can annotate it with `#[props(!optional)]`:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_props_options.rs:ExplicitOption}}
```

Then, you have to explicitly pass either `Some("str")` or `None`:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_props_options.rs:ExplicitOption_usage}}
```

### Default Props

You can use `#[props(default = 42)]` to make a field optional and specify its default value:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_props_options.rs:DefaultComponent}}
```

Then, similarly to optional props, you don't have to provide it:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_props_options.rs:DefaultComponent_usage}}
```

### Automatic Conversion with `.into`

It is common for Rust functions to accept `impl Into<SomeType>` rather than just `SomeType` to support a wider range of parameters. If you want similar functionality with props, you can use `#[props(into)]`. For example, you could add it on a `String` prop – and `&str` will also be automatically accepted, as it can be converted into `String`:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_props_options.rs:IntoComponent}}
```

Then, you can use it so:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_props_options.rs:IntoComponent_usage}}
```

## The component macro

So far, every Component function we've seen had a corresponding ComponentProps struct to pass in props. This was quite verbose... Wouldn't it be nice to have props as simple function arguments? Then we wouldn't need to define a Props struct, and instead of typing `cx.props.whatever`, we could just use `whatever` directly!

`component` allows you to do just that. Instead of typing the "full" version:

```rust, no_run
#[derive(Props, PartialEq)]
struct TitleCardProps {
    title: String,
}

fn TitleCard(cx: Scope<TitleCardProps>) -> Element {
    cx.render(rsx!{
        h1 { "{cx.props.title}" }
    })
}
```

...you can define a function that accepts props as arguments. Then, just annotate it with `#[component]`, and the macro will turn it into a regular Component for you:

```rust, no_run
#[component]
fn TitleCard(cx: Scope, title: String) -> Element {
    cx.render(rsx!{
        h1 { "{title}" }
    })
}
```

> While the new Component is shorter and easier to read, this macro should not be used by library authors since you have less control over Prop documentation.

## Component Children

In some cases, you may wish to create a component that acts as a container for some other content, without the component needing to know what that content is. To achieve this, create a prop of type `Element`:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_element_props.rs:Clickable}}
```

Then, when rendering the component, you can pass in the output of `cx.render(rsx!(...))`:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_element_props.rs:Clickable_usage}}
```

> Note: Since `Element<'a>` is a borrowed prop, there will be no memoization.

> Warning: While it may compile, do not include the same `Element` more than once in the RSX. The resulting behavior is unspecified.

### The children field

Rather than passing the RSX through a regular prop, you may wish to accept children similarly to how elements can have children. The "magic" `children` prop lets you achieve this:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_children.rs:Clickable}}
```

This makes using the component much simpler: simply put the RSX inside the `{}` brackets – and there is no need for a `render` call or another macro!

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/component_children.rs:Clickable_usage}}
```

```inject-dioxus
DemoFrame {
    component_children::App {}
}
```
