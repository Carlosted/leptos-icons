#[cfg(feature = "VsDiff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDiff")]
/// *This icon requires the feature* `VsDiff` *to be enabled*.
#[component]
pub fn Diff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M2 3.5l.5-.5h5l.5.5v9l-.5.5h-5l-.5-.5v-9zM3 12h4V6H3v6zm0-7h4V4H3v1zm6.5-2h5l.5.5v9l-.5.5h-5l-.5-.5v-9l.5-.5zm.5 9h4v-2h-4v2zm0-4h4V4h-4v4z" /></svg>
   }
}