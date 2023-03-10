#[cfg(feature = "AiOutlinedBorder")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedBorder")]
/// *This icon requires the feature* `AiOutlinedBorder` *to be enabled*.
#[component]
pub fn Border(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zm-40 728H184V184h656v656z" /></svg>
   }
}