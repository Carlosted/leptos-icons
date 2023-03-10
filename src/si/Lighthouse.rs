#[cfg(feature = "SiLighthouse")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiLighthouse")]
/// *This icon requires the feature* `SiLighthouse` *to be enabled*.
#[component]
pub fn Lighthouse(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0l5.5 3.5v5H20v3h-2.25l2 12.5H4.25l2-12.5H4v-3h2.5V3.53zm2.94 13.25l-6.22 2.26L8 20.04l7.5-2.75zM12 3.56L9.5 5.17V8.5h5V5.15Z" /></svg>
   }
}