#[cfg(feature = "VsProject")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsProject")]
/// *This icon requires the feature* `VsProject` *to be enabled*.
#[component]
pub fn Project(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.5 1h13l.5.5v13l-.5.5h-13l-.5-.5v-13l.5-.5zM2 14h12V2H2v12zM3 3h2v10H3V3zm6 0H7v6h2V3zm2 0h2v8h-2V3z" /></svg>
   }
}