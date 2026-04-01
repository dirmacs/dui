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
    pub key: String,
    pub label: String,
    pub sortable: bool,
    pub width: Option<String>,
}

/// A data table with sortable column headers and hover-highlighted rows.
///
/// Uses DUI CSS classes: `.dm-table`, `.dm-table-header`, `.dm-table-row`, `.dm-table-cell`, `.dm-table-sort`.
/// No Tailwind required.
#[component]
pub fn Table(
    /// Column definitions.
    columns: Vec<TableColumn>,
    /// Row data. Each inner Vec has one element per column.
    rows: Signal<Vec<Vec<String>>>,
    /// Currently sorted column key.
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
        <div class=format!("dm-table-overflow {}", class)>
            <table class="dm-table">
                <thead class="dm-table-header">
                    <tr>
                        {columns.iter().map(|col| {
                            let key = col.key.clone();
                            let key2 = col.key.clone();
                            let sortable = col.sortable;
                            let on_sort = on_sort.clone();
                            let width_style = col.width.clone().map(|w| format!("width:{}", w)).unwrap_or_default();
                            view! {
                                <th
                                    class=move || format!(
                                        "{}",
                                        if sortable { "dm-table-sort" } else { "" },
                                    )
                                    style=width_style.clone()
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
                                    {col.label.clone()}
                                    {sortable.then(|| {
                                        let indicator = move || {
                                            if sort_key.get() == key2 {
                                                match sort_dir.get() {
                                                    SortDir::Asc  => " \u{2191}",
                                                    SortDir::Desc => " \u{2193}",
                                                    SortDir::None => " \u{2195}",
                                                }
                                            } else {
                                                " \u{2195}"
                                            }
                                        };
                                        view! { <span class="dm-text-muted">{indicator}</span> }
                                    })}
                                </th>
                            }
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                <tbody>
                    {move || rows.get().iter().map(|row| {
                        view! {
                            <tr class="dm-table-row">
                                {row.iter().map(|cell| {
                                    view! { <td class="dm-table-cell">{cell.clone()}</td> }
                                }).collect::<Vec<_>>()}
                            </tr>
                        }
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
        </div>
    }
}
