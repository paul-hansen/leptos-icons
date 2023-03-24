#[cfg(feature = "TbSendOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSendOff")]
/// *This icon requires the feature* `TbSendOff` *to be enabled*.
#[component]
pub fn SendOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-send-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M10 14l2 -2m2 -2l7 -7" /><path d="M10.718 6.713l10.282 -3.713l-3.715 10.289m-1.063 2.941l-1.722 4.77a.55 .55 0 0 1 -1 0l-3.5 -7l-7 -3.5a.55 .55 0 0 1 0 -1l4.772 -1.723" /><path d="M3 3l18 18" /></svg>
   }
}