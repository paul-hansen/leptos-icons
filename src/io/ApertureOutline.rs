#[cfg(feature = "IoApertureOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoApertureOutline")]
/// *This icon requires the feature* `IoApertureOutline` *to be enabled*.
#[component]
pub fn ApertureOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M448,256c0-106-86-192-192-192S64,150,64,256s86,192,192,192S448,362,448,256Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><line x1="360" y1="94.59" x2="360" y2="296" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="443.13" y1="212.87" x2="296" y2="360" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="417.41" y1="360" x2="216" y2="360" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="299.13" y1="443.13" x2="155.13" y2="299.13" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="152" y1="416" x2="152" y2="216" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="68.87" y1="299.13" x2="212.87" y2="155.13" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="94.59" y1="152" x2="288" y2="152" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="212.87" y1="68.87" x2="360" y2="216" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}