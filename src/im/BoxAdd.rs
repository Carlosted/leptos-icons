#[cfg(feature = "ImBoxAdd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImBoxAdd")]
/// *This icon requires the feature* `ImBoxAdd` *to be enabled*.
#[component]
pub fn BoxAdd(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M13 1h-10l-3 3v10.5c0 0.276 0.224 0.5 0.5 0.5h15c0.276 0 0.5-0.224 0.5-0.5v-10.5l-3-3zM8 13l-5-4h3v-3h4v3h3l-5 4zM2.414 3l1-1h9.172l1 1h-11.172z" /></svg>
   }
}