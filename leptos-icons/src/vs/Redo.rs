#[cfg(feature = "VsRedo")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsRedo")]
/// *This icon requires the feature* `VsRedo` *to be enabled*.
#[component]
pub fn Redo(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M12.5 2v3.5L12 6H8.5V5h2.521l-.941-.941a3.552 3.552 0 1 0-5.023 5.023l5.197 5.198-.72.72-5.198-5.198A4.57 4.57 0 0 1 10.8 3.339l.7.7V2h1z" /></svg>
   }
}