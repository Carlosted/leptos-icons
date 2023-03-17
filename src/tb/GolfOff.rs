#[cfg(feature = "TbGolfOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbGolfOff")]
/// *This icon requires the feature* `TbGolfOff` *to be enabled*.
#[component]
pub fn GolfOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-golf-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 18v-6m0 -4v-5l7 4l-5.07 2.897" /><path d="M9 17.67c-.62 .36 -1 .82 -1 1.33c0 1.1 1.8 2 4 2s4 -.9 4 -2c0 -.5 -.38 -.97 -1 -1.33" /><path d="M3 3l18 18" /></svg>
   }
}