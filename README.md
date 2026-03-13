# dirmacs-ui

Shared Leptos 0.7 component library for all Dirmacs frontend applications.

## Installation

Add to your `Cargo.toml`:

```toml
dirmacs-ui = { path = "../dirmacs-ui" }
```

Import the prelude:

```rust
use dirmacs_ui::prelude::*;
```

## Components (18)

| Component | Description |
|---|---|
| Button | Primary/secondary/danger variants, `Box<dyn Fn(MouseEvent)>` on_click |
| Card | Container with optional class prop (`String` with `#[prop(into)]`) |
| Modal | CSS visibility toggle (not conditional render), children rendered once |
| Badge | Status badges with color variants |
| Input | Text input with label and error state |
| Select | Dropdown select with options |
| Table | Data table with header/body slots |
| Tabs | Tab navigation with content panels |
| Toast | Notification system with levels (Success, Error, Warning, Info) |
| Sidebar | Navigation sidebar with collapsible sections |
| Avatar | User avatar with fallback initials |
| StatusBadge | Colored status indicator |
| StatsCard | Metric card with title and value |
| ProgressBar | Horizontal progress indicator |
| Skeleton | Loading placeholder |
| SkeletonCard | Card-shaped loading placeholder |
| EmptyState | Empty data state with icon and message |
| AlertBanner | Dismissible alert banner |
| Tooltip | Hover tooltip |

## Key Patterns

### Button on_click

```rust
<Button on_click=Box::new(move |_| { /* handler */ })>
    "Click me"
</Button>
```

For closures that aren't `Fn`, use a plain `<button on:click=...>` instead.

### Modal (CSS toggle, not conditional)

```rust
let open = RwSignal::new(false);
<Modal open=open title="My Modal">
    <p>"Modal content"</p>
</Modal>
```

Children are rendered once; open/close toggles via CSS class. Modal closes itself via the `open` RwSignal (X button, Escape key, click outside).

### Toast

```rust
// In App root:
provide_toast();

// In any page:
let toast = use_context::<ToastState>();
if let Some(t) = &toast {
    t.push(ToastLevel::Success, "Operation completed");
}
```

### Card class prop

```rust
<Card class="p-4".to_string()>
    <p>"Content"</p>
</Card>
// or with format:
<Card class=format!("p-4 {}", extra_class)>
    <p>"Content"</p>
</Card>
```
