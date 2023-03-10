#[cfg(feature = "TiMinusOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiMinusOutline")]
/// *This icon requires the feature* `TiMinusOutline` *to be enabled*.
#[component]
pub fn MinusOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M18 16h-12c-1.654 0-3-1.346-3-3s1.346-3 3-3h12c1.654 0 3 1.346 3 3s-1.346 3-3 3zm-12-4c-.551 0-1 .449-1 1s.449 1 1 1h12c.551 0 1-.449 1-1s-.449-1-1-1h-12z" /></svg>
   }
}