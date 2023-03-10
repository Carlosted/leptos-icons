#[cfg(feature = "VsArrowSmallRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsArrowSmallRight")]
/// *This icon requires the feature* `VsArrowSmallRight` *to be enabled*.
#[component]
pub fn ArrowSmallRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8.64 5l2.5 2.5v.7l-2.5 2.5-.71-.7 1.64-1.65H4v-1h5.57L7.92 5.7l.72-.7z" /></svg>
   }
}