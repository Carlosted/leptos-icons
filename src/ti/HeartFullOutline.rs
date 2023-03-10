#[cfg(feature = "TiHeartFullOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiHeartFullOutline")]
/// *This icon requires the feature* `TiHeartFullOutline` *to be enabled*.
#[component]
pub fn HeartFullOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1" width="24" height="24" viewBox="0 0 24 24"><path d="M2.2 9.4c0 1.3.2 3.3 2 5.1 1.6 1.6 6.9 5.2 7.1 5.4.2.1.4.2.6.2s.4-.1.6-.2c.2-.2 5.5-3.7 7.1-5.4 1.8-1.8 2-3.8 2-5.1 0-3-2.4-5.4-5.4-5.4-1.6 0-3.2.9-4.2 2.3-1-1.4-2.6-2.3-4.4-2.3-2.9 0-5.4 2.4-5.4 5.4z" /></svg>
   }
}