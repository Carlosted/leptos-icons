#[cfg(feature = "OcLgClockFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgClockFill")]
/// *This icon requires the feature* `OcLgClockFill` *to be enabled*.
#[component]
pub fn ClockFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M1 12C1 5.925 5.925 1 12 1s11 4.925 11 11-4.925 11-11 11S1 18.075 1 12Zm11.575-4.75a.825.825 0 1 0-1.65 0v5.5c0 .296.159.57.416.716l3.5 2a.825.825 0 0 0 .818-1.432l-3.084-1.763Z" /></svg>
   }
}