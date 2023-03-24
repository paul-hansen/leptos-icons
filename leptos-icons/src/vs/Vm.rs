#[cfg(feature = "VsVm")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsVm")]
/// *This icon requires the feature* `VsVm` *to be enabled*.
#[component]
pub fn Vm(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M14.5 2h-13l-.5.5v10l.5.5H7v1H4v1h8v-1H9v-1h5.5l.5-.5v-10l-.5-.5zM14 12H2V3h12v9z" /></svg>
   }
}