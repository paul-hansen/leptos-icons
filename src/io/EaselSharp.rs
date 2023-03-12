#[cfg(feature = "IoEaselSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoEaselSharp")]
/// *This icon requires the feature* `IoEaselSharp` *to be enabled*.
#[component]
pub fn EaselSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M468,64H278V32H234V64H44A12,12,0,0,0,32,76V356a12,12,0,0,0,12,12h78.19L89.93,470.46l36.53,9.61L161.74,368H234v64h44V368h71.84l31,111.7,36.83-8.57L389.05,368H468a12,12,0,0,0,12-12V76A12,12,0,0,0,468,64ZM442,330H70V102H442Z" /><rect x="88" y="120" width="336" height="192" /></svg>
   }
}