#[cfg(feature = "SiXilinx")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiXilinx")]
/// *This icon requires the feature* `SiXilinx` *to be enabled*.
#[component]
pub fn Xilinx(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M8 18l5.241 6H5.586L.345 18l5.241-6L.345 6l5.241-6h7.655L8 6l5.241 6L8 18zM23.655 0H13.241l5.241 6 5.173-6zM13.241 24h10.414l-5.172-6-5.242 6z" /></svg>
   }
}