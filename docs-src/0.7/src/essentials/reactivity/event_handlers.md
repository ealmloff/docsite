# Event Handlers

Event handlers are used to respond to user actions. For example, an event handler could be triggered when the user clicks, scrolls, moves the mouse, or types a character.

Event handlers are attached to elements. For example, we usually don't care about all the clicks that happen within an app, only those on a particular button.

Event handlers are similar to regular attributes, but their name usually starts with `on`- and they accept closures as values. The closure will be called whenever the event it listens for is triggered and will be passed that event.

For example, to handle clicks on an element, we can specify an `onclick` handler:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_click.rs:rsx}}
```
```inject-dioxus
DemoFrame {
    event_click::AppDemo {}
}
```

## The Event object

Event handlers receive an [`Event`](https://docs.rs/dioxus-core/latest/dioxus_core/struct.Event.html) object containing information about the event. Different types of events contain different types of data. For example, mouse-related events contain [`MouseData`](https://docs.rs/dioxus/latest/dioxus/events/struct.MouseData.html), which tells you things like where the mouse was clicked and what mouse buttons were used.

In the example above, this event data was logged to the terminal:

```
Clicked! Event: UiEvent { bubble_state: Cell { value: true }, data: MouseData { coordinates: Coordinates { screen: (242.0, 256.0), client: (26.0, 17.0), element: (16.0, 7.0), page: (26.0, 17.0) }, modifiers: (empty), held_buttons: EnumSet(), trigger_button: Some(Primary) } }
Clicked! Event: UiEvent { bubble_state: Cell { value: true }, data: MouseData { coordinates: Coordinates { screen: (242.0, 256.0), client: (26.0, 17.0), element: (16.0, 7.0), page: (26.0, 17.0) }, modifiers: (empty), held_buttons: EnumSet(), trigger_button: Some(Primary) } }
```

To learn what the different event types for HTML provide, read the [events module docs](https://docs.rs/dioxus-html/latest/dioxus_html/events/index.html).

### Event propagation

Some events will trigger first on the element the event originated at upward. For example, a click event on a `button` inside a `div` would first trigger the button's event listener and then the div's event listener.

> For more information about event propagation see [the mdn docs on event bubbling](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Building_blocks/Events#event_bubbling)

If you want to prevent this behavior, you can call `stop_propagation()` on the event:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_nested.rs:rsx}}
```

## Prevent Default

Some events have a default behavior. For keyboard events, this might be entering the typed character. For mouse events, this might be selecting some text.

You can call the `prevent_default()` method on the event to stop this default behavior.

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_prevent_default.rs:prevent_default}}
```

```inject-dioxus
DemoFrame {
    event_prevent_default::AppDemo {}
}
```

Any event handlers will still be called.

## Handler Props

Sometimes, you might want to make a component that accepts an event handler. A simple example would be a `FancyButton` component, which accepts an `onclick` handler:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_handler_prop.rs:component_with_handler}}
```

Then, you can use it like any other handler:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_handler_prop.rs:usage}}
```

> Note: just like any other attribute, you can name the handlers anything you want! Any closure you pass in will automatically be turned into an `EventHandler`.

## Custom Data

Event Handlers are generic over any type, so you can pass in any data you want to them, e.g:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_handler_prop.rs:custom_data}}
```

## Returning a value from an event handler

If you want to accept a closure like an event handler that returns a value, you can use the `Callback` type. The callback type accepts two generic arguments, `I`, the input type, and `O`, the output type. Just like `EventHandler`, `Callback` is automatically converted in props and can be easily copied into anywhere in your component:

```rust, no_run
{{#include ../docs-router/src/doc_examples/event_handler_prop.rs:callback}}
```
