#[cfg(feature = "BiSolidCommentEdit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCommentEdit")]
/// *This icon requires the feature* `BiSolidCommentEdit` *to be enabled*.
#[component]
pub fn CommentEdit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .897-2 2v18l4-4h14c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zM8.999 14.987H7v-1.999l5.53-5.522 1.998 1.999-5.529 5.522zm6.472-6.464-1.999-1.999 1.524-1.523L16.995 7l-1.524 1.523z" /></svg>
   }
}