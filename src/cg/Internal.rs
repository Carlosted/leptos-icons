#[cfg(feature = "CgInternal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgInternal")]
/// *This icon requires the feature* `CgInternal` *to be enabled*.
#[component]
pub fn Internal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M20.7084 4.41165L10.4586 14.6986H14.0488V16.6986H7.04883V9.69861H9.04883V13.2799L19.2916 3L20.7084 4.41165Z" fill="currentColor" /><path d="M11 4.70581V6.70581H5V18.7058H17V12.7058H19V20.7058H3V4.70581H11Z" fill="currentColor" /></svg>
   }
}