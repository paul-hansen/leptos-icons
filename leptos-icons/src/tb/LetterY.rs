#[cfg(feature = "TbLetterY")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLetterY")]
/// *This icon requires the feature* `TbLetterY` *to be enabled*.
#[component]
pub fn LetterY(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-letter-y" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 4l5 9l5 -9" /><path d="M12 13l0 7" /></svg>
   }
}