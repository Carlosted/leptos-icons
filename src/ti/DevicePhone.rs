#[cfg(feature = "TiDevicePhone")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiDevicePhone")]
/// *This icon requires the feature* `TiDevicePhone` *to be enabled*.
#[component]
pub fn DevicePhone(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><g><path d="M15 3h-7c-1.654 0-3 1.346-3 3v12c0 1.654 1.346 3 3 3h7c1.654 0 3-1.346 3-3v-12c0-1.654-1.346-3-3-3zm1 15c0 .551-.449 1-1 1h-7c-.551 0-1-.449-1-1v-12c0-.551.449-1 1-1h7c.551 0 1 .449 1 1v12zM14 6h-5c-.55 0-1 .45-1 1v9c0 .55.45 1 1 1h1.5c0 .553.448 1 1 1s1-.447 1-1h1.5c.55 0 1-.45 1-1v-9c0-.55-.45-1-1-1zm0 10h-5v-9h5v9z" /></g></svg>
   }
}