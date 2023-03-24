#[cfg(feature = "VsLayoutStatusbar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLayoutStatusbar")]
/// *This icon requires the feature* `VsLayoutStatusbar` *to be enabled*.
#[component]
pub fn LayoutStatusbar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M2 1.00073L1 2.00073V14.0007L2 15.0007H14L15 14.0007V2.00073L14 1.00073H2ZM2 13.0007V2.00073H14V13.0007H2Z" /></svg>
   }
}