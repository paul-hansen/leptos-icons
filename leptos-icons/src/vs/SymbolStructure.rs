#[cfg(feature = "VsSymbolStructure")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSymbolStructure")]
/// *This icon requires the feature* `VsSymbolStructure` *to be enabled*.
#[component]
pub fn SymbolStructure(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M2 2L1 3v3l1 1h12l1-1V3l-1-1H2zm0 1h12v3H2V3zm-1 7l1-1h3l1 1v3l-1 1H2l-1-1v-3zm2 0H2v3h3v-3H3zm7 0l1-1h3l1 1v3l-1 1h-3l-1-1v-3zm2 0h-1v3h3v-3h-2z" /></svg>
   }
}