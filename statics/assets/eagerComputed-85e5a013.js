import{R as s}from"./collapseMotion-090e2670.js";import{r as u,n as o,l as a,aE as t,a1 as f}from"./index-ead633b2.js";function l(){var e=u({}),n=null;return o(function(){n=s.subscribe(function(r){e.value=r})}),a(function(){s.unsubscribe(n)}),e}function v(e){var n=t();return f(function(){n.value=e()},{flush:"sync"}),n}export{v as e,l as u};