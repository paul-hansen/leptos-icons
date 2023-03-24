#[cfg(feature = "VsSymbolConstant")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSymbolConstant")]
/// *This icon requires the feature* `VsSymbolConstant` *to be enabled*.
#[component]
pub fn SymbolConstant(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M4 6h8v1H4V6zm8 3H4v1h8V9z" /><path fill-rule="evenodd" clip-rule="evenodd" d="M1 4l1-1h12l1 1v8l-1 1H2l-1-1V4zm1 0v8h12V4H2z" /></svg>
   }
}