#[cfg(feature = "CgExtension")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgExtension")]
/// *This icon requires the feature* `CgExtension` *to be enabled*.
#[component]
pub fn Extension(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M13 3H21V11H13V3ZM15 5H19V9H15V5Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M17 21V13H11V7H3V21H17ZM9 9H5V13H9V9ZM5 19L5 15H9V19H5ZM11 19V15H15V19H11Z" fill="currentColor" /></svg>
   }
}