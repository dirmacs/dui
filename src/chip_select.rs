use leptos::prelude::*;

/// Multi-option toggle selector with horizontally-wrapping pill buttons.
///
/// Selected chips get accent border + fill. Supports keyboard navigation
/// and ARIA listbox role. Touch-friendly with min 44px tap targets.
///
/// # Example
/// ```rust
/// let selected = RwSignal::new(Vec::<String>::new());
/// let options = vec!["Word of mouth", "LinkedIn", "Cold outreach"];
/// view! {
/// <ChipSelect
/// options=options
/// selected=selected
/// label="How do most leads find you?"
/// />
/// }
/// ```
#[component]
pub fn ChipSelect(
 /// Available options to choose from.
 options: Vec<&'static str>,
 /// Reactive selected values (two-way binding).
 selected: RwSignal<Vec<String>>,
 /// Label displayed above the chips.
 #[prop(optional)]
 label: Option<&'static str>,
 /// Maximum number of selections (None = unlimited).
 #[prop(optional)]
 max: Option<usize>,
 /// Disabled state.
 #[prop(optional, into)]
 disabled: Signal<bool>,
 /// Extra CSS classes on the outer wrapper.
 #[prop(default = "")]
 class: &'static str,
) -> impl IntoView {
 let toggle = move |option: String| {
  if disabled.get() {
   return;
  }
  let mut current = selected.get();
  if let Some(pos) = current.iter().position(|s| *s == option) {
   current.remove(pos);
  } else {
   if let Some(max_sel) = max {
    if current.len() >= max_sel {
     return;
    }
   }
   current.push(option);
  }
  selected.set(current);
 };

 view! {
  <div class=format!("flex flex-col gap-2 {}", class)>
   {label.map(|l| view! {
    <label class="text-sm font-medium" style="color: var(--dm-text);">
     {l}
    </label>
   })}
   <div
    role="listbox"
    aria-label=label.unwrap_or("Select options")
    aria-multiselectable="true"
    class="flex flex-wrap gap-2"
   >
    {options.into_iter().map(|opt| {
     let opt_str = opt.to_string();
     let opt_for_click = opt_str.clone();
     let opt_for_check = opt_str.clone();
     let opt_for_aria = opt_str.clone();
     view! {
      <button
       type="button"
       role="option"
       aria-selected=move || {
        if selected.get().contains(&opt_for_aria) {
         "true"
        } else {
         "false"
        }
       }
       tabindex="0"
       disabled=move || disabled.get()
       class=move || {
        let is_selected = selected.get().contains(&opt_for_check);
        format!(
         "px-3 py-1.5 rounded-full text-sm font-medium min-h-[44px] min-w-[44px] transition-all duration-150 cursor-pointer border-2 select-none focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed {}",
         if is_selected {
          "border-[var(--dm-accent)] bg-[var(--dm-accent)]/10 text-[var(--dm-accent)]"
         } else {
          "border-[var(--dm-border)] bg-[var(--dm-surface)] text-[var(--dm-text-muted)] hover:border-[var(--dm-accent)]/50 hover:text-[var(--dm-text)]"
         }
        )
       }
       on:click=move |_| toggle(opt_for_click.clone())
      >
       {opt_str}
      </button>
     }
    }).collect::<Vec<_>>()}
   </div>
  </div>
 }
}