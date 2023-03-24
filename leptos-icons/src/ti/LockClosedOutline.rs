#[cfg(feature = "TiLockClosedOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiLockClosedOutline")]
/// *This icon requires the feature* `TiLockClosedOutline` *to be enabled*.
#[component]
pub fn LockClosedOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="17" r="1.3" /><path d="M17 10h-1v-2c0-2.206-1.794-4-4-4s-4 1.794-4 4v2h-1c-1.104 0-2 .896-2 2v7c0 1.104.896 2 2 2h10c1.104 0 2-.896 2-2v-7c0-1.104-.896-2-2-2zm-7-2c0-1.104.896-2 2-2s2 .896 2 2v3h-4v-3zm7 11h-10v-7h10.003l-.003 7z" /></svg>
   }
}