#[cfg(feature = "Tb24Hours")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "Tb24Hours")]
/// *This icon requires the feature* `Tb24Hours` *to be enabled*.
#[component]
pub fn _24Hours(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-24-hours" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -4v4h4" /><path d="M4 13a8.094 8.094 0 0 0 3 5.24" /><path d="M11 15h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /><path d="M17 15v2a1 1 0 0 0 1 1h1" /><path d="M20 15v6" /></svg>
   }
}