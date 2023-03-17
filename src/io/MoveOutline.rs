#[cfg(feature = "IoMoveOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMoveOutline")]
/// *This icon requires the feature* `IoMoveOutline` *to be enabled*.
#[component]
pub fn MoveOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="176 112 256 32 336 112" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="255.98" y1="32" x2="256" y2="480" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="176 400 256 480 336 400" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="400 176 480 256 400 336" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="112 176 32 256 112 336" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="32" y1="256" x2="480" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}