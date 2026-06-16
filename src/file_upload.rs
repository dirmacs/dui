//! FileUpload — drag-and-drop file upload with file list.

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
/// Uses DUI CSS classes + inline styles. No Tailwind required.
#[component]
pub fn FileUpload(
    /// Accepted file types.
    #[prop(default = "")]
    accept: &'static str,
    /// Allow multiple files.
    #[prop(default = false)]
    multiple: bool,
    /// Callback when files are selected.
    on_upload: Box<dyn Fn(Vec<UploadedFile>) + 'static>,
    /// Reactive file list for display.
    files: RwSignal<Vec<UploadedFile>>,
    /// Label for the upload zone.
    #[prop(optional)]
    label: Option<&'static str>,
    /// Description text.
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
        for i in 0..file_list.length() {
            if let Some(file) = file_list.item(i) {
                uploaded.push(UploadedFile {
                    name: file.name(),
                    size: file.size() as u64,
                    mime_type: file.type_(),
                });
            }
        }
        uploaded
    };

    view! {
        <div class=format!("dm-flex dm-flex-col dm-gap-2 {}", class)>
            {label.map(|l| view! { <label class="dm-input-label">{l}</label> })}

            <div
                class=move || format!(
                    "dm-card dm-text-center dm-cursor-pointer dm-transition {}",
                    if dragging.get() { "dm-border-accent" } else { "" },
                )
                style="padding:2rem;border-style:dashed"
                on:click=move |_| {
                    if disabled.get() { return; }
                    let doc = web_sys::window().unwrap().document().unwrap();
                    let input: web_sys::HtmlInputElement = doc.create_element("input").unwrap().unchecked_into();
                    input.set_type("file");
                    if !accept.is_empty() { input.set_accept(accept); }
                    if multiple { input.set_multiple(true); }
                    let on_upload = on_upload_click.clone();
                    let files_sig = files;
                    let cb = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::Event)>::new(move |ev: web_sys::Event| {
                        let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                        if let Some(file_list) = target.files() {
                            let mut uploaded = Vec::new();
                            for i in 0..file_list.length() {
                                if let Some(file) = file_list.item(i) {
                                    uploaded.push(UploadedFile { name: file.name(), size: file.size() as u64, mime_type: file.type_() });
                                }
                            }
                            files_sig.update(|v| v.extend(uploaded.clone()));
                            on_upload(uploaded);
                        }
                    });
                    input.set_onchange(Some(cb.as_ref().unchecked_ref()));
                    cb.forget();
                    input.click();
                }
                on:dragover=move |ev: web_sys::DragEvent| { ev.prevent_default(); dragging.set(true); }
                on:dragleave=move |_| dragging.set(false)
                on:drop=move |ev: web_sys::DragEvent| {
                    ev.prevent_default();
                    dragging.set(false);
                    if disabled.get() { return; }
                    if let Some(dt) = ev.data_transfer() {
                        if let Some(file_list) = dt.files() {
                            let uploaded = handle_files(file_list);
                            files.update(|v| v.extend(uploaded.clone()));
                            on_upload(uploaded);
                        }
                    }
                }
            >
                <div class="dm-text-muted dm-text-sm">"Drop files here or click to browse"</div>
                {description.map(|d| view! { <div class="dm-text-xs dm-text-dim dm-mt-1">{d}</div> })}
            </div>

            // File list
            {move || {
                let current_files = files.get();
                if current_files.is_empty() { return view! { <div></div> }.into_any(); }
                view! {
                    <div class="dm-flex dm-flex-col dm-gap-1">
                        {current_files.iter().map(|f| {
                            let name = f.name.clone();
                            let size_kb = f.size / 1024;
                            view! {
                                <div class="dm-flex dm-items-center dm-justify-between dm-p-2 dm-rounded dm-bg-surface">
                                    <span class="dm-text-sm dm-text-primary">{name}</span>
                                    <span class="dm-text-xs dm-text-muted">{format!("{}KB", size_kb)}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_any()
            }}
        </div>
    }
}
