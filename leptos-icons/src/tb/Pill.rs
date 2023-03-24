#[cfg(feature = "TbPill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbPill")]
/// *This icon requires the feature* `TbPill` *to be enabled*.
#[component]
pub fn Pill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-pill" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4.5 12.5l8 -8a4.94 4.94 0 0 1 7 7l-8 8a4.94 4.94 0 0 1 -7 -7" /><path d="M8.5 8.5l7 7" /></svg>
   }
}