#[cfg(feature = "VsLayoutPanelOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLayoutPanelOff")]
/// *This icon requires the feature* `VsLayoutPanelOff` *to be enabled*.
#[component]
pub fn LayoutPanelOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M2 1.00073L1 2.00073V14.0007L2 15.0007H14L15 14.0007V2.00073L14 1.00073H2ZM2 10.0007V2.00073H14V10.0007H2ZM2 11.0007H14V14.0007H2V11.0007Z" /></svg>
   }
}