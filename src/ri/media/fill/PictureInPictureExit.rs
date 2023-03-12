#[cfg(feature = "RiMediaFillPictureInPictureExit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaFillPictureInPictureExit")]
/// *This icon requires the feature* `RiMediaFillPictureInPictureExit` *to be enabled*.
#[component]
pub fn PictureInPictureExit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path fill-rule="nonzero" d="M21 3a1 1 0 0 1 1 1v7h-2V5H4v14h6v2H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h18zm0 10a1 1 0 0 1 1 1v6a1 1 0 0 1-1 1h-8a1 1 0 0 1-1-1v-6a1 1 0 0 1 1-1h8zm-9.5-6L9.457 9.043l2.25 2.25-1.414 1.414-2.25-2.25L6 12.5V7h5.5z" /></g></svg>
   }
}