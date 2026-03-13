<p align="center">
  <h1 align="center">DUI</h1>
  <p align="center">
    <strong>The component library Leptos deserves.</strong>
  </p>
  <p align="center">
    29 accessible, signal-driven components. Dark-first design system. Zero JavaScript.
  </p>
  <p align="center">
    <a href="https://crates.io/crates/dui-leptos"><img src="https://img.shields.io/crates/v/dui-leptos.svg" alt="crates.io"></a>
    <a href="https://docs.rs/dui-leptos"><img src="https://img.shields.io/docsrs/dui-leptos" alt="docs.rs"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License"></a>
    <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.75+-orange.svg" alt="Rust 1.75+"></a>
  </p>
</p>

---

DUI is a production-ready UI component library for [Leptos 0.7](https://leptos.dev) CSR applications. It ships 29 components with built-in accessibility (ARIA roles, keyboard navigation, focus management), a complete design token system via CSS custom properties, and dark/light mode support out of the box.

It powers three shipping applications and is built by [Dirmacs](https://dirmacs.com).

## Why DUI?

The Rust frontend ecosystem has frameworks (Leptos, Dioxus, Yew) but lacks **practical, production-tested component libraries**. Existing options are either:

- Tied to a single design system (Thaw → Fluent)
- Headless-only with no styling (Radix-Leptos)
- Early-stage or unmaintained

DUI fills the gap: **styled, accessible, and built for teams that ship.**

| | DUI | Thaw | Leptonic | Radix-Leptos |
|---|---|---|---|---|
| Components | 29 | ~60 | ~30 | 57 |
| Styled | Yes (Tailwind) | Yes (Fluent) | Yes (custom) | No (headless) |
| Accessible | Yes | Partial | Partial | Yes |
| Dark + Light | Yes | Yes | No | N/A |
| Production apps | 3 | ? | ? | 0 |
| Approach | Practical | Design-system | Framework | Primitives |

## Installation

```toml
[dependencies]
dui-leptos = "0.2"
```

> **Note**: The crate is published as `dui-leptos` on crates.io, but the Rust import is just `dui`. This is intentional — `use dui::prelude::*` is all you need.

### CSS Setup

Copy `css/dui.css` to your project's static assets and link it:

```html
<link rel="stylesheet" href="/dui.css" />
```

DUI components use [Tailwind CSS](https://tailwindcss.com) utility classes. Add `dui`'s source to your Tailwind content config so classes aren't purged:

```js
// tailwind.config.js
module.exports = {
  content: [
    "./src/**/*.rs",
    "./node_modules/dui-leptos/src/**/*.rs", // or your path
  ],
}
```

## Quick Start

```rust
use leptos::prelude::*;
use dui::prelude::*;

#[component]
fn App() -> impl IntoView {
    // Toast system — call once at root
    provide_toast();

    let count = RwSignal::new(0);

    view! {
        <Card class="p-6 max-w-sm mx-auto mt-10">
            <h1 class="text-xl font-bold mb-4">"Hello DUI"</h1>
            <p class="text-dm-muted mb-4">
                "Count: " {move || count.get().to_string()}
            </p>
            <Button
                variant=ButtonVariant::Primary
                on_click=Box::new(move |_| count.update(|n| *n += 1))
            >
                "Increment"
            </Button>
        </Card>
        <ToastContainer />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
```

## Components

### Form Controls

#### Button

Multi-variant button with loading spinner, disabled state, and focus ring.

```rust
// Variants: Primary (default), Secondary, Ghost, Danger
// Sizes: Sm, Md (default), Lg

<Button variant=ButtonVariant::Primary size=ButtonSize::Lg
    loading=loading_signal
    on_click=Box::new(move |_| { /* ... */ })
>
    "Save Changes"
</Button>

// Ghost button for toolbar actions
<Button variant=ButtonVariant::Ghost size=ButtonSize::Sm>
    "Cancel"
</Button>

// Danger with loading state
<Button variant=ButtonVariant::Danger loading=deleting>
    "Delete Account"
</Button>
```

**Props**: `variant: ButtonVariant`, `size: ButtonSize`, `loading: Signal<bool>`, `disabled: Signal<bool>`, `on_click: Option<Box<dyn Fn(MouseEvent)>>`, `class: &'static str`, `children: Children`

#### Input

Text input with label, placeholder, error state, and validation styling.

```rust
let email = RwSignal::new(String::new());

<Input
    label=Some("Email")
    value=email
    placeholder="you@example.com"
    input_type="email"
    error=Some("Invalid email address".to_string())
/>
```

**Props**: `value: RwSignal<String>`, `label: Option<&'static str>`, `placeholder: &'static str`, `input_type: &'static str`, `error: Option<String>`, `disabled: bool`, `class: &'static str`

#### Textarea

Multi-line text input with character count and configurable resize.

```rust
let bio = RwSignal::new(String::new());

<Textarea
    value=bio
    label=Some("Bio")
    placeholder="Tell us about yourself..."
    rows=4
    max_length=Some(500)
    resize="vertical"
/>
```

**Props**: `value: RwSignal<String>`, `label: Option<&'static str>`, `placeholder: &'static str`, `rows: u32`, `max_length: Option<usize>`, `resize: &'static str`, `error: Option<String>`, `disabled: bool`, `class: &'static str`

#### Select

Dropdown select with label and placeholder.

```rust
let tier = RwSignal::new(String::new());

<Select
    label=Some("Plan")
    value=tier
    placeholder="Choose a plan"
    options=vec![
        ("free".to_string(), "Free".to_string()),
        ("pro".to_string(), "Pro — $29/mo".to_string()),
        ("enterprise".to_string(), "Enterprise".to_string()),
    ]
/>
```

**Props**: `value: RwSignal<String>`, `label: Option<&'static str>`, `placeholder: &'static str`, `options: Vec<(String, String)>`, `disabled: bool`, `class: &'static str`

#### Checkbox

Custom-styled checkbox with SVG checkmark, label and optional description.

```rust
let agreed = RwSignal::new(false);

<Checkbox
    checked=agreed
    label=Some("I agree to the Terms of Service")
    description=Some("You must agree before continuing")
/>
```

**Props**: `checked: RwSignal<bool>`, `label: Option<&'static str>`, `description: Option<&'static str>`, `disabled: bool`, `class: &'static str`

**Accessibility**: `role="checkbox"`, `aria-checked`, `aria-describedby`, Enter/Space toggle

#### RadioGroup

Radio button group with vertical/horizontal layout and arrow key navigation.

```rust
let plan = RwSignal::new(String::new());

<RadioGroup
    value=plan
    name="plan"
    options=vec![
        RadioOption { value: "free".into(), label: "Free".into(),
            description: Some("5 agents, 1K requests/mo".into()), disabled: false },
        RadioOption { value: "pro".into(), label: "Pro".into(),
            description: Some("Unlimited agents, 100K requests/mo".into()), disabled: false },
        RadioOption { value: "enterprise".into(), label: "Enterprise".into(),
            description: Some("Custom limits, SLA, support".into()), disabled: false },
    ]
    orientation=RadioOrientation::Vertical
/>
```

**Props**: `value: RwSignal<String>`, `name: &'static str`, `options: Vec<RadioOption>`, `orientation: RadioOrientation`, `class: &'static str`

**Accessibility**: `role="radiogroup"`, `role="radio"`, `aria-checked`, Arrow key navigation (wrapping)

#### Switch

Toggle switch with three sizes and keyboard support.

```rust
let notifications = RwSignal::new(true);

<Switch
    checked=notifications
    label=Some("Email notifications")
    size=SwitchSize::Md
/>
```

**Props**: `checked: RwSignal<bool>`, `label: Option<&'static str>`, `size: SwitchSize`, `disabled: bool`, `class: &'static str`

**Accessibility**: `role="switch"`, `aria-checked`, Enter/Space toggle

---

### Data Display

#### Card

Container panel with optional glow effect.

```rust
<Card class="p-6">
    <h2>"Dashboard"</h2>
    <p>"Content here"</p>
</Card>

// With accent glow
<Card class="p-6" glow=true>
    <p>"Featured content"</p>
</Card>
```

**Props**: `class: String` (with `#[prop(into)]`), `glow: bool`, `children: Children`

#### Badge

Colored label for status, categories, or counts.

```rust
<Badge color=BadgeColor::Green>"Active"</Badge>
<Badge color=BadgeColor::Red>"Critical"</Badge>
<Badge color=BadgeColor::Purple>"Beta"</Badge>
```

**Props**: `color: BadgeColor` (Gray/Blue/Green/Yellow/Red/Purple), `class: &'static str`, `children: Children`

#### Table

Sortable data table with hover rows.

```rust
<Table
    headers=vec![
        TableHeader { key: "name".into(), label: "Name".into(), sortable: true },
        TableHeader { key: "status".into(), label: "Status".into(), sortable: true },
        TableHeader { key: "created".into(), label: "Created".into(), sortable: false },
    ]
    sort_key=sort_key
    sort_dir=sort_dir
    on_sort=Box::new(move |key| { /* handle sort */ })
>
    // Table rows as children
    <tr class="border-b border-dm hover:bg-dm-hover">
        <td class="px-4 py-3">"Agent Alpha"</td>
        <td class="px-4 py-3"><Badge color=BadgeColor::Green>"Active"</Badge></td>
        <td class="px-4 py-3">"2026-03-13"</td>
    </tr>
</Table>
```

**Props**: `headers: Vec<TableHeader>`, `sort_key: RwSignal<String>`, `sort_dir: RwSignal<SortDirection>`, `on_sort: Box<dyn Fn(String)>`, `class: &'static str`, `children: Children`

#### Avatar

User avatar with image or deterministic initial-color fallback.

```rust
<Avatar name="John Doe" size=AvatarSize::Lg />
<Avatar name="Jane" src=Some("/avatars/jane.jpg") size=AvatarSize::Md />
```

**Props**: `name: String`, `src: Option<String>`, `size: AvatarSize` (Xs/Sm/Md/Lg/Xl)

#### StatsCard, StatusBadge, ProgressBar, Skeleton

```rust
// Metric card
<StatsCard title="Total Agents" value="29" subtitle=Some("↑ 3 this week") />

// Status indicator with pulse animation
<StatusBadge status=ServiceStatus::Healthy label="ARES API" />

// Progress bar with color transitions
<ProgressBar value=progress label=Some("Uploading...".to_string()) />

// Loading placeholders
<Skeleton height="h-4" width="w-48" />
<SkeletonCard />
```

---

### Navigation

#### Tabs

Horizontal tab bar with active indicator and count badges.

```rust
let active = RwSignal::new("agents".to_string());

<Tabs
    items=vec![
        TabItem { key: "agents".into(), label: "Agents".into(), count: Some(29) },
        TabItem { key: "models".into(), label: "Models".into(), count: Some(11) },
        TabItem { key: "logs".into(), label: "Logs".into(), count: None },
    ]
    active_tab=active
/>
```

**Accessibility**: `role="tablist"`, `role="tab"`, `aria-selected`, `tabindex` management

#### Breadcrumb

Navigation trail with chevron separators.

```rust
<Breadcrumb items=vec![
    BreadcrumbItem { label: "Home".into(), href: Some("/".into()) },
    BreadcrumbItem { label: "Tenants".into(), href: Some("/tenants".into()) },
    BreadcrumbItem { label: "Acme Corp".into(), href: None },  // current page
] />
```

**Accessibility**: `<nav aria-label="Breadcrumb">`, `aria-current="page"` on last item

#### Sidebar

Full-height navigation sidebar with icons, sections, and user area. See source for full prop API.

---

### Overlays

#### Modal

Dialog overlay with backdrop, Escape/click-outside close, and focus trap.

```rust
let open = RwSignal::new(false);

<Button on_click=Box::new(move |_| open.set(true))>"Open Modal"</Button>

<Modal open=open title="Confirm Delete" max_width="max-w-md">
    <p class="text-dm-muted mb-4">"This action cannot be undone."</p>
    <div class="flex gap-3 justify-end">
        <Button variant=ButtonVariant::Ghost
            on_click=Box::new(move |_| open.set(false))>"Cancel"</Button>
        <Button variant=ButtonVariant::Danger>"Delete"</Button>
    </div>
</Modal>
```

**Key behavior**: Children are rendered once — visibility toggles via CSS class, not conditional rendering. The modal closes itself by writing `false` to the `open` signal.

**Accessibility**: `role="dialog"`, `aria-modal="true"`, `aria-label`

#### Dropdown

Context menu with items, separators, labels, and danger items.

```rust
let menu_open = RwSignal::new(false);

<Dropdown
    open=menu_open
    items=vec![
        DropdownEntry::Label("Actions".into()),
        DropdownEntry::Item(DropdownItem {
            id: "edit".into(), label: "Edit".into(),
            icon: None, danger: false, disabled: false,
        }),
        DropdownEntry::Separator,
        DropdownEntry::Item(DropdownItem {
            id: "delete".into(), label: "Delete".into(),
            icon: None, danger: true, disabled: false,
        }),
    ]
    on_select=Box::new(move |id| match id.as_str() {
        "edit" => { /* ... */ },
        "delete" => { /* ... */ },
        _ => {}
    })
>
    <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm
        on_click=Box::new(move |_| menu_open.update(|v| *v = !*v))
    >"Actions ▾"</Button>
</Dropdown>
```

**Accessibility**: `role="menu"`, `role="menuitem"`, Arrow key navigation, Enter to select, Escape to close

#### CommandPalette

Cmd+K style search interface with fuzzy filtering, keyboard navigation, and grouped results.

```rust
let cmd_open = RwSignal::new(false);

// Open with keyboard shortcut
Effect::new(move |_| {
    // Wire Cmd+K / Ctrl+K to toggle
});

<CommandPalette
    open=cmd_open
    items=Signal::new(vec![
        CommandItem {
            id: "new-agent".into(),
            label: "Create New Agent".into(),
            description: Some("Configure a new AI agent".into()),
            shortcut: Some("N".into()),
            group: Some("Actions".into()),
            keywords: vec!["add".into(), "agent".into()],
            icon: None,
        },
        CommandItem {
            id: "settings".into(),
            label: "Settings".into(),
            description: Some("Manage your preferences".into()),
            shortcut: Some("⌘,".into()),
            group: Some("Navigation".into()),
            keywords: vec!["preferences".into(), "config".into()],
            icon: None,
        },
    ])
    on_select=Callback::new(move |id: String| {
        // Handle selection
    })
    placeholder="Type a command or search..."
/>
```

**Accessibility**: `role="dialog"`, `role="combobox"` on search input, `role="listbox"` on results, `role="option"` with `aria-selected`, Arrow key navigation, Enter to select, Escape to close

#### Sheet

Slide-out panel from any edge of the screen.

```rust
let sheet_open = RwSignal::new(false);

<Sheet open=sheet_open side=SheetSide::Right title="Agent Details" width="max-w-md">
    <p>"Sheet content here"</p>
</Sheet>
```

**Props**: `open: RwSignal<bool>`, `side: SheetSide` (Right/Left/Top/Bottom), `title: &'static str`, `width: &'static str`, `children: Children`

#### Tooltip

Hover popup with directional arrow.

```rust
<Tooltip text="Copy to clipboard".to_string() position=TooltipPosition::Top>
    <button>"📋"</button>
</Tooltip>
```

**Accessibility**: `role="tooltip"`

---

### Feedback

#### Toast

Global notification system with 4 levels and auto-dismiss.

```rust
// Step 1: Provide at app root
provide_toast();
// ... and render the container
<ToastContainer />

// Step 2: Use anywhere via context
let toast = use_context::<ToastState>().unwrap();
toast.push(ToastLevel::Success, "Agent deployed successfully");
toast.push(ToastLevel::Error, "Failed to connect");
toast.push(ToastLevel::Warning, "Rate limit approaching");
toast.push(ToastLevel::Info, "New version available");

// Custom duration (ms)
toast.push_with_duration(ToastLevel::Error, "Connection timeout", 8000);
```

**Important**: `ToastState` is `Clone` but NOT `Copy`. Never capture it in a closure — always call `use_context::<ToastState>()` at the point of use inside `spawn_local` or standalone functions.

#### AlertBanner

Dismissible alert banner with icon and 4 severity levels.

```rust
let show_alert = RwSignal::new(true);

<AlertBanner
    level=AlertLevel::Warning
    message="Your API key expires in 3 days".to_string()
    visible=show_alert
/>
```

**Accessibility**: `role="alert"`, `aria-label="Dismiss alert"` on close button

#### EmptyState

Placeholder for empty data views with icon, title, and action slot.

```rust
<EmptyState
    icon="M..."  // SVG path data
    title="No agents yet"
    description="Create your first agent to get started."
>
    <Button variant=ButtonVariant::Primary>"Create Agent"</Button>
</EmptyState>
```

---

### Layout

#### Divider

Horizontal or vertical separator with optional label.

```rust
<Divider />
<Divider label=Some("OR") />
<Divider orientation=DividerOrientation::Vertical />
```

**Accessibility**: `role="separator"`, `aria-orientation`

#### AccordionItem

Collapsible content section with animated chevron.

```rust
<AccordionItem title="Advanced Settings" initially_open=false>
    <p>"Hidden content revealed on click"</p>
</AccordionItem>
```

**Accessibility**: `aria-expanded`, `aria-controls`

#### Kbd / KbdShortcut

Keyboard shortcut display styled as physical keycaps.

```rust
<Kbd>"⌘"</Kbd>
<KbdShortcut keys=vec!["Ctrl", "K"] />
```

---

## Theming

DUI uses 40+ CSS custom properties prefixed with `--dm-*`. Override any token to customize the entire library:

```css
:root {
  /* Backgrounds (darkest to lightest) */
  --dm-bg:          #0B1220;
  --dm-bg-panel:    #0F1724;
  --dm-bg-elevated: #141E2E;
  --dm-bg-hover:    #1A2740;
  --dm-bg-active:   #1F3050;

  /* Text */
  --dm-text:        #E8ECF2;
  --dm-text-muted:  #98A1B3;
  --dm-text-dim:    #5A6478;

  /* Accent (your brand color) */
  --dm-accent:      #4F7CFF;
  --dm-accent-2:    #7FB0FF;
  --dm-accent-dim:  rgba(79, 124, 255, 0.15);

  /* Semantic */
  --dm-success:     #34D399;
  --dm-warning:     #FBBF24;
  --dm-danger:      #F87171;
  --dm-info:        #60A5FA;

  /* Border */
  --dm-border:        rgba(255, 255, 255, 0.08);
  --dm-border-strong: rgba(255, 255, 255, 0.15);

  /* Spacing (4px base), Radius, Shadows, Transitions, Z-layers */
  /* See css/dui.css for the full token list */
}
```

### Brand Customization Example

```css
/* Purple brand */
:root {
  --dm-accent: #8B5CF6;
  --dm-accent-2: #A78BFA;
  --dm-accent-dim: rgba(139, 92, 246, 0.15);
  --dm-accent-glow: rgba(139, 92, 246, 0.35);
  --dm-interactive: #7C3AED;
}
```

### Light Mode

```html
<!-- Explicit light mode -->
<html data-theme="light">

<!-- Explicit dark mode -->
<html data-theme="dark">

<!-- Auto-detect from OS preference (default) -->
<html>

<!-- CSS class alternative -->
<html class="dm-light">
```

Light mode provides a complete alternate palette with proper contrast ratios for all tokens.

## Accessibility

Every interactive component includes:

| Feature | Implementation |
|---------|---------------|
| **ARIA roles** | `dialog`, `alert`, `tablist`/`tab`, `switch`, `checkbox`, `radio`, `radiogroup`, `menu`/`menuitem`, `progressbar`, `separator`, `tooltip`, `combobox`/`listbox` |
| **Keyboard nav** | Tab focus, Enter/Space activation, Escape to close overlays, Arrow keys for radio groups, dropdowns, and command palette |
| **Focus ring** | Visible `dm-focus-ring` class on all focusable elements |
| **Screen readers** | `aria-label`, `aria-selected`, `aria-checked`, `aria-expanded`, `aria-current`, `aria-describedby`, `aria-invalid`, `aria-modal` |
| **Reduced motion** | All animations respect `prefers-reduced-motion: reduce` |

## Animations

Built-in CSS animation utilities (all respect `prefers-reduced-motion`):

| Class | Effect |
|-------|--------|
| `.dm-animate-fade-in` | Opacity 0 → 1 |
| `.dm-animate-fade-in-up` | Slide up + fade |
| `.dm-animate-fade-in-down` | Slide down + fade |
| `.dm-animate-slide-left` | Slide from left |
| `.dm-animate-slide-right` | Slide from right |
| `.dm-animate-scale-in` | Scale 92% → 100% + fade |
| `.dm-animate-glow` | Pulsing accent glow |
| `.dm-animate-pulse` | Opacity pulse |
| `.dm-animate-spin` | 360° rotation (loading spinners) |

## FAQ

**Why "DUI"?**
It brings character. You'll remember the name.

**Does DUI work with Leptos SSR?**
Not yet. DUI targets CSR (Client-Side Rendering) via Trunk. SSR support is on the roadmap.

**Can I use DUI without Tailwind?**
The components use Tailwind utility classes internally. You need Tailwind configured in your project for the styling to work. The design tokens (`--dm-*`) are pure CSS custom properties though.

**How is DUI different from Thaw?**
Thaw implements Microsoft's Fluent Design system. DUI has its own dark-first design language and is styled with Tailwind. If you want Fluent, use Thaw. If you want a practical, customizable library that looks great out of the box, use DUI.

**Can I customize individual components?**
Yes. Every component accepts a `class` prop for additional Tailwind classes. For deeper customization, override the `--dm-*` CSS tokens or fork the component source.

## Contributing

DUI is open source under the MIT license. Contributions are welcome.

```bash
# Clone
git clone https://github.com/dirmacs/dui.git
cd dui

# Check
cargo check

# Run tests
cargo test

# Build docs
cargo doc --open
```

## License

MIT — see [LICENSE](LICENSE)

---

<p align="center">
  Built by <a href="https://dirmacs.com">Dirmacs</a>
</p>
