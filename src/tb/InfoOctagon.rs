#[cfg(feature = "TbInfoOctagon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbInfoOctagon")]
/// *This icon requires the feature* `TbInfoOctagon` *to be enabled*.
#[component]
pub fn InfoOctagon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-info-octagon" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M9.103 2h5.794a3 3 0 0 1 2.122 .879l4.101 4.101a3 3 0 0 1 .88 2.123v5.794a3 3 0 0 1 -.879 2.122l-4.101 4.101a3 3 0 0 1 -2.122 .879h-5.795a3 3 0 0 1 -2.122 -.879l-4.101 -4.1a3 3 0 0 1 -.88 -2.123v-5.794a3 3 0 0 1 .879 -2.122l4.101 -4.101a3 3 0 0 1 2.123 -.88z" /><path d="M12 8l.01 0" /><path d="M11 12l1 0l0 4l1 0" /></svg>
   }
}