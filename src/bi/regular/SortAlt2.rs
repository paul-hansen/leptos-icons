#[cfg(feature = "BiRegularSortAlt2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSortAlt2")]
/// *This icon requires the feature* `BiRegularSortAlt2` *to be enabled*.
#[component]
pub fn SortAlt2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7 20h2V8h3L8 4 4 8h3zm13-4h-3V4h-2v12h-3l4 4z" /></svg>
   }
}