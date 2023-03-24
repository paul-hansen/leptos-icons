#[cfg(feature = "VsRss")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsRss")]
/// *This icon requires the feature* `VsRss` *to be enabled*.
#[component]
pub fn Rss(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M5 13H3v-2c1.11 0 2 .89 2 2zM3 3v1a9 9 0 0 1 9 9h1C13 7.48 8.52 3 3 3zm0 4v1c2.75 0 5 2.25 5 5h1c0-3.31-2.69-6-6-6z" /></svg>
   }
}