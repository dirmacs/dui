//! Table — headers, rows, hover highlight, optional sortable columns.

use leptos::prelude::*;

/// Sort direction for a column.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SortDir {
    #[default]
    Asc,
    Desc,
    None,
}

/// A column definition.
#[derive(Debug, Clone)]
pub struct TableColumn {
    /// Unique key.
    pub key: String,
    /// Display header text.
    pub label: String,
    /// Whether the column is sortable.
    pub sortable: bool,
    /// Optional width class (e.g. "w-48", "w-1/3").
    pub width: Option<String>,
}

/// A data table with sortable column headers and hover-highlighted rows.
///
/// Rows are passed as `Vec<Vec<View>>` — each inner vec maps 1:1 to columns.
/// Sorting is signal-based: when a sortable header is clicked, `on_sort` fires
/// with the column key and new direction. The consumer re-sorts their data.
#[component]
pub fn Table(
    /// Column definitions.
    columns: Vec<TableColumn>,
    /// Row data. Each inner Vec has one element per column.
    rows: Signal<Vec<Vec<String>>>,
    /// Currently sorted column key (empty = no sort).
    #[prop(optional, into)]
    sort_key: RwSignal<String>,
    /// Current sort direction.
    #[prop(optional, into)]
    sort_dir: RwSignal<SortDir>,
    /// Called when a sortable column header is clicked.
    #[prop(optional)]
    on_sort: Option<Box<dyn Fn(String, SortDir)>>,
    /// Extra classes on the table wrapper.
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    let on_sort = std::rc::Rc::new(on_sort);

    view! {
        <div class=format!("w-full overflow-x-auto rounded-lg border border-dm {}", class)>
            <table class="w-full text-sm text-left">
                // Header
                <thead class="bg-dm-elevated">
                    <tr>
                        {columns.iter().map(|col| {
                            let key = col.key.clone();
                            let key2 = col.key.clone();
                            let sortable = col.sortable;
                            let on_sort = on_sort.clone();
                            let width = col.width.clone().unwrap_or_default();
                            view! {
                                <th
                                    class=move || format!(
                                        "px-4 py-3 text-xs font-semibold uppercase tracking-wider \
                                         text-dm-muted border-b border-dm {} {}",
                                        width,
                                        if sortable { "cursor-pointer select-none hover:text-dm-text transition-colors" } else { "" },
                                    )
                                    on:click={
                                        let key = key.clone();
                                        let on_sort = on_sort.clone();
                                        move |_| {
                                            if sortable {
                                                let new_dir = if sort_key.get() == key {
                                                    match sort_dir.get() {
                                                        SortDir::Asc  => SortDir::Desc,
                                                        SortDir::Desc => SortDir::None,
                                                        SortDir::None => SortDir::Asc,
                                                    }
                                                } else {
                                                    SortDir::Asc
                                                };
                                                sort_key.set(key.clone());
                                                sort_dir.set(new_dir);
                                                if let Some(ref cb) = *on_sort {
                                                    cb(key.clone(), new_dir);
                                                }
                                            }
                                        }
                                    }
                                >
                                    <span class="flex items-center gap-1.5">
                                        {col.label.clone()}
                                        {sortable.then(|| {
                                            let key2 = key2.clone();
                                            let key2_for_compare = key2.clone();
                                            view! {
                                                <span class=move || {
                                                    if sort_key.get() == key2_for_compare {
                                                        "text-dm-accent"
                                                    } else {
                                                        "text-dm-dim"
                                                    }
                                                }>
                                                    {move || {
                                                        let k = key2.clone();
                                                        if sort_key.get() == k {
                                                            match sort_dir.get() {
                                                                SortDir::Asc  => "\u{2191}",
                                                                SortDir::Desc => "\u{2193}",
                                                                SortDir::None => "\u{2195}",
                                                            }
                                                        } else {
                                                            "\u{2195}"
                                                        }
                                                    }}
                                                </span>
                                            }
                                        })}
                                    </span>
                                </th>
                            }
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>

                // Body
                <tbody>
                    {move || rows.get().into_iter().enumerate().map(|(row_idx, cells)| {
                        view! {
                            <tr class=format!(
                                "border-b border-dm last:border-0 \
                                 hover:bg-dm-hover/50 transition-colors duration-100 {}",
                                if row_idx % 2 == 1 { "bg-dm-bg/30" } else { "" }
                            )>
                                {cells.into_iter().map(|cell| {
                                    view! {
                                        <td class="px-4 py-3 text-dm-text">{cell}</td>
                                    }
                                }).collect::<Vec<_>>()}
                            </tr>
                        }
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
        </div>
    }
}
