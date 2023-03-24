#[cfg(feature = "VsMultipleWindows")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsMultipleWindows")]
/// *This icon requires the feature* `VsMultipleWindows` *to be enabled*.
#[component]
pub fn MultipleWindows(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M6 1.5l.5-.5h8l.5.5v7l-.5.5H12V8h2V4H7v1H6V1.5zM7 2v1h7V2H7zM1.5 7l-.5.5v7l.5.5h8l.5-.5v-7L9.5 7h-8zM2 9V8h7v1H2zm0 1h7v4H2v-4z" /></svg>
   }
}