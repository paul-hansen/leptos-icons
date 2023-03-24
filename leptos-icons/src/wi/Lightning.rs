#[cfg(feature = "WiLightning")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "WiLightning")]
/// *This icon requires the feature* `WiLightning` *to be enabled*.
#[component]
pub fn Lightning(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" id="Layer_1" x="0px" y="0px" viewBox="0 0 30 30" style="enable-background:new 0 0 30 30;" xml:space="preserve"><path d="M7.96,24.51h0.39l6.88-10.18c0.09-0.18,0.04-0.27-0.15-0.27h-2.84l2.99-5.45c0.09-0.18,0.02-0.27-0.2-0.27h-3.81&#xD;&#xA;	c-0.11,0-0.2,0.06-0.29,0.18l-2.78,7.4c-0.02,0.18,0.04,0.27,0.19,0.27h2.75L7.96,24.51z M16.46,18.18h0.27l5.22-7.67&#xD;&#xA;	c0.05-0.08,0.06-0.15,0.04-0.2s-0.08-0.07-0.17-0.07h-2.1l2.18-4.03c0.12-0.2,0.06-0.3-0.18-0.3h-2.74c-0.13,0-0.23,0.06-0.3,0.19&#xD;&#xA;	l-2.08,5.48c-0.03,0.09-0.03,0.16,0.01,0.21c0.04,0.05,0.1,0.07,0.19,0.07h2.04L16.46,18.18z" /></svg>
   }
}