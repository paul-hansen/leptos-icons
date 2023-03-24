#[cfg(feature = "VsLayoutMenubar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLayoutMenubar")]
/// *This icon requires the feature* `VsLayoutMenubar` *to be enabled*.
#[component]
pub fn LayoutMenubar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M6 3H3V4H6V3Z" /><path d="M3 5H6V6H3V5Z" /><path d="M6 7H3V8H6V7Z" /><path d="M2 1L1 2V14L2 15H14L15 14V2L14 1H2ZM2 14V2H14V14H2Z" /></svg>
   }
}