#[cfg(feature = "VsChromeClose")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsChromeClose")]
/// *This icon requires the feature* `VsChromeClose` *to be enabled*.
#[component]
pub fn ChromeClose(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M7.116 8l-4.558 4.558.884.884L8 8.884l4.558 4.558.884-.884L8.884 8l4.558-4.558-.884-.884L8 7.116 3.442 2.558l-.884.884L7.116 8z" /></svg>
   }
}