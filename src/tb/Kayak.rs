#[cfg(feature = "TbKayak")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbKayak")]
/// *This icon requires the feature* `TbKayak` *to be enabled*.
#[component]
pub fn Kayak(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-kayak" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6.414 6.414a2 2 0 0 0 0 -2.828l-1.414 -1.414l-2.828 2.828l1.414 1.414a2 2 0 0 0 2.828 0z" /><path d="M17.586 17.586a2 2 0 0 0 0 2.828l1.414 1.414l2.828 -2.828l-1.414 -1.414a2 2 0 0 0 -2.828 0z" /><path d="M6.5 6.5l11 11" /><path d="M22 2.5c-9.983 2.601 -17.627 7.952 -20 19.5c9.983 -2.601 17.627 -7.952 20 -19.5z" /><path d="M6.5 12.5l5 5" /><path d="M12.5 6.5l5 5" /></svg>
   }
}