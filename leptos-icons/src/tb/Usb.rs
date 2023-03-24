#[cfg(feature = "TbUsb")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbUsb")]
/// *This icon requires the feature* `TbUsb` *to be enabled*.
#[component]
pub fn Usb(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-usb" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 19m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><path d="M12 17v-11.5" /><path d="M7 10v3l5 3" /><path d="M12 14.5l5 -2v-2.5" /><path d="M16 10h2v-2h-2z" /><path d="M7 9m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /><path d="M10 5.5h4l-2 -2.5z" /></svg>
   }
}