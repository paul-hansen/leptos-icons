#[cfg(feature = "VsServer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsServer")]
/// *This icon requires the feature* `VsServer` *to be enabled*.
#[component]
pub fn Server(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M2.5 5L2 4.5v-3l.5-.5h11l.5.5v3l-.5.5h-11zM10 2H9v1H8V2H7v1H6V2H5v1H4V2H3v2h10V2h-2v1h-1V2zm-7.5 8L2 9.5v-3l.5-.5h11l.5.5v3l-.5.5h-11zM6 7H5v1H4V7H3v2h10V7h-2v1h-1V7H9v1H8V7H7v1H6V7zm7.5 8l.5-.5v-3l-.5-.5h-11l-.5.5v3l.5.5h11zM3 14v-2h1v1h1v-1h1v1h1v-1h1v1h1v-1h1v1h1v-1h2v2H3z" /></svg>
   }
}