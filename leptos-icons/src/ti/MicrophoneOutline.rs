#[cfg(feature = "TiMicrophoneOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiMicrophoneOutline")]
/// *This icon requires the feature* `TiMicrophoneOutline` *to be enabled*.
#[component]
pub fn MicrophoneOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><g><path d="M12 16c-2.206 0-4-1.795-4-4v-6c0-2.205 1.794-4 4-4s4 1.795 4 4v6c0 2.205-1.794 4-4 4zm0-12c-1.103 0-2 .896-2 2v6c0 1.104.897 2 2 2s2-.896 2-2v-6c0-1.104-.897-2-2-2zM19 12v-2c0-.553-.447-1-1-1s-1 .447-1 1v2c0 2.757-2.243 5-5 5s-5-2.243-5-5v-2c0-.553-.447-1-1-1s-1 .447-1 1v2c0 3.52 2.613 6.432 6 6.92v1.08h-3c-.553 0-1 .447-1 1s.447 1 1 1h8c.553 0 1-.447 1-1s-.447-1-1-1h-3v-1.08c3.387-.488 6-3.4 6-6.92z" /></g></svg>
   }
}