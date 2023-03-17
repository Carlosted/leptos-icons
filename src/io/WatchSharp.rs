#[cfg(feature = "IoWatchSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoWatchSharp")]
/// *This icon requires the feature* `IoWatchSharp` *to be enabled*.
#[component]
pub fn WatchSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="136" y="136" width="240" height="240" rx="8" ry="8" /><path d="M384,96H336V16H176V96H128a32,32,0,0,0-32,32V384a32,32,0,0,0,32,32h48v80H336V416h48a32,32,0,0,0,32-32V128A32,32,0,0,0,384,96Zm8,272a24,24,0,0,1-24,24H144a24,24,0,0,1-24-24V144a24,24,0,0,1,24-24H368a24,24,0,0,1,24,24Z" /></svg>
   }
}