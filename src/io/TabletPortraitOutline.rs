#[cfg(feature = "IoTabletPortraitOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTabletPortraitOutline")]
/// *This icon requires the feature* `IoTabletPortraitOutline` *to be enabled*.
#[component]
pub fn TabletPortraitOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="80" y="16" width="352" height="480" rx="48" ry="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}