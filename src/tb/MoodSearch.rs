#[cfg(feature = "TbMoodSearch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMoodSearch")]
/// *This icon requires the feature* `TbMoodSearch` *to be enabled*.
#[component]
pub fn MoodSearch(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-mood-search" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M21 12a9 9 0 1 0 -8.99 9" /><path d="M9 10h.01" /><path d="M15 10h.01" /><path d="M9.5 15a3.556 3.556 0 0 0 1.823 .937c.221 .042 .448 .063 .677 .063" /><path d="M18 18m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /><path d="M20.2 20.2l1.8 1.8" /></svg>
   }
}