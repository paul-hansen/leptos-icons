#[cfg(feature = "VsListFilter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsListFilter")]
/// *This icon requires the feature* `VsListFilter` *to be enabled*.
#[component]
pub fn ListFilter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M6 12v-1h4v1H6zM4 7h8v1H4V7zm10-4v1H2V3h12z" /></svg>
   }
}