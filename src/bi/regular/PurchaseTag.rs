#[cfg(feature = "BiRegularPurchaseTag")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularPurchaseTag")]
/// *This icon requires the feature* `BiRegularPurchaseTag` *to be enabled*.
#[component]
pub fn PurchaseTag(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13.707 3.293A.996.996 0 0 0 13 3H4a1 1 0 0 0-1 1v9c0 .266.105.52.293.707l8 8a.997.997 0 0 0 1.414 0l9-9a.999.999 0 0 0 0-1.414l-8-8zM12 19.586l-7-7V5h7.586l7 7L12 19.586z" /><circle cx="8.496" cy="8.495" r="1.505" /></svg>
   }
}