#[cfg(feature = "VsDeviceCamera")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDeviceCamera")]
/// *This icon requires the feature* `VsDeviceCamera` *to be enabled*.
#[component]
pub fn DeviceCamera(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M10.707 3H14.5l.5.5v9l-.5.5h-13l-.5-.5v-9l.5-.5h3.793l.853-.854L6.5 2h3l.354.146.853.854zM2 12h12V4h-3.5l-.354-.146L9.293 3H6.707l-.853.854L5.5 4H2v8zm1.5-7a.5.5 0 1 0 0 1 .5.5 0 0 0 0-1zM8 6a2 2 0 1 1 0 4 2 2 0 0 1 0-4zm0-1a3 3 0 1 0 0 6 3 3 0 0 0 0-6z" /></svg>
   }
}