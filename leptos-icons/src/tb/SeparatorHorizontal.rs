#[cfg(feature = "TbSeparatorHorizontal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSeparatorHorizontal")]
/// *This icon requires the feature* `TbSeparatorHorizontal` *to be enabled*.
#[component]
pub fn SeparatorHorizontal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-separator-horizontal" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 12l16 0" /><path d="M8 8l4 -4l4 4" /><path d="M16 16l-4 4l-4 -4" /></svg>
   }
}