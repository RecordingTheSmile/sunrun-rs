import{ab as P,M as z,e as j,ap as U,z as g,r as w,G as S,J as $,s as A,n as D,V as E,C as l,b as r,x as C,aq as W,v as p,P as t}from"./index-b65b5484.js";import{K as x}from"./useMergedState-efc27634.js";import{W as G}from"./index-3d98cbd4.js";import{u as H,o as L}from"./index-829d9efa.js";var q=z("small","default"),J=function(){return{id:String,prefixCls:String,size:t.oneOf(q),disabled:{type:Boolean,default:void 0},checkedChildren:t.any,unCheckedChildren:t.any,tabindex:t.oneOfType([t.string,t.number]),autofocus:{type:Boolean,default:void 0},loading:{type:Boolean,default:void 0},checked:t.oneOfType([t.string,t.number,t.looseBool]),checkedValue:t.oneOfType([t.string,t.number,t.looseBool]).def(!0),unCheckedValue:t.oneOfType([t.string,t.number,t.looseBool]).def(!1),onChange:{type:Function},onClick:{type:Function},onKeydown:{type:Function},onMouseup:{type:Function},"onUpdate:checked":{type:Function},onBlur:Function,onFocus:Function}},R=j({compatConfig:{MODE:3},name:"ASwitch",__ANT_SWITCH:!0,inheritAttrs:!1,props:J(),slots:["checkedChildren","unCheckedChildren"],setup:function(n,s){var d=s.attrs,m=s.slots,B=s.expose,o=s.emit,y=H();U(function(){g(!("defaultChecked"in d),"Switch","'defaultChecked' is deprecated, please use 'v-model:checked'"),g(!("value"in d),"Switch","`value` is not validate prop, do you mean `checked`?")});var h=w(n.checked!==void 0?n.checked:d.defaultChecked),f=S(function(){return h.value===n.checkedValue});$(function(){return n.checked},function(){h.value=n.checked});var v=A("switch",n),u=v.prefixCls,F=v.direction,V=v.size,i=w(),b=function(){var e;(e=i.value)===null||e===void 0||e.focus()},T=function(){var e;(e=i.value)===null||e===void 0||e.blur()};B({focus:b,blur:T}),D(function(){E(function(){n.autofocus&&!n.disabled&&i.value.focus()})});var k=function(e,c){n.disabled||(o("update:checked",e),o("change",e,c),y.onFieldChange())},I=function(e){o("blur",e)},M=function(e){b();var c=f.value?n.unCheckedValue:n.checkedValue;k(c,e),o("click",c,e)},N=function(e){e.keyCode===x.LEFT?k(n.unCheckedValue,e):e.keyCode===x.RIGHT&&k(n.checkedValue,e),o("keydown",e)},O=function(e){var c;(c=i.value)===null||c===void 0||c.blur(),o("mouseup",e)},K=S(function(){var a;return a={},l(a,"".concat(u.value,"-small"),V.value==="small"),l(a,"".concat(u.value,"-loading"),n.loading),l(a,"".concat(u.value,"-checked"),f.value),l(a,"".concat(u.value,"-disabled"),n.disabled),l(a,u.value,!0),l(a,"".concat(u.value,"-rtl"),F.value==="rtl"),a});return function(){var a;return r(G,{insertExtraNode:!0},{default:function(){return[r("button",C(C(C({},L(n,["prefixCls","checkedChildren","unCheckedChildren","checked","autofocus","checkedValue","unCheckedValue","id","onChange","onUpdate:checked"])),d),{},{id:(a=n.id)!==null&&a!==void 0?a:y.id.value,onKeydown:N,onClick:M,onBlur:I,onMouseup:O,type:"button",role:"switch","aria-checked":h.value,disabled:n.disabled||n.loading,class:[d.class,K.value],ref:i}),[r("div",{class:"".concat(u.value,"-handle")},[n.loading?r(W,{class:"".concat(u.value,"-loading-icon")},null):null]),r("span",{class:"".concat(u.value,"-inner")},[f.value?p(m,n,"checkedChildren"):p(m,n,"unCheckedChildren")])])]}})}}});const ee=P(R);export{ee as _};
