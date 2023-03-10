#[cfg(feature = "VsLock")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLock")]
/// *This icon requires the feature* `VsLock` *to be enabled*.
#[component]
pub fn Lock(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M13 7h-1V5a4 4 0 1 0-8 0v2H3L2 8v6l1 1h10l1-1V8l-1-1zM5 5a3 3 0 1 1 6 0v2H5V5zm8 9H3V8h10v6z" /></svg>
   }
}