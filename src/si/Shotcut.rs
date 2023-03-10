#[cfg(feature = "SiShotcut")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiShotcut")]
/// *This icon requires the feature* `SiShotcut` *to be enabled*.
#[component]
pub fn Shotcut(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 0h6.667v24H0v-.889h5.778V.889H0V0zm7.556 0v24H24v-.889H8.444V.889H24V0H7.556zm1.388 22.611H24V1.389H8.944v21.222zM5.278 1.389H0v21.222h5.278V1.389z" /></svg>
   }
}