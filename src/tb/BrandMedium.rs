#[cfg(feature = "TbBrandMedium")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandMedium")]
/// *This icon requires the feature* `TbBrandMedium` *to be enabled*.
#[component]
pub fn BrandMedium(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-medium" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z" /><path d="M8 9h1l3 3l3 -3h1" /><path d="M8 15l2 0" /><path d="M14 15l2 0" /><path d="M9 9l0 6" /><path d="M15 9l0 6" /></svg>
   }
}