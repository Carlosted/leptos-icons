#[cfg(feature = "VsUnlock")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsUnlock")]
/// *This icon requires the feature* `VsUnlock` *to be enabled*.
#[component]
pub fn Unlock(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M5 7V5a3 3 0 0 1 5.83-1h1.044A4.002 4.002 0 0 0 4 5v2H3L2 8v6l1 1h10l1-1V8l-1-1H5zm6 1h2v6H3V8h8z" /></svg>
   }
}