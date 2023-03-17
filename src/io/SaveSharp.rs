#[cfg(feature = "IoSaveSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSaveSharp")]
/// *This icon requires the feature* `IoSaveSharp` *to be enabled*.
#[component]
pub fn SaveSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M380.44,32H64A32,32,0,0,0,32,64V448a32,32,0,0,0,32,32H448a32.09,32.09,0,0,0,32-32V131.56ZM112,176V112H304v64ZM335.91,355.76a80,80,0,1,1-83.66-83.67A80.21,80.21,0,0,1,335.91,355.76Z" /></svg>
   }
}