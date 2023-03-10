#[cfg(feature = "CgScreenMirror")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgScreenMirror")]
/// *This icon requires the feature* `CgScreenMirror` *to be enabled*.
#[component]
pub fn ScreenMirror(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5 8H19V14H16V16H21V6H3V16H8V14H5V8Z" fill="currentColor" /><path d="M16.3301 19L12 13L7.66987 19H16.3301Z" fill="currentColor" /></svg>
   }
}