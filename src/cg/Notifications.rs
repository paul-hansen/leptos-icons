#[cfg(feature = "CgNotifications")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgNotifications")]
/// *This icon requires the feature* `CgNotifications` *to be enabled*.
#[component]
pub fn Notifications(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M20 7C20 8.65685 18.6569 10 17 10C15.3431 10 14 8.65685 14 7C14 5.34315 15.3431 4 17 4C18.6569 4 20 5.34315 20 7Z" fill="currentColor" /><path d="M12 6H4V20H18V12H16V18H6V8H12V6Z" fill="currentColor" /></svg>
   }
}