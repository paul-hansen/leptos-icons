#[cfg(feature = "TbRowInsertTop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbRowInsertTop")]
/// *This icon requires the feature* `TbRowInsertTop` *to be enabled*.
#[component]
pub fn RowInsertTop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-row-insert-top" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 18v-4a1 1 0 0 1 1 -1h14a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-14a1 1 0 0 1 -1 -1z" /><path d="M12 9v-4" /><path d="M10 7l4 0" /></svg>
   }
}