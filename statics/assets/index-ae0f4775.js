import{ab as Q,M as q,e as z,s as G,r as m,v as g,L as J,C as t,b as o,a2 as K,af as R,ag as U,ah as W,ai as X,x as F,aj as Y,P as c,aa as Z,ak as ee,a6 as ae,al as ne,am as oe,an as le,ao as te,Q as se}from"./index-b65b5484.js";import{c as ce}from"./index-829d9efa.js";var ie={success:Z,info:ee,error:ae,warning:ne},re={success:oe,info:le,error:te,warning:se},ue=q("success","info","warning","error"),de=function(){return{type:c.oneOf(ue),closable:{type:Boolean,default:void 0},closeText:c.any,message:c.any,description:c.any,afterClose:Function,showIcon:{type:Boolean,default:void 0},prefixCls:String,banner:{type:Boolean,default:void 0},icon:c.any,closeIcon:c.any,onClose:Function}},ve=z({compatConfig:{MODE:3},name:"AAlert",inheritAttrs:!1,props:de(),setup:function(n,v){var s=v.slots,L=v.emit,h=v.attrs,B=v.expose,I=G("alert",n),H=I.prefixCls,N=I.direction,i=m(!1),b=m(!1),w=m(),S=function(a){a.preventDefault();var l=w.value;l.style.height="".concat(l.offsetHeight,"px"),l.style.height="".concat(l.offsetHeight,"px"),i.value=!0,L("close",a)},_=function(){var a;i.value=!1,b.value=!0,(a=n.afterClose)===null||a===void 0||a.call(n)};B({animationEnd:_});var j=m({});return function(){var r,a,l=n.banner,E=n.closeIcon,O=E===void 0?(r=s.closeIcon)===null||r===void 0?void 0:r.call(s):E,y=n.closable,u=n.type,d=n.showIcon,x=g(s,n,"closeText"),f=g(s,n,"description"),P=g(s,n,"message"),p=g(s,n,"icon");d=l&&d===void 0?!0:d,u=l&&u===void 0?"warning":u||"info";var k=(f?re:ie)[u]||null;x&&(y=!0);var e=H.value,M=J(e,(a={},t(a,"".concat(e,"-").concat(u),!0),t(a,"".concat(e,"-closing"),i.value),t(a,"".concat(e,"-with-description"),!!f),t(a,"".concat(e,"-no-icon"),!d),t(a,"".concat(e,"-banner"),!!l),t(a,"".concat(e,"-closable"),y),t(a,"".concat(e,"-rtl"),N.value==="rtl"),a)),D=y?o("button",{type:"button",onClick:S,class:"".concat(e,"-close-icon"),tabindex:0},[x?o("span",{class:"".concat(e,"-close-text")},[x]):O===void 0?o(K,null,null):O]):null,V=p&&(R(p)?ce(p,{class:"".concat(e,"-icon")}):o("span",{class:"".concat(e,"-icon")},[p]))||o(k,{class:"".concat(e,"-icon")},null),$=U("".concat(e,"-motion"),{appear:!1,css:!0,onAfterLeave:_,onBeforeLeave:function(C){C.style.maxHeight="".concat(C.offsetHeight,"px")},onLeave:function(C){C.style.maxHeight="0px"}});return b.value?null:o(Y,$,{default:function(){return[W(o("div",F(F({role:"alert"},h),{},{style:[h.style,j.value],class:[h.class,M],"data-show":!i.value,ref:w}),[d?V:null,o("div",{class:"".concat(e,"-content")},[P?o("div",{class:"".concat(e,"-message")},[P]):null,f?o("div",{class:"".concat(e,"-description")},[f]):null]),D]),[[X,!i.value]])]}})}}});const Ce=Q(ve);export{Ce as _};
