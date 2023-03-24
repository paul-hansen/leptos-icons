#[cfg(feature = "VsChevronLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsChevronLeft")]
/// *This icon requires the feature* `VsChevronLeft` *to be enabled*.
#[component]
pub fn ChevronLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M5.928 7.976l4.357 4.357-.618.62L5 8.284v-.618L9.667 3l.618.619-4.357 4.357z" /></svg>
   }
}