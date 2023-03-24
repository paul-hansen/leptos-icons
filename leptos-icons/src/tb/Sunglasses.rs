#[cfg(feature = "TbSunglasses")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSunglasses")]
/// *This icon requires the feature* `TbSunglasses` *to be enabled*.
#[component]
pub fn Sunglasses(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-sunglasses" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M8 4h-2l-3 10" /><path d="M16 4h2l3 10" /><path d="M10 16h4" /><path d="M21 16.5a3.5 3.5 0 0 1 -7 0v-2.5h7v2.5" /><path d="M10 16.5a3.5 3.5 0 0 1 -7 0v-2.5h7v2.5" /><path d="M4 14l4.5 4.5" /><path d="M15 14l4.5 4.5" /></svg>
   }
}