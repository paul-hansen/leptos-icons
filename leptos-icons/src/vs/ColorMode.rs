#[cfg(feature = "VsColorMode")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsColorMode")]
/// *This icon requires the feature* `VsColorMode` *to be enabled*.
#[component]
pub fn ColorMode(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1zm0 13V2a6 6 0 1 1 0 12z" /></svg>
   }
}