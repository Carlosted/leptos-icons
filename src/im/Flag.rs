#[cfg(feature = "ImFlag")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFlag")]
/// *This icon requires the feature* `ImFlag` *to be enabled*.
#[component]
pub fn Flag(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 0h2v16h-2v-16z" /><path fill="#000000" d="M13 10.047c1.291 0 2.415-0.312 3-0.773v-8c-0.585 0.461-1.709 0.773-3 0.773s-2.415-0.312-3-0.773v8c0.585 0.461 1.709 0.773 3 0.773z" /><path fill="#000000" d="M9.5 0.508c-0.733-0.312-1.805-0.508-3-0.508-1.506 0-2.818 0.312-3.5 0.773v8c0.682-0.461 1.994-0.773 3.5-0.773 1.195 0 2.267 0.197 3 0.508v-8z" /></svg>
   }
}