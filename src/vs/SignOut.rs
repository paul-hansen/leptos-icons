#[cfg(feature = "VsSignOut")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSignOut")]
/// *This icon requires the feature* `VsSignOut` *to be enabled*.
#[component]
pub fn SignOut(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M11.02 3.77v1.56l1-.99V2.5l-.5-.5h-9l-.5.5v.486L2 3v10.29l.36.46 5 1.72L8 15v-1h3.52l.5-.5v-1.81l-1-1V13H8V4.71l-.33-.46L4.036 3h6.984v.77zM7 14.28l-4-1.34V3.72l4 1.34v9.22zm6.52-5.8H8.55v-1h4.93l-1.6-1.6.71-.7 2.47 2.46v.71l-2.49 2.48-.7-.7 1.65-1.65z" /></svg>
   }
}