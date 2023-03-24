#[cfg(feature = "TbSignalHPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSignalHPlus")]
/// *This icon requires the feature* `TbSignalHPlus` *to be enabled*.
#[component]
pub fn SignalHPlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-signal-h-plus" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 16v-8" /><path d="M11 8v8" /><path d="M7 12h4" /><path d="M14 12h4" /><path d="M16 10v4" /></svg>
   }
}