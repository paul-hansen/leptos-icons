#[cfg(feature = "BiRegularExclude")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularExclude")]
/// *This icon requires the feature* `BiRegularExclude` *to be enabled*.
#[component]
pub fn Exclude(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 8h-3V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v9c0 1.103.897 2 2 2h3v3c0 1.103.897 2 2 2h9c1.103 0 2-.897 2-2v-9c0-1.103-.897-2-2-2zm-4 7H9V9h6v6z" /></svg>
   }
}