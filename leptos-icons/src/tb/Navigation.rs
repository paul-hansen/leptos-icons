#[cfg(feature = "TbNavigation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbNavigation")]
/// *This icon requires the feature* `TbNavigation` *to be enabled*.
#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-navigation" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 18.5l7.265 2.463a.535 .535 0 0 0 .57 -.116a.548 .548 0 0 0 .134 -.572l-7.969 -17.275l-7.97 17.275a.547 .547 0 0 0 .135 .572a.535 .535 0 0 0 .57 .116l7.265 -2.463" /></svg>
   }
}