#[cfg(feature = "RiSystemFillHistory")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillHistory")]
/// *This icon requires the feature* `RiSystemFillHistory` *to be enabled*.
#[component]
pub fn History(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12h2c0 4.418 3.582 8 8 8s8-3.582 8-8-3.582-8-8-8C9.536 4 7.332 5.114 5.865 6.865L8 9H2V3l2.447 2.446C6.28 3.336 8.984 2 12 2zm1 5v4.585l3.243 3.243-1.415 1.415L11 12.413V7h2z" /></g></svg>
   }
}