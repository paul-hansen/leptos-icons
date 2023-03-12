#[cfg(feature = "OcLgSkipFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgSkipFill")]
/// *This icon requires the feature* `OcLgSkipFill` *to be enabled*.
#[component]
pub fn SkipFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M1 12C1 5.925 5.925 1 12 1s11 4.925 11 11-4.925 11-11 11S1 18.075 1 12Zm16.333-4.167a.825.825 0 0 0-1.166-1.166l-9.5 9.5a.825.825 0 0 0 1.166 1.166Z" /></svg>
   }
}