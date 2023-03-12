#[cfg(feature = "BiSolidDiamond")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDiamond")]
/// *This icon requires the feature* `BiSolidDiamond` *to be enabled*.
#[component]
pub fn Diamond(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16.445 3h-8.89c-.345 0-.666.178-.849.47L3.25 9h17.5l-3.456-5.53a1.003 1.003 0 0 0-.849-.47zM11.26 21.186a1 1 0 0 0 1.48 0L22 11H2l9.26 10.186z" /></svg>
   }
}