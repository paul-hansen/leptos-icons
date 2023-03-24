#[cfg(feature = "WiDayShowers")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "WiDayShowers")]
/// *This icon requires the feature* `WiDayShowers` *to be enabled*.
#[component]
pub fn DayShowers(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" id="Layer_1" x="0px" y="0px" viewBox="0 0 30 30" style="enable-background:new 0 0 30 30;" xml:space="preserve"><path d="M1.56,16.88c0,1.33,0.46,2.47,1.39,3.43s2.06,1.46,3.4,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.34c0-0.12-0.06-0.18-0.17-0.18&#xD;&#xA;	c-0.85-0.04-1.57-0.38-2.17-1.02s-0.89-1.39-0.89-2.25c0-0.84,0.28-1.56,0.84-2.17s1.26-0.96,2.1-1.06l0.5-0.04&#xD;&#xA;	c0.13,0,0.2-0.06,0.2-0.18l0.06-0.53c0.11-1.08,0.56-1.99,1.37-2.71c0.81-0.73,1.76-1.09,2.86-1.09c1.09,0,2.05,0.36,2.86,1.09&#xD;&#xA;	c0.81,0.73,1.27,1.63,1.37,2.71l0.07,0.57c0,0.12,0.06,0.18,0.18,0.18h1.67c0.88,0,1.63,0.32,2.27,0.96&#xD;&#xA;	c0.64,0.64,0.96,1.39,0.96,2.27c0,0.85-0.3,1.59-0.9,2.22s-1.32,0.98-2.16,1.05c-0.11,0-0.17,0.06-0.17,0.18v1.34&#xD;&#xA;	c0,0.11,0.06,0.17,0.17,0.17c0.88-0.02,1.67-0.26,2.4-0.72s1.3-1.05,1.71-1.8c0.42-0.75,0.62-1.56,0.62-2.44&#xD;&#xA;	c0-0.71-0.14-1.37-0.41-1.96c0.76-0.94,1.13-2.03,1.13-3.28c0-0.71-0.14-1.39-0.41-2.04c-0.27-0.65-0.65-1.2-1.12-1.67&#xD;&#xA;	C21,7.46,20.44,7.09,19.8,6.82c-0.65-0.28-1.33-0.41-2.04-0.41c-1.51,0-2.78,0.55-3.81,1.66c-0.79-0.43-1.7-0.64-2.73-0.64&#xD;&#xA;	c-1.41,0-2.66,0.44-3.75,1.31s-1.77,1.99-2.07,3.35c-1.12,0.26-2.05,0.83-2.77,1.72C1.92,14.7,1.56,15.73,1.56,16.88z M6.97,23.58&#xD;&#xA;	c0,0.18,0.05,0.36,0.16,0.53c0.11,0.18,0.26,0.29,0.45,0.36c0.19,0.07,0.4,0.05,0.61-0.06c0.22-0.11,0.36-0.29,0.44-0.55l0.25-1.05&#xD;&#xA;	c0.07-0.21,0.05-0.41-0.07-0.62c-0.12-0.21-0.29-0.35-0.51-0.42c-0.25-0.06-0.47-0.03-0.67,0.08s-0.32,0.3-0.37,0.53l-0.28,0.99&#xD;&#xA;	C6.98,23.42,6.97,23.49,6.97,23.58z M8.28,18.86c0,0.38,0.21,0.64,0.64,0.79c0.22,0.08,0.43,0.06,0.64-0.05&#xD;&#xA;	c0.21-0.11,0.34-0.29,0.41-0.53l0.24-1.03c0.07-0.21,0.05-0.41-0.07-0.62c-0.11-0.21-0.28-0.35-0.51-0.42&#xD;&#xA;	c-0.24-0.06-0.47-0.04-0.67,0.08s-0.32,0.29-0.37,0.52l-0.3,1.02C8.29,18.7,8.28,18.78,8.28,18.86z M9.5,26.75&#xD;&#xA;	c0,0.16,0.06,0.33,0.17,0.5c0.11,0.17,0.28,0.29,0.49,0.36c0.01,0,0.04,0,0.1,0.01c0.06,0.01,0.11,0.01,0.15,0.01&#xD;&#xA;	c0.14,0,0.26-0.02,0.37-0.07c0.19-0.08,0.33-0.27,0.41-0.58l0.27-0.99c0.07-0.23,0.05-0.45-0.07-0.65c-0.12-0.2-0.29-0.34-0.51-0.4&#xD;&#xA;	c-0.23-0.07-0.45-0.05-0.65,0.07c-0.2,0.12-0.34,0.29-0.4,0.51l-0.28,1.02C9.51,26.63,9.5,26.7,9.5,26.75z M9.96,4.68&#xD;&#xA;	c0,0.25,0.08,0.46,0.25,0.62l0.66,0.65c0.34,0.34,0.73,0.34,1.17,0c0.16-0.17,0.24-0.38,0.24-0.61c0-0.23-0.08-0.43-0.24-0.61&#xD;&#xA;	l-0.63-0.66c-0.16-0.16-0.36-0.24-0.6-0.24c-0.23,0-0.43,0.08-0.6,0.25C10.04,4.24,9.96,4.44,9.96,4.68z M10.85,21.96&#xD;&#xA;	c0,0.17,0.05,0.34,0.16,0.51c0.11,0.17,0.26,0.28,0.47,0.35c0.23,0.07,0.44,0.05,0.64-0.05c0.19-0.1,0.33-0.29,0.4-0.56l0.24-1.01&#xD;&#xA;	c0.07-0.23,0.05-0.45-0.06-0.65c-0.11-0.2-0.28-0.34-0.5-0.41c-0.25-0.07-0.48-0.04-0.68,0.08c-0.2,0.12-0.33,0.3-0.37,0.53&#xD;&#xA;	l-0.28,1.03C10.85,21.81,10.85,21.87,10.85,21.96z M13.63,23.68c0.02,0.38,0.23,0.65,0.63,0.83l0.25,0.04&#xD;&#xA;	c0.16,0,0.32-0.05,0.47-0.16c0.15-0.11,0.26-0.27,0.32-0.5l0.29-1.01c0.06-0.24,0.03-0.46-0.09-0.66c-0.12-0.2-0.3-0.33-0.53-0.37&#xD;&#xA;	c-0.21-0.07-0.41-0.05-0.62,0.07s-0.34,0.29-0.41,0.51l-0.27,1.02c-0.01,0.02-0.01,0.05-0.02,0.08s-0.01,0.06-0.02,0.08&#xD;&#xA;	S13.63,23.66,13.63,23.68z M15.03,18.92c0,0.16,0.05,0.32,0.15,0.48c0.1,0.16,0.25,0.27,0.45,0.32l0.25,0.03&#xD;&#xA;	c0.19,0,0.37-0.06,0.52-0.18s0.24-0.28,0.28-0.47l0.27-0.99c0.07-0.24,0.05-0.45-0.07-0.65c-0.11-0.2-0.28-0.33-0.51-0.39&#xD;&#xA;	c-0.23-0.07-0.45-0.05-0.64,0.06c-0.2,0.11-0.33,0.28-0.39,0.5l-0.3,1.06C15.04,18.77,15.03,18.85,15.03,18.92z M15.36,9.06&#xD;&#xA;	c0.66-0.64,1.46-0.96,2.4-0.96c0.98,0,1.82,0.35,2.51,1.04c0.69,0.69,1.04,1.53,1.04,2.51c0,0.56-0.16,1.15-0.47,1.76&#xD;&#xA;	c-0.96-0.96-2.11-1.43-3.47-1.43h-0.34C16.77,10.84,16.21,9.87,15.36,9.06z M16.9,3.83c0,0.25,0.08,0.45,0.24,0.61&#xD;&#xA;	c0.16,0.16,0.36,0.24,0.61,0.24s0.45-0.08,0.61-0.24c0.16-0.16,0.24-0.36,0.24-0.61V1.81c0-0.25-0.08-0.46-0.24-0.62&#xD;&#xA;	c-0.16-0.16-0.36-0.24-0.61-0.24s-0.45,0.08-0.61,0.24c-0.16,0.16-0.24,0.37-0.24,0.62V3.83z M22.45,6.12&#xD;&#xA;	c0,0.25,0.08,0.45,0.23,0.61c0.21,0.17,0.41,0.25,0.62,0.25c0.19,0,0.38-0.08,0.59-0.25l1.43-1.43c0.16-0.18,0.24-0.39,0.24-0.63&#xD;&#xA;	c0-0.24-0.08-0.44-0.24-0.6c-0.16-0.16-0.36-0.24-0.59-0.24s-0.43,0.08-0.61,0.24L22.68,5.5C22.53,5.68,22.45,5.88,22.45,6.12z&#xD;&#xA;	 M23.24,17.95c0,0.23,0.09,0.44,0.26,0.63l0.62,0.64c0.21,0.21,0.41,0.31,0.62,0.31c0.19,0,0.39-0.1,0.58-0.31&#xD;&#xA;	c0.18-0.18,0.27-0.39,0.26-0.61c-0.01-0.23-0.09-0.43-0.26-0.6l-0.65-0.66c-0.16-0.16-0.35-0.24-0.57-0.24&#xD;&#xA;	c-0.24,0-0.44,0.08-0.61,0.25C23.33,17.52,23.24,17.72,23.24,17.95z M24.71,11.64c0,0.22,0.08,0.42,0.24,0.58&#xD;&#xA;	c0.16,0.16,0.36,0.24,0.58,0.24h2.04c0.26,0,0.47-0.08,0.63-0.23c0.16-0.16,0.24-0.35,0.24-0.59c0-0.25-0.08-0.46-0.25-0.62&#xD;&#xA;	c-0.17-0.16-0.37-0.24-0.62-0.24h-2.04c-0.23,0-0.43,0.08-0.59,0.25C24.79,11.2,24.71,11.41,24.71,11.64z" /></svg>
   }
}