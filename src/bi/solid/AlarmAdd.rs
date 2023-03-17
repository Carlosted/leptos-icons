#[cfg(feature = "BiSolidAlarmAdd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidAlarmAdd")]
/// *This icon requires the feature* `BiSolidAlarmAdd` *to be enabled*.
#[component]
pub fn AlarmAdd(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 4c-4.879 0-9 4.121-9 9s4.121 9 9 9 9-4.121 9-9-4.121-9-9-9zm4 10h-3v3h-2v-3H8v-2h3V9h2v3h3v2zm1.284-10.293 1.412-1.416 3.01 3-1.413 1.417zM5.282 2.294 6.7 3.706l-2.99 3-1.417-1.413z" /></svg>
   }
}