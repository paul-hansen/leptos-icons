#[cfg(feature = "TbSwitchVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSwitchVertical")]
/// *This icon requires the feature* `TbSwitchVertical` *to be enabled*.
#[component]
pub fn SwitchVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-switch-vertical" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 8l4 -4l4 4" /><path d="M7 4l0 9" /><path d="M13 16l4 4l4 -4" /><path d="M17 10l0 10" /></svg>
   }
}