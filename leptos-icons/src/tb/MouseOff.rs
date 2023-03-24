#[cfg(feature = "TbMouseOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMouseOff")]
/// *This icon requires the feature* `TbMouseOff` *to be enabled*.
#[component]
pub fn MouseOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-mouse-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7.733 3.704a3.982 3.982 0 0 1 2.267 -.704h4a4 4 0 0 1 4 4v7m-.1 3.895a4 4 0 0 1 -3.9 3.105h-4a4 4 0 0 1 -4 -4v-10c0 -.3 .033 -.593 .096 -.874" /><path d="M12 7v1" /><path d="M3 3l18 18" /></svg>
   }
}