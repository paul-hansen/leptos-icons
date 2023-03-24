#[cfg(feature = "TbSpeedboat")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSpeedboat")]
/// *This icon requires the feature* `TbSpeedboat` *to be enabled*.
#[component]
pub fn Speedboat(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-speedboat" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 17h13.4a3 3 0 0 0 2.5 -1.34l3.1 -4.66h0h-6.23a4 4 0 0 0 -1.49 .29l-3.56 1.42a4 4 0 0 1 -1.49 .29h-3.73h0h-1l-1.5 4z" /><path d="M6 13l1.5 -5" /><path d="M6 8h8l2 3" /></svg>
   }
}