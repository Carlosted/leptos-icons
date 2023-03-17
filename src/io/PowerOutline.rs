#[cfg(feature = "IoPowerOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPowerOutline")]
/// *This icon requires the feature* `IoPowerOutline` *to be enabled*.
#[component]
pub fn PowerOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M378,108a191.41,191.41,0,0,1,70,148c0,106-86,192-192,192S64,362,64,256a192,192,0,0,1,69-148" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="256" y1="64" x2="256" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}