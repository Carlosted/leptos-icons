#[cfg(feature = "SiTelegraph")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiTelegraph")]
/// *This icon requires the feature* `SiTelegraph` *to be enabled*.
#[component]
pub fn Telegraph(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 0v24h24V0H0zm6 6h12v3h-4.5v9h-3V9H6V6Z" /></svg>
   }
}