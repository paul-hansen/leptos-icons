#[cfg(feature = "VsDebugStackframeActive")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugStackframeActive")]
/// *This icon requires the feature* `VsDebugStackframeActive` *to be enabled*.
#[component]
pub fn DebugStackframeActive(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"><path d="M10 8a2 2 0 1 1-4 0 2 2 0 0 1 4 0z" /><path d="M14.5 7.15l-4.26-4.74L9.31 2H4.25L3 3.25v9.48l1.25 1.25h5.06l.93-.42 4.26-4.74V7.15zm-5.19 5.58H4.25V3.25h5.06l4.26 4.73-4.26 4.75z" /></svg>
   }
}