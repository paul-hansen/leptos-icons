#[cfg(feature = "VsSaveAll")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSaveAll")]
/// *This icon requires the feature* `VsSaveAll` *to be enabled*.
#[component]
pub fn SaveAll(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M14.85 2.65l-1.5-1.5L13 1H4.48l-.5.5V4H1.5l-.5.5v10l.5.5h10l.5-.5V12h2.5l.5-.5V3l-.15-.35zM11 14H2V5h1v3.07h6V5h.79L11 6.21V14zM6 7V5h2v2H6zm8 4h-2V6l-.15-.35-1.5-1.5L10 4H5V2h7.81l1.21 1.21L14 11z" /></svg>
   }
}