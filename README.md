# dui

A production-ready **Leptos 0.7** component library — 29 accessible, signal-driven components with a dark-first design system and light mode support.

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)

## Why dui?

- **29 components** — Forms, data display, navigation, overlays, feedback, layout
- **Accessible** — ARIA roles, keyboard navigation, focus management, `prefers-reduced-motion`
- **Dark-first + light mode** — Complete CSS custom property theming via `--dm-*` tokens
- **Signal-driven** — Built on Leptos reactive primitives (`Signal<T>`, `RwSignal<T>`)
- **Zero JS** — Pure Rust/WASM, no JavaScript runtime dependencies
- **Production-tested** — Powers 3 shipping applications

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
dui-leptos = "0.2"
```

Or from a local path:

```toml
[dependencies]
dui-leptos = { path = "../dui" }
```

Link the CSS in your `index.html`:

```html
<link rel="stylesheet" href="/dui.css" />
```

Import and use:

```rust
use dui::prelude::*;

#[component]
fn App() -> impl IntoView {
    provide_toast();

    view! {
        <Button variant=ButtonVariant::Primary on_click=Box::new(|_| {})>
            "Get Started"
        </Button>
        <ToastContainer />
    }
}
```

## Components (29)

### Form Controls

| Component | Description |
|-----------|-------------|
| **Button** | Primary / Secondary / Ghost / Danger variants, sizes Sm/Md/Lg, loading spinner |
| **Input** | Text / Password / Search with label, error state, validation |
| **Textarea** | Multi-line input with character count, resize control |
| **Select** | Dropdown select with placeholder, label, disabled state |
| **Checkbox** | Custom styled with SVG checkmark, label + description |
| **RadioGroup** | Vertical / horizontal orientation, per-option disable, arrow key navigation |
| **Switch** | Toggle switch Sm/Md/Lg, ARIA `role="switch"`, keyboard support |

### Data Display

| Component | Description |
|-----------|-------------|
| **Badge** | Gray / Blue / Green / Yellow / Red / Purple color variants |
| **Card** | Container with glow variants, optional header/footer slots |
| **Table** | Sortable columns, hover rows, ascending/descending/none cycle |
| **Avatar** | Image or initials fallback, 5 sizes (Xs–Xl), deterministic color from name |
| **StatsCard** | Metric card with title, value, subtitle, icon |
| **StatusBadge** | Healthy (pulsing) / Degraded / Down / Unknown with dot indicator |
| **ProgressBar** | Animated fill with color transitions (blue → green at 80%) |
| **Skeleton** | Shimmer loading placeholder + SkeletonText + SkeletonCard helpers |

### Navigation

| Component | Description |
|-----------|-------------|
| **Sidebar** | Collapsible nav with icons, active indicator, user section |
| **Tabs** | Horizontal tab bar with `role="tablist"`, count badges, active underline |
| **Breadcrumb** | Navigation trail with separator, `aria-current="page"` on last item |

### Overlays

| Component | Description |
|-----------|-------------|
| **Modal** | Dialog with backdrop, Escape/click-outside close, `role="dialog"` |
| **Tooltip** | Hover popup with arrow, 4 positions (Top/Bottom/Left/Right) |
| **Dropdown** | Menu with items/separators/labels, keyboard nav, danger items |
| **Sheet** | Slide-out panel from any edge (Right/Left/Top/Bottom) |
| **CommandPalette** | Cmd+K search interface with fuzzy filter, keyboard nav, grouping |

### Feedback

| Component | Description |
|-----------|-------------|
| **Toast** | Notification system with 4 levels, auto-dismiss, context-based API |
| **AlertBanner** | Dismissible banner with `role="alert"`, 4 severity levels |
| **EmptyState** | Placeholder with icon, title, description, action slot |

### Layout

| Component | Description |
|-----------|-------------|
| **Divider** | Horizontal/vertical separator with optional label |
| **AccordionItem** | Collapsible section with rotating chevron, `aria-expanded` |
| **Kbd** | Keyboard shortcut display (keycap styling) + KbdShortcut helper |

## Theming

All components use CSS custom properties with the `--dm-*` prefix. Override them to customize:

```css
:root {
  --dm-accent: #4F7CFF;      /* Primary brand color */
  --dm-bg: #0B1220;           /* Darkest background */
  --dm-bg-panel: #0F1724;     /* Card/panel background */
  --dm-text: #E8ECF2;         /* Primary text */
  --dm-border: rgba(255, 255, 255, 0.08);
  --dm-radius-md: 8px;
  /* ... 40+ tokens total */
}
```

### Light Mode

Three ways to enable light mode:

```html
<!-- 1. Via data attribute -->
<html data-theme="light">

<!-- 2. Via CSS class -->
<html class="dm-light">

<!-- 3. Automatic via OS preference (no attribute needed) -->
```

Force dark mode regardless of OS preference:

```html
<html data-theme="dark">
```

## Key Patterns

### Button

```rust
<Button
    variant=ButtonVariant::Primary
    size=ButtonSize::Lg
    loading=Signal::new(false)
    on_click=Box::new(|_| { /* handler */ })
>
    "Save Changes"
</Button>
```

### Modal

Children are rendered once; visibility toggles via CSS class. Closes itself via the `open` signal.

```rust
let open = RwSignal::new(false);
<Modal open=open title="Confirm Action" max_width="max-w-md">
    <p>"Are you sure?"</p>
</Modal>
```

### Toast

```rust
// In App root:
provide_toast();

// In any component:
let toast = use_context::<ToastState>().unwrap();
toast.push(ToastLevel::Success, "Tenant created");
toast.push_with_duration(ToastLevel::Error, "Failed", 8000);
```

### CommandPalette

```rust
let cmd_open = RwSignal::new(false);
let items = Signal::new(vec![
    CommandItem {
        id: "new-agent".into(),
        label: "Create Agent".into(),
        shortcut: Some("N".into()),
        group: Some("Actions".into()),
        ..Default::default()
    },
]);

<CommandPalette
    open=cmd_open
    items=items
    on_select=Callback::new(move |id: String| { /* handle */ })
/>
```

### Form Controls

```rust
let name = RwSignal::new(String::new());
let agreed = RwSignal::new(false);
let plan = RwSignal::new(String::new());
let notifications = RwSignal::new(true);

<Input label=Some("Name") value=name placeholder="Enter name" />
<Textarea value=name rows=4 max_length=Some(500) />
<Checkbox checked=agreed label=Some("I agree to the terms") />
<RadioGroup
    value=plan
    options=vec![
        RadioOption { value: "free".into(), label: "Free".into(), ..Default::default() },
        RadioOption { value: "pro".into(), label: "Pro".into(), ..Default::default() },
    ]
/>
<Switch checked=notifications label=Some("Email notifications") />
```

## Accessibility

All interactive components include:

- **ARIA roles**: `dialog`, `alert`, `tablist`/`tab`, `switch`, `checkbox`, `radio`, `radiogroup`, `menu`/`menuitem`, `progressbar`, `separator`, `tooltip`, `combobox`/`listbox`
- **Keyboard navigation**: Tab focus, Enter/Space activation, Escape to close overlays, Arrow keys for radio groups and dropdowns
- **Focus management**: `dm-focus-ring` class for visible focus indicators, `tabindex` management
- **Screen readers**: `aria-label`, `aria-selected`, `aria-checked`, `aria-expanded`, `aria-current`, `aria-describedby`, `aria-invalid`
- **Motion**: Respects `prefers-reduced-motion` — all animations disabled

## Design Tokens

The full token set is defined in `css/dui.css`:

| Category | Tokens |
|----------|--------|
| **Backgrounds** | `--dm-bg`, `--dm-bg-panel`, `--dm-bg-elevated`, `--dm-bg-hover`, `--dm-bg-active` |
| **Text** | `--dm-text`, `--dm-text-muted`, `--dm-text-dim` |
| **Accent** | `--dm-accent`, `--dm-accent-2`, `--dm-accent-dim`, `--dm-accent-glow` |
| **Semantic** | `--dm-success`, `--dm-warning`, `--dm-danger`, `--dm-info`, `--dm-purple` (+ dim variants) |
| **Border** | `--dm-border`, `--dm-border-strong`, `--dm-border-accent` |
| **Spacing** | `--dm-space-{1..16}` (4px base) |
| **Radius** | `--dm-radius-{sm,md,lg,xl,full}` |
| **Shadows** | `--dm-shadow-{sm,md,lg,glow}` |
| **Transitions** | `--dm-ease`, `--dm-ease-out`, `--dm-dur-{fast,base,slow}` |
| **Z-index** | `--dm-z-{dropdown,modal,toast,tooltip}` |

## Animations

Built-in animation utilities:

```css
.dm-animate-fade-in       /* Opacity 0→1 */
.dm-animate-fade-in-up    /* Slide up + fade */
.dm-animate-fade-in-down  /* Slide down + fade */
.dm-animate-slide-left    /* Slide from left */
.dm-animate-slide-right   /* Slide from right */
.dm-animate-scale-in      /* Scale 92%→100% + fade */
.dm-animate-glow          /* Pulsing glow */
.dm-animate-pulse         /* Opacity pulse */
.dm-animate-spin          /* 360° rotation */
```

## License

MIT
