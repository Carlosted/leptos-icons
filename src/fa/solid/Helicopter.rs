#[cfg(feature = "FaSolidHelicopter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidHelicopter")]
/// *This icon requires the feature* `FaSolidHelicopter` *to be enabled*.
#[component]
pub fn Helicopter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M128 32c0-17.7 14.3-32 32-32H544c17.7 0 32 14.3 32 32s-14.3 32-32 32H384v64h32c88.4 0 160 71.6 160 160v64c0 17.7-14.3 32-32 32H384 320c-20.1 0-39.1-9.5-51.2-25.6l-71.4-95.2c-3.5-4.7-8.3-8.3-13.7-10.5L47.2 198.1c-9.5-3.8-16.7-12-19.2-22L5 83.9C2.4 73.8 10.1 64 20.5 64H48c10.1 0 19.6 4.7 25.6 12.8L112 128H320V64H160c-17.7 0-32-14.3-32-32zM384 320H512V288c0-53-43-96-96-96H384V320zM630.6 425.4c12.5 12.5 12.5 32.8 0 45.3l-3.9 3.9c-24 24-56.6 37.5-90.5 37.5H256c-17.7 0-32-14.3-32-32s14.3-32 32-32H536.2c17 0 33.3-6.7 45.3-18.7l3.9-3.9c12.5-12.5 32.8-12.5 45.3 0z" /></svg>
   }
}