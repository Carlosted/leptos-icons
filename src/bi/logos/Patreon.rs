#[cfg(feature = "BiLogosPatreon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosPatreon")]
/// *This icon requires the feature* `BiLogosPatreon` *to be enabled*.
#[component]
pub fn Patreon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="14.508" cy="9.831" r="6.496" /><path d="M2.996 3.335H6.17v17.33H2.996z" /></svg>
   }
}