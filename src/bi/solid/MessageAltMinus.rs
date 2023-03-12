#[cfg(feature = "BiSolidMessageAltMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageAltMinus")]
/// *This icon requires the feature* `BiSolidMessageAltMinus` *to be enabled*.
#[component]
pub fn MessageAltMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 2c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h3.5l3.5 4 3.5-4H19c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2H5zm11 9H8V9h8v2z" /></svg>
   }
}