#[cfg(feature = "ImEdge")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImEdge")]
/// *This icon requires the feature* `ImEdge` *to be enabled*.
#[component]
pub fn Edge(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0.241 7.103c0.469-3.7 2.994-7.056 7.519-7.103 2.731 0.053 4.978 1.291 6.316 3.65 0.672 1.231 0.881 2.525 0.925 3.953v1.678h-10.041c0.047 4.141 6.094 4 8.697 2.175v3.372c-1.525 0.916-4.984 1.734-7.662 0.681-2.281-0.856-3.906-3.244-3.897-5.541-0.075-2.978 1.481-4.95 3.897-6.072-0.513 0.634-0.903 1.334-1.106 2.547h5.669c0 0 0.331-3.388-3.209-3.388-3.338 0.116-5.744 2.056-7.106 4.047v0z" /></svg>
   }
}