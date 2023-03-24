#[cfg(feature = "TbMessageLanguage")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMessageLanguage")]
/// *This icon requires the feature* `TbMessageLanguage` *to be enabled*.
#[component]
pub fn MessageLanguage(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-message-language" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 21v-13a3 3 0 0 1 3 -3h10a3 3 0 0 1 3 3v6a3 3 0 0 1 -3 3h-9l-4 4" /><path d="M10 14v-4a2 2 0 1 1 4 0v4" /><path d="M14 12h-4" /></svg>
   }
}