#[cfg(feature = "TiChartLine")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiChartLine")]
/// *This icon requires the feature* `TiChartLine` *to be enabled*.
#[component]
pub fn ChartLine(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M4.75 15.561c.369.294.811.439 1.248.439.588 0 1.168-.258 1.563-.752l2.789-3.486 2.45 1.838c.864.648 2.088.492 2.762-.352l4-5c.69-.861.55-2.121-.312-2.811-.863-.689-2.121-.551-2.812.312l-2.789 3.486-2.449-1.835c-.864-.648-2.087-.494-2.762.35l-4 5c-.69.863-.549 2.121.312 2.811zM5 21h14c.553 0 1-.447 1-1s-.447-1-1-1h-14c-.553 0-1 .447-1 1s.447 1 1 1z" /></svg>
   }
}