#[cfg(feature = "VsDebugStop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugStop")]
/// *This icon requires the feature* `VsDebugStop` *to be enabled*.
#[component]
pub fn DebugStop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M2 2v12h12V2H2zm10.75 10.75h-9.5v-9.5h9.5v9.5z" /></svg>
   }
}