use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Represents an uploaded file.
#[derive(Clone, Debug)]
pub struct UploadedFile {
 pub name: String,
 pub size: u64,
 pub mime_type: String,
}

/// Drag-and-drop file upload component.
///
/// Shows a drop zone and file list with remove buttons.
/// Touch-friendly: full-width, tap-to-upload on mobile.
///
/// # Example
/// ```rust
/// let files = RwSignal::new(Vec::<UploadedFile>::new());
/// view! {
/// <FileUpload
/// accept=".pdf,.docx,.txt"
/// multiple=true
/// on_upload=move |f: Vec<UploadedFile>| { files.update(|v| v.extend(f)); }
/// files=files
/// label="Upload business documents"
/// />
/// }
/// ```
#[component]
pub fn FileUpload(
 /// Comma-separated accepted file types.
 #[prop(default = "")]
 accept: &'static str,
 /// Allow multiple files.
 #[prop(default = false)]
 multiple: bool,
 /// Callback when files are selected (receives file metadata).
 on_upload: Box<dyn Fn(Vec<UploadedFile>) + 'static>,
 /// Reactive file list for display.
 files: RwSignal<Vec<UploadedFile>>,
 /// Label for the upload zone.
 #[prop(optional)]
 label: Option<&'static str>,
 /// Description text below the label.
 #[prop(optional)]
 description: Option<&'static str>,
 /// Disabled state.
 #[prop(optional, into)]
 disabled: Signal<bool>,
 /// Extra CSS classes.
 #[prop(default = "")]
 class: &'static str,
) -> impl IntoView {
 let dragging = RwSignal::new(false);
 let on_upload = std::rc::Rc::new(on_upload);
 let on_upload_click = on_upload.clone();
 let handle_files = move |file_list: web_sys::FileList| {
  let mut uploaded = Vec::new();
  let length = file_list.length();
  for i in 0..length {
   // item() returns Option<File> directly
   if let Some(file) = file_list.item(i) {
    uploaded.push(UploadedFile {
     name: file.name(),
     size: file.size() as u64,
     mime_type: file.type_(),
    });
   }
  }
  if !uploaded.is_empty() {
   (on_upload_click)(uploaded);
  }
 };
 let handle_files_drop = handle_files.clone();
 view! {
  <div class=format!("flex flex-col gap-2 {}", class)>
   {label.map(|l| view! {
    <label class="text-sm font-medium" style="color: var(--dm-text);">
     {l}
    </label>
   })}
   {description.map(|d| view! {
    <p class="text-xs" style="color: var(--dm-text-muted);">{d}</p>
   })}
   <div class=move || format!(
    "relative flex flex-col items-center justify-center min-h-[120px] rounded-lg border-2 border-dashed p-4 transition-colors cursor-pointer {}",
    if dragging.get() { "border-[var(--dm-accent)] bg-[var(--dm-accent)]/5" } else { "border-[var(--dm-border)] hover:border-[var(--dm-accent)]/50" }
   )
   on:dragover=move |ev: web_sys::DragEvent| {
    ev.prevent_default();
    dragging.set(true);
   }
   on:dragleave=move |_| dragging.set(false)
   on:drop=move |ev: web_sys::DragEvent| {
    ev.prevent_default();
    dragging.set(false);
    if let Some(dt) = ev.data_transfer() {
     if let Some(file_list) = dt.files() {
      handle_files_drop(file_list);
     }
    }
   }
   on:click=move |_| {
    // Trigger hidden input click
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
     if let Some(el) = doc.get_element_by_id("dui-file-input") {
      let _ = el.dyn_ref::<web_sys::HtmlElement>().map(|e| e.click());
     }
    }
   }
  >
   <input id="dui-file-input" type="file" accept=accept multiple=multiple class="hidden" disabled=move || disabled.get() on:change=move |ev: web_sys::Event| {
    use wasm_bindgen::JsCast;
    if let Some(input) = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
     if let Some(file_list) = input.files() {
      handle_files(file_list);
     }
    }
   } />
   <span class="text-sm" style="color: var(--dm-text-muted);">
    "Drop files here or click to browse"
   </span>
  </div>
  // File list
  <div class="flex flex-col gap-1">
   {move || files.get().iter().enumerate().map(|(i, f)| {
    let name = f.name.clone();
    let size_kb = f.size / 1024;
    view! {
     <div class="flex items-center justify-between px-3 py-1.5 rounded text-sm" style="background: var(--dm-surface); color: var(--dm-text);">
      <span>{format!("{} ({}KB)", name, size_kb)}</span>
      <button type="button" class="text-xs px-2 py-0.5 rounded hover:bg-[var(--dm-danger)]/20" style="color: var(--dm-danger);" on:click=move |_| {
       files.update(|v| { v.remove(i); });
      } >
       "Remove"
      </button>
     </div>
    }
   }).collect::<Vec<_>>()}
  </div>
  </div>
 }
}