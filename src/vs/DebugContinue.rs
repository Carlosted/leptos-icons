#[cfg(feature = "VsDebugContinue")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugContinue")]
/// *This icon requires the feature* `VsDebugContinue` *to be enabled*.
#[component]
pub fn DebugContinue(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M2.5 2H4v12H2.5V2zm4.936.39L6.25 3v10l1.186.61 7-5V7.39l-7-5zM12.71 8l-4.96 3.543V4.457L12.71 8z" /></svg>
   }
}