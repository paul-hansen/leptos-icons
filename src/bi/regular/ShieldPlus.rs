#[cfg(feature = "BiRegularShieldPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularShieldPlus")]
/// *This icon requires the feature* `BiRegularShieldPlus` *to be enabled*.
#[component]
pub fn ShieldPlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11.63 21.91A.9.9 0 0 0 12 22a1 1 0 0 0 .41-.09C22 17.67 21 7 21 6.9a1 1 0 0 0-.55-.79l-8-4a1 1 0 0 0-.9 0l-8 4A1 1 0 0 0 3 6.9c0 .1-.92 10.77 8.63 15.01zM5 7.63l7-3.51 7 3.51c.05 2-.27 9-7 12.27C5.26 16.63 4.94 9.64 5 7.63z" /><path d="M11.06 16h2v-3h3.01v-2h-3.01V8h-2v3h-3v2h3v3z" /></svg>
   }
}