#[cfg(feature = "SiGiphy")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiGiphy")]
/// *This icon requires the feature* `SiGiphy` *to be enabled*.
#[component]
pub fn Giphy(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M2.666 0v24h18.668V8.666l-2.668 2.668v10H5.334V2.668H10L12.666 0zm10.668 0v8h8V5.334h-2.668V2.668H16V0" /></svg>
   }
}