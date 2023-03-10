#[cfg(feature = "VsArrowSmallDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsArrowSmallDown")]
/// *This icon requires the feature* `VsArrowSmallDown` *to be enabled*.
#[component]
pub fn ArrowSmallDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M10.7 8.64l-2.5 2.5h-.7L5 8.64l.7-.71 1.65 1.64V4h1v5.57L10 7.92l.7.72z" /></svg>
   }
}