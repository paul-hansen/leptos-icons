#[cfg(feature = "BiSolidFlagAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFlagAlt")]
/// *This icon requires the feature* `BiSolidFlagAlt` *to be enabled*.
#[component]
pub fn FlagAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m14.303 6-3-2H6V2H4v20h2v-8h4.697l3 2H20V6z" /></svg>
   }
}