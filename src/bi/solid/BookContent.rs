#[cfg(feature = "BiSolidBookContent")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBookContent")]
/// *This icon requires the feature* `BiSolidBookContent` *to be enabled*.
#[component]
pub fn BookContent(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 3H4c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zm-1 4v2h-5V7h5zm-5 4h5v2h-5v-2zM4 19V5h7v14H4z" /></svg>
   }
}