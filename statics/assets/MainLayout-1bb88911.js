import{e as k,s as M,v as D,_ as Oe,b as a,x as $,P as x,y as xe,z as Re,A as Ee,C as Ie,D as P,E as Se,r as T,G as we,H as De,I as G,J as Te,K as He,L as ie,n as Ve,M as Ue,N as ce,O as ue,Q as Fe,u as We,g as Ge,h as Je,o as E,i as I,w as c,a as H,d as S,R as B,t as de,j as fe,S as Qe,T as Xe,B as Ye,p as Ze,k as Ke}from"./index-ead633b2.js";import"./index-88f2ff4c.js";import{D as qe,a as Be,M as q,S as Pe,b as et,c as tt,d as rt,_ as at,e as nt}from"./index-3ce88edd.js";import{U as ve,A as ot}from"./UserOutlined-09e24cff.js";import{_ as lt}from"./index-8410ae47.js";import{R as me,L as pe}from"./LeftOutlined-91b2dd16.js";import{_ as st}from"./_plugin-vue_export-helper-c27b6911.js";import"./collapseMotion-090e2670.js";import"./eagerComputed-85e5a013.js";import"./useSize-108b0685.js";import"./useMergedState-426bca96.js";var it=function(e){return!isNaN(parseFloat(e))&&isFinite(e)};const ct=it;var ut=["class","style"],dt=function(){return{prefixCls:String,href:String,separator:x.any,overlay:x.any,onClick:Function}};const V=k({compatConfig:{MODE:3},name:"ABreadcrumbItem",inheritAttrs:!1,__ANT_BREADCRUMB_ITEM:!0,props:dt(),slots:["separator","overlay"],setup:function(e,r){var n=r.slots,o=r.attrs,_=M("breadcrumb",e),d=_.prefixCls,h=function(s,i){var l=D(n,e,"overlay");return l?a(Be,{overlay:l,placement:"bottom"},{default:function(){return[a("span",{class:"".concat(i,"-overlay-link")},[s,a(qe,null,null)])]}}):s};return function(){var y,s=(y=D(n,e,"separator"))!==null&&y!==void 0?y:"/",i=D(n,e),l=o.class,m=o.style,f=Oe(o,ut),g;return e.href!==void 0?g=a("a",$({class:"".concat(d.value,"-link"),onClick:e.onClick},f),[i]):g=a("span",$({class:"".concat(d.value,"-link"),onClick:e.onClick},f),[i]),g=h(g,d.value),i?a("span",{class:l,style:m},[g,s&&a("span",{class:"".concat(d.value,"-separator")},[s])]):null}}});var ft=function(){return{prefixCls:String,routes:{type:Array},params:x.any,separator:x.any,itemRender:{type:Function}}};function vt(t,e){if(!t.breadcrumbName)return null;var r=Object.keys(e).join("|"),n=t.breadcrumbName.replace(new RegExp(":(".concat(r,")"),"g"),function(o,_){return e[_]||o});return n}function ge(t){var e=t.route,r=t.params,n=t.routes,o=t.paths,_=n.indexOf(e)===n.length-1,d=vt(e,r);return _?a("span",null,[d]):a("a",{href:"#/".concat(o.join("/"))},[d])}const L=k({compatConfig:{MODE:3},name:"ABreadcrumb",props:ft(),slots:["separator","itemRender"],setup:function(e,r){var n=r.slots,o=M("breadcrumb",e),_=o.prefixCls,d=o.direction,h=function(l,m){return l=(l||"").replace(/^\//,""),Object.keys(m).forEach(function(f){l=l.replace(":".concat(f),m[f])}),l},y=function(l,m,f){var g=Se(l),C=h(m||"",f);return C&&g.push(C),g},s=function(l){var m=l.routes,f=m===void 0?[]:m,g=l.params,C=g===void 0?{}:g,w=l.separator,u=l.itemRender,p=u===void 0?ge:u,b=[];return f.map(function(v){var O=h(v.path,C);O&&b.push(O);var N=[].concat(b),j=null;return v.children&&v.children.length&&(j=a(q,null,{default:function(){return[v.children.map(function(A){return a(q.Item,{key:A.path||A.breadcrumbName},{default:function(){return[p({route:A,params:C,routes:f,paths:y(N,A.path,C)})]}})})]}})),a(V,{overlay:j,separator:w,key:O||v.breadcrumbName},{default:function(){return[p({route:v,params:C,routes:f,paths:N})]}})})};return function(){var i,l,m,f=e.routes,g=e.params,C=g===void 0?{}:g,w=xe(D(n,e)),u=(i=D(n,e,"separator"))!==null&&i!==void 0?i:"/",p=e.itemRender||n.itemRender||ge;f&&f.length>0?m=s({routes:f,params:C,separator:u,itemRender:p}):w.length&&(m=w.map(function(v,O){return Re(Ee(v.type)==="object"&&(v.type.__ANT_BREADCRUMB_ITEM||v.type.__ANT_BREADCRUMB_SEPARATOR),"Breadcrumb","Only accepts Breadcrumb.Item and Breadcrumb.Separator as it's children"),Ie(v,{separator:u,key:O})}));var b=(l={},P(l,_.value,!0),P(l,"".concat(_.value,"-rtl"),d.value==="rtl"),l);return a("div",{class:b},[m])}}});var mt=["separator","class"],pt=function(){return{prefixCls:String}};const ee=k({compatConfig:{MODE:3},name:"ABreadcrumbSeparator",__ANT_BREADCRUMB_SEPARATOR:!0,inheritAttrs:!1,props:pt(),setup:function(e,r){var n=r.slots,o=r.attrs,_=M("breadcrumb",e),d=_.prefixCls;return function(){var h;o.separator;var y=o.class,s=Oe(o,mt),i=xe((h=n.default)===null||h===void 0?void 0:h.call(n));return a("span",$({class:["".concat(d.value,"-separator"),y]},s),[i.length>0?i:"/"])}}});L.Item=V;L.Separator=ee;L.install=function(t){return t.component(L.name,L),t.component(V.name,V),t.component(ee.name,ee),t};var te=function(){return{prefixCls:String,hasSider:{type:Boolean,default:void 0},tagName:String}};function J(t){var e=t.suffixCls,r=t.tagName,n=t.name;return function(o){var _=k({compatConfig:{MODE:3},name:n,props:te(),setup:function(h,y){var s=y.slots,i=M(e,h),l=i.prefixCls;return function(){var m=$($({},h),{},{prefixCls:l.value,tagName:r});return a(o,m,s)}}});return _}}var re=k({compatConfig:{MODE:3},props:te(),setup:function(e,r){var n=r.slots;return function(){return a(e.tagName,{class:e.prefixCls},n)}}}),gt=k({compatConfig:{MODE:3},props:te(),setup:function(e,r){var n=r.slots,o=M("",e),_=o.direction,d=T([]),h={addSider:function(i){d.value=[].concat(Se(d.value),[i])},removeSider:function(i){d.value=d.value.filter(function(l){return l!==i})}};we(Pe,h);var y=De(function(){var s,i=e.prefixCls,l=e.hasSider;return s={},P(s,"".concat(i),!0),P(s,"".concat(i,"-has-sider"),typeof l=="boolean"?l:d.value.length>0),P(s,"".concat(i,"-rtl"),_.value==="rtl"),s});return function(){var s=e.tagName;return a(s,{class:y.value},n)}}}),ht=J({suffixCls:"layout",tagName:"section",name:"ALayout"})(gt),U=J({suffixCls:"layout-header",tagName:"header",name:"ALayoutHeader"})(re),Z=J({suffixCls:"layout-footer",tagName:"footer",name:"ALayoutFooter"})(re),F=J({suffixCls:"layout-content",tagName:"main",name:"ALayoutContent"})(re);const K=ht;var _t={icon:{tag:"svg",attrs:{viewBox:"0 0 1024 1024",focusable:"false"},children:[{tag:"path",attrs:{d:"M912 192H328c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h584c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm0 284H328c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h584c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm0 284H328c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h584c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zM104 228a56 56 0 10112 0 56 56 0 10-112 0zm0 284a56 56 0 10112 0 56 56 0 10-112 0zm0 284a56 56 0 10112 0 56 56 0 10-112 0z"}}]},name:"bars",theme:"outlined"};const yt=_t;function he(t){for(var e=1;e<arguments.length;e++){var r=arguments[e]!=null?Object(arguments[e]):{},n=Object.keys(r);typeof Object.getOwnPropertySymbols=="function"&&(n=n.concat(Object.getOwnPropertySymbols(r).filter(function(o){return Object.getOwnPropertyDescriptor(r,o).enumerable}))),n.forEach(function(o){bt(t,o,r[o])})}return t}function bt(t,e,r){return e in t?Object.defineProperty(t,e,{value:r,enumerable:!0,configurable:!0,writable:!0}):t[e]=r,t}var ae=function(e,r){var n=he({},e,r.attrs);return a(G,he({},n,{icon:yt}),null)};ae.displayName="BarsOutlined";ae.inheritAttrs=!1;const Ct=ae;var _e={xs:"479.98px",sm:"575.98px",md:"767.98px",lg:"991.98px",xl:"1199.98px",xxl:"1599.98px",xxxl:"1999.98px"},Ot=function(){return{prefixCls:String,collapsible:{type:Boolean,default:void 0},collapsed:{type:Boolean,default:void 0},defaultCollapsed:{type:Boolean,default:void 0},reverseArrow:{type:Boolean,default:void 0},zeroWidthTriggerStyle:{type:Object,default:void 0},trigger:x.any,width:x.oneOfType([x.number,x.string]),collapsedWidth:x.oneOfType([x.number,x.string]),breakpoint:x.oneOf(ue("xs","sm","md","lg","xl","xxl","xxxl")),theme:x.oneOf(ue("light","dark")).def("dark"),onBreakpoint:Function,onCollapse:Function}},xt=function(){var t=0;return function(){var e=arguments.length>0&&arguments[0]!==void 0?arguments[0]:"";return t+=1,"".concat(e).concat(t)}}();const W=k({compatConfig:{MODE:3},name:"ALayoutSider",inheritAttrs:!1,props:Te(Ot(),{collapsible:!1,defaultCollapsed:!1,reverseArrow:!1,width:200,collapsedWidth:80}),emits:["breakpoint","update:collapsed","collapse"],setup:function(e,r){var n=r.emit,o=r.attrs,_=r.slots,d=M("layout-sider",e),h=d.prefixCls,y=He(Pe,void 0),s=T(!!(e.collapsed!==void 0?e.collapsed:e.defaultCollapsed)),i=T(!1);ie(function(){return e.collapsed},function(){s.value=!!e.collapsed}),we(et,s);var l=function(p,b){e.collapsed===void 0&&(s.value=p),n("update:collapsed",p),n("collapse",p,b)},m=T(function(u){i.value=u.matches,n("breakpoint",u.matches),s.value!==u.matches&&l(u.matches,"responsive")}),f;function g(u){return m.value(u)}var C=xt("ant-sider-");y&&y.addSider(C),Ve(function(){ie(function(){return e.breakpoint},function(){try{var u;(u=f)===null||u===void 0||u.removeEventListener("change",g)}catch{var p;(p=f)===null||p===void 0||p.removeListener(g)}if(typeof window<"u"){var b=window,v=b.matchMedia;if(v&&e.breakpoint&&e.breakpoint in _e){f=v("(max-width: ".concat(_e[e.breakpoint],")"));try{f.addEventListener("change",g)}catch{f.addListener(g)}g(f)}}},{immediate:!0})}),Ue(function(){try{var u;(u=f)===null||u===void 0||u.removeEventListener("change",g)}catch{var p;(p=f)===null||p===void 0||p.removeListener(g)}y&&y.removeSider(C)});var w=function(){l(!s.value,"clickTrigger")};return function(){var u,p,b,v=h.value,O=e.collapsedWidth,N=e.width,j=e.reverseArrow,Q=e.zeroWidthTriggerStyle,A=e.trigger,R=A===void 0?(u=_.trigger)===null||u===void 0?void 0:u.call(_):A,se=e.collapsible,Ne=e.theme,X=s.value?O:N,z=ct(X)?"".concat(X,"px"):String(X),Y=parseFloat(String(O||0))===0?a("span",{onClick:w,class:ce("".concat(v,"-zero-width-trigger"),"".concat(v,"-zero-width-trigger-").concat(j?"right":"left")),style:Q},[R||a(Ct,null,null)]):null,Ae={expanded:j?a(me,null,null):a(pe,null,null),collapsed:j?a(pe,null,null):a(me,null,null)},$e=s.value?"collapsed":"expanded",je=Ae[$e],ze=R!==null?Y||a("div",{class:"".concat(v,"-trigger"),onClick:w,style:{width:z}},[R||je]):null,Le=[o.style,{flex:"0 0 ".concat(z),maxWidth:z,minWidth:z,width:z}],Me=ce(v,"".concat(v,"-").concat(Ne),(p={},P(p,"".concat(v,"-collapsed"),!!s.value),P(p,"".concat(v,"-has-trigger"),se&&R!==null&&!Y),P(p,"".concat(v,"-below"),!!i.value),P(p,"".concat(v,"-zero-width"),parseFloat(z)===0),p),o.class);return a("aside",$($({},o),{},{class:Me,style:Le}),[a("div",{class:"".concat(v,"-children")},[(b=_.default)===null||b===void 0?void 0:b.call(_)]),se||i.value&&Y?ze:null])}}});var St=U,wt=W,Bt=F;const Pt=Fe(K,{Header:U,Footer:Z,Content:F,Sider:W,install:function(e){return e.component(K.name,K),e.component(U.name,U),e.component(Z.name,Z),e.component(W.name,W),e.component(F.name,F),e}});var kt={icon:{tag:"svg",attrs:{viewBox:"64 64 896 896",focusable:"false"},children:[{tag:"path",attrs:{d:"M888 792H200V168c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v688c0 4.4 3.6 8 8 8h752c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm-600-80h56c4.4 0 8-3.6 8-8V560c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v144c0 4.4 3.6 8 8 8zm152 0h56c4.4 0 8-3.6 8-8V384c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v320c0 4.4 3.6 8 8 8zm152 0h56c4.4 0 8-3.6 8-8V462c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v242c0 4.4 3.6 8 8 8zm152 0h56c4.4 0 8-3.6 8-8V304c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v400c0 4.4 3.6 8 8 8z"}}]},name:"bar-chart",theme:"outlined"};const Nt=kt;function ye(t){for(var e=1;e<arguments.length;e++){var r=arguments[e]!=null?Object(arguments[e]):{},n=Object.keys(r);typeof Object.getOwnPropertySymbols=="function"&&(n=n.concat(Object.getOwnPropertySymbols(r).filter(function(o){return Object.getOwnPropertyDescriptor(r,o).enumerable}))),n.forEach(function(o){At(t,o,r[o])})}return t}function At(t,e,r){return e in t?Object.defineProperty(t,e,{value:r,enumerable:!0,configurable:!0,writable:!0}):t[e]=r,t}var ne=function(e,r){var n=ye({},e,r.attrs);return a(G,ye({},n,{icon:Nt}),null)};ne.displayName="BarChartOutlined";ne.inheritAttrs=!1;const $t=ne;var jt={icon:{tag:"svg",attrs:{viewBox:"64 64 896 896",focusable:"false"},children:[{tag:"path",attrs:{d:"M320 224h-66v-56c0-4.4-3.6-8-8-8h-52c-4.4 0-8 3.6-8 8v56h-66c-4.4 0-8 3.6-8 8v560c0 4.4 3.6 8 8 8h66v56c0 4.4 3.6 8 8 8h52c4.4 0 8-3.6 8-8v-56h66c4.4 0 8-3.6 8-8V232c0-4.4-3.6-8-8-8zm-60 508h-80V292h80v440zm644-436h-66v-96c0-4.4-3.6-8-8-8h-52c-4.4 0-8 3.6-8 8v96h-66c-4.4 0-8 3.6-8 8v416c0 4.4 3.6 8 8 8h66v96c0 4.4 3.6 8 8 8h52c4.4 0 8-3.6 8-8v-96h66c4.4 0 8-3.6 8-8V304c0-4.4-3.6-8-8-8zm-60 364h-80V364h80v296zM612 404h-66V232c0-4.4-3.6-8-8-8h-52c-4.4 0-8 3.6-8 8v172h-66c-4.4 0-8 3.6-8 8v200c0 4.4 3.6 8 8 8h66v172c0 4.4 3.6 8 8 8h52c4.4 0 8-3.6 8-8V620h66c4.4 0 8-3.6 8-8V412c0-4.4-3.6-8-8-8zm-60 145a3 3 0 01-3 3h-74a3 3 0 01-3-3v-74a3 3 0 013-3h74a3 3 0 013 3v74z"}}]},name:"sliders",theme:"outlined"};const zt=jt;function be(t){for(var e=1;e<arguments.length;e++){var r=arguments[e]!=null?Object(arguments[e]):{},n=Object.keys(r);typeof Object.getOwnPropertySymbols=="function"&&(n=n.concat(Object.getOwnPropertySymbols(r).filter(function(o){return Object.getOwnPropertyDescriptor(r,o).enumerable}))),n.forEach(function(o){Lt(t,o,r[o])})}return t}function Lt(t,e,r){return e in t?Object.defineProperty(t,e,{value:r,enumerable:!0,configurable:!0,writable:!0}):t[e]=r,t}var oe=function(e,r){var n=be({},e,r.attrs);return a(G,be({},n,{icon:zt}),null)};oe.displayName="SlidersOutlined";oe.inheritAttrs=!1;const Mt=oe;var Rt={icon:{tag:"svg",attrs:{viewBox:"64 64 896 896",focusable:"false"},children:[{tag:"path",attrs:{d:"M912 192H328c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h584c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm0 284H328c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h584c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm0 284H328c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h584c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zM104 228a56 56 0 10112 0 56 56 0 10-112 0zm0 284a56 56 0 10112 0 56 56 0 10-112 0zm0 284a56 56 0 10112 0 56 56 0 10-112 0z"}}]},name:"unordered-list",theme:"outlined"};const Et=Rt;function Ce(t){for(var e=1;e<arguments.length;e++){var r=arguments[e]!=null?Object(arguments[e]):{},n=Object.keys(r);typeof Object.getOwnPropertySymbols=="function"&&(n=n.concat(Object.getOwnPropertySymbols(r).filter(function(o){return Object.getOwnPropertyDescriptor(r,o).enumerable}))),n.forEach(function(o){It(t,o,r[o])})}return t}function It(t,e,r){return e in t?Object.defineProperty(t,e,{value:r,enumerable:!0,configurable:!0,writable:!0}):t[e]=r,t}var le=function(e,r){var n=Ce({},e,r.attrs);return a(G,Ce({},n,{icon:Et}),null)};le.displayName="UnorderedListOutlined";le.inheritAttrs=!1;const Dt=le,ke=t=>(Ze("data-v-c85b28f3"),t=t(),Ke(),t),Tt={style:{width:"100%",display:"flex","align-content":"center"}},Ht=ke(()=>H("div",{class:"logo"},null,-1)),Vt={class:"user-avatar-container"},Ut={style:{"margin-left":"1em","font-size":"15px","font-weight":"600"}},Ft=ke(()=>H("div",{class:"header-line"},null,-1)),Wt=k({__name:"MainLayout",setup(t){const e=We(),r=Ge(),n=T(!1),o=d=>{d.key!==r.currentRoute.value.name&&r.push({name:d.key.toString()})},_=()=>{Qe.confirm({icon:a(Xe),title:"您确定要退出登录吗？",onOk:()=>{e.clear(),r.push({name:"login"})},okText:"是的，退出登录",cancelText:"取消"})};return(d,h)=>{const y=lt,s=ot,i=at,l=Ye,m=nt,f=q,g=Be,C=St,w=wt,u=V,p=L,b=Je("router-view"),v=Bt,O=Pt;return E(),I(O,{style:{display:"flex","min-height":"100%",width:"100%"}},{default:c(()=>[a(C,{class:"header"},{default:c(()=>[H("div",Tt,[Ht,a(y,{style:{"margin-top":"0.5em"},level:4,class:"logo-title"},{default:c(()=>[S("阳光小助手")]),_:1})]),a(g,null,{overlay:c(()=>[a(f,null,{default:c(()=>[a(i),a(m,null,{default:c(()=>[a(l,{type:"link",onClick:h[0]||(h[0]=N=>d.$router.push({name:"centerMe"}))},{default:c(()=>[S("个人中心")]),_:1})]),_:1}),a(i),a(m,null,{default:c(()=>[a(l,{type:"link",danger:"",onClick:_},{default:c(()=>[S("退出登录")]),_:1})]),_:1})]),_:1})]),default:c(()=>[H("div",Vt,[a(s,{size:40,class:"user-avatar"},{icon:c(()=>[a(B(ve))]),_:1}),H("span",Ut,de(B(e).username),1)])]),_:1})]),_:1}),Ft,a(O,{"has-sider":""},{default:c(()=>[a(w,{class:"sider",collapsed:n.value,width:"200px",style:{background:"#fff"}},{default:c(()=>[a(f,{onClick:o,class:"sider",mode:"inline",style:{borderRight:0},"selected-keys":[d.$route.name]},{default:c(()=>[a(m,{key:"consoleIndex"},{icon:c(()=>[a(B(Mt))]),default:c(()=>[S(" 控制台首页 ")]),_:1}),a(m,{key:"consoleSunrunTasks"},{icon:c(()=>[a(B(Dt))]),default:c(()=>[S(" 阳光体育任务管理 ")]),_:1}),a(m,{key:"consoleSunrunLogs"},{icon:c(()=>[a(B($t))]),default:c(()=>[S(" 阳光体育任务日志 ")]),_:1}),B(e).isAdmin?(E(),I(m,{key:"consoleUserManage"},{icon:c(()=>[a(B(ve))]),default:c(()=>[S(" 平台用户管理 ")]),_:1})):fe("",!0)]),_:1},8,["selected-keys"]),n.value?(E(),I(l,{key:1,style:{width:"100%"},onClick:h[2]||(h[2]=N=>n.value=!n.value),type:"primary"},{icon:c(()=>[a(B(rt))]),default:c(()=>[S(" 展开")]),_:1})):(E(),I(l,{key:0,style:{width:"100%"},onClick:h[1]||(h[1]=N=>n.value=!n.value),type:"primary"},{icon:c(()=>[a(B(tt))]),default:c(()=>[S(" 收起 ")]),_:1}))]),_:1},8,["collapsed"]),a(O,{style:{padding:"0 24px 24px"}},{default:c(()=>[a(p,{style:{margin:"16px 0"}},{default:c(()=>[a(u,null,{default:c(()=>[S("控制台")]),_:1}),d.$route.meta.title?(E(),I(u,{key:0},{default:c(()=>[S(de(d.$route.meta.title),1)]),_:1})):fe("",!0)]),_:1}),a(v,{style:{background:"#fff",padding:"24px",margin:0,minHeight:"280px"}},{default:c(()=>[a(b)]),_:1})]),_:1})]),_:1})]),_:1})}}});const ar=st(Wt,[["__scopeId","data-v-c85b28f3"]]);export{ar as default};
