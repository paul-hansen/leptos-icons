#[cfg(feature = "VsListTree")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsListTree")]
/// *This icon requires the feature* `VsListTree` *to be enabled*.
#[component]
pub fn ListTree(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M14 3v1H2V3h12zm-1 3v1H6V6h7zm0 3v1H5V9h8zm0 3v1H5v-1h8z" /><path d="M5 4h1v9H5z" /></svg>
   }
}