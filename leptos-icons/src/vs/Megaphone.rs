#[cfg(feature = "VsMegaphone")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsMegaphone")]
/// *This icon requires the feature* `VsMegaphone` *to be enabled*.
#[component]
pub fn Megaphone(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M2 6.77l12.33-3.43.67.53v8.6l-.67.53-6.089-1.595a2.16 2.16 0 1 1-4.178-1.095L2 9.77l-.42-.53V7.3L2 6.77zm3.006 3.787a1.13 1.13 0 0 0-.04.242 1.17 1.17 0 0 0 2.288.347l-2.248-.589zM2.58 8.82L14 11.83V4.5L2.58 7.72v1.1z" /></svg>
   }
}