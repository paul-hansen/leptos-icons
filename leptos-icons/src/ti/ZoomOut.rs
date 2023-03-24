#[cfg(feature = "TiZoomOut")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiZoomOut")]
/// *This icon requires the feature* `TiZoomOut` *to be enabled*.
#[component]
pub fn ZoomOut(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M13 11h-5c-.276 0-.5.224-.5.5s.224.5.5.5h5c.276 0 .5-.224.5-.5s-.224-.5-.5-.5zM19.381 15.956l-.949-.986-.537-.537-.749-.75c.227-.688.354-1.42.354-2.183 0-3.859-3.14-7-7-7s-7 3.141-7 7 3.14 7 7 7c.763 0 1.496-.127 2.184-.354l.75.749 1.512 1.51.06.061.065.055c.601.506 1.348.784 2.104.784 1.726 0 3.13-1.404 3.13-3.131 0-.84-.328-1.628-.924-2.218zm-13.881-4.456c0-2.757 2.243-5 5-5s5 2.243 5 5-2.243 5-5 5-5-2.243-5-5z" /></svg>
   }
}