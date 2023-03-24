#[cfg(feature = "TbZodiacPisces")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbZodiacPisces")]
/// *This icon requires the feature* `TbZodiacPisces` *to be enabled*.
#[component]
pub fn ZodiacPisces(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-zodiac-pisces" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M5 3a21 21 0 0 1 0 18" /><path d="M19 3a21 21 0 0 0 0 18" /><path d="M5 12l14 0" /></svg>
   }
}