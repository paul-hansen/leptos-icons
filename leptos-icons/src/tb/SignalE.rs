#[cfg(feature = "TbSignalE")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSignalE")]
/// *This icon requires the feature* `TbSignalE` *to be enabled*.
#[component]
pub fn SignalE(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-signal-e" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M14 8h-4v8h4" /><path d="M10 12h2.5" /></svg>
   }
}