#[cfg(feature = "TbMapPinShare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMapPinShare")]
/// *This icon requires the feature* `TbMapPinShare` *to be enabled*.
#[component]
pub fn MapPinShare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-map-pin-share" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M14.828 18.828a4 4 0 1 0 -5.656 0l2.828 2.829l2.828 -2.829z" /><path d="M12 16v.01" /><path d="M18.364 5.636a9 9 0 0 0 -12.728 0" /><path d="M15.536 8.464a5 5 0 0 0 -7.072 0" /></svg>
   }
}