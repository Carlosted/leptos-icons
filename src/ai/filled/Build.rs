#[cfg(feature = "AiFilledBuild")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiFilledBuild")]
/// *This icon requires the feature* `AiFilledBuild` *to be enabled*.
#[component]
pub fn Build(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M916 210H376c-17.7 0-32 14.3-32 32v236H108c-17.7 0-32 14.3-32 32v272c0 17.7 14.3 32 32 32h540c17.7 0 32-14.3 32-32V546h236c17.7 0 32-14.3 32-32V242c0-17.7-14.3-32-32-32zM612 746H412V546h200v200zm268-268H680V278h200v200z" /></svg>
   }
}