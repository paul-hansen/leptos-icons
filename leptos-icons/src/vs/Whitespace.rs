#[cfg(feature = "VsWhitespace")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsWhitespace")]
/// *This icon requires the feature* `VsWhitespace` *to be enabled*.
#[component]
pub fn Whitespace(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M12 2V1H6.5a3.5 3.5 0 0 0 0 7H8v5H7v1h5v-1h-1V2h1zM8 7H6.5a2.5 2.5 0 1 1 0-5H8v5zm2 6H9V2h1v11z" /></svg>
   }
}