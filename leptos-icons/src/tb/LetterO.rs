#[cfg(feature = "TbLetterO")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLetterO")]
/// *This icon requires the feature* `TbLetterO` *to be enabled*.
#[component]
pub fn LetterO(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-letter-o" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M18 9a5 5 0 0 0 -5 -5h-2a5 5 0 0 0 -5 5v6a5 5 0 0 0 5 5h2a5 5 0 0 0 5 -5v-6" /></svg>
   }
}