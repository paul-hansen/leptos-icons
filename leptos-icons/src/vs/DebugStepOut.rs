#[cfg(feature = "VsDebugStepOut")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugStepOut")]
/// *This icon requires the feature* `VsDebugStepOut` *to be enabled*.
#[component]
pub fn DebugStepOut(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M8 1h-.542L3.553 4.905l1.061 1.06 2.637-2.61v6.177h1.498V3.355l2.637 2.61 1.061-1.06L8.542 1H8zm1.956 12.013a2 2 0 1 1-4 0 2 2 0 0 1 4 0z" /></svg>
   }
}