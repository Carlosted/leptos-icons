#[cfg(feature = "SiPatreon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiPatreon")]
/// *This icon requires the feature* `SiPatreon` *to be enabled*.
#[component]
pub fn Patreon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 .48v23.04h4.22V.48zm15.385 0c-4.764 0-8.641 3.88-8.641 8.65 0 4.755 3.877 8.623 8.641 8.623 4.75 0 8.615-3.868 8.615-8.623C24 4.36 20.136.48 15.385.48z" /></svg>
   }
}