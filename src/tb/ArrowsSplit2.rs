#[cfg(feature = "TbArrowsSplit2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowsSplit2")]
/// *This icon requires the feature* `TbArrowsSplit2` *to be enabled*.
#[component]
pub fn ArrowsSplit2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrows-split-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M21 17h-5.397a5 5 0 0 1 -4.096 -2.133l-.514 -.734a5 5 0 0 0 -4.096 -2.133h-3.897" /><path d="M21 7h-5.395a5 5 0 0 0 -4.098 2.135l-.51 .73a5 5 0 0 1 -4.097 2.135h-3.9" /><path d="M18 10l3 -3l-3 -3" /><path d="M18 20l3 -3l-3 -3" /></svg>
   }
}