import{e as z,I as A,aU as V,V as j,aA as k,aF as $,B as P,b as s,_ as B,C as r,x as b,aY as T,P as x}from"./index-b65b5484.js";import{d as F}from"./index-c9997867.js";import{i as M}from"./_plugin-vue_export-helper-8f6a5dc0.js";var O=["class","style"],W=function(){return{prefixCls:String,spinning:{type:Boolean,default:void 0},size:String,wrapperClassName:String,tip:x.any,delay:Number,indicator:x.any}},p=null;function Y(t,n){return!!t&&!!n&&!isNaN(Number(n))}function q(t){var n=t.indicator;p=typeof n=="function"?n:function(){return s(n,null,null)}}const d=z({compatConfig:{MODE:3},name:"ASpin",inheritAttrs:!1,props:M(W(),{size:"default",spinning:!0,wrapperClassName:""}),setup:function(){return{originalUpdateSpinning:null,configProvider:A("configProvider",V)}},data:function(){var n=this.spinning,e=this.delay,i=Y(n,e);return{sSpinning:n&&!i}},created:function(){this.originalUpdateSpinning=this.updateSpinning,this.debouncifyUpdateSpinning(this.$props)},mounted:function(){this.updateSpinning()},updated:function(){var n=this;j(function(){n.debouncifyUpdateSpinning(),n.updateSpinning()})},beforeUnmount:function(){this.cancelExistingSpin()},methods:{debouncifyUpdateSpinning:function(n){var e=n||this.$props,i=e.delay;i&&(this.cancelExistingSpin(),this.updateSpinning=F(this.originalUpdateSpinning,i))},updateSpinning:function(){var n=this.spinning,e=this.sSpinning;e!==n&&(this.sSpinning=n)},cancelExistingSpin:function(){var n=this.updateSpinning;n&&n.cancel&&n.cancel()},renderIndicator:function(n){var e="".concat(n,"-dot"),i=k(this,"indicator");return i===null?null:(Array.isArray(i)&&(i=i.length===1?i[0]:i),$(i)?P(i,{class:e}):p&&$(p())?P(p(),{class:e}):s("span",{class:"".concat(e," ").concat(n,"-dot-spin")},[s("i",{class:"".concat(n,"-dot-item")},null),s("i",{class:"".concat(n,"-dot-item")},null),s("i",{class:"".concat(n,"-dot-item")},null),s("i",{class:"".concat(n,"-dot-item")},null)]))}},render:function(){var n,e,i,o=this.$props,h=o.size,N=o.prefixCls,v=o.tip,l=v===void 0?(n=(e=this.$slots).tip)===null||n===void 0?void 0:n.call(e):v,_=o.wrapperClassName,u=this.$attrs,S=u.class,C=u.style,U=B(u,O),m=this.configProvider,I=m.getPrefixCls,D=m.direction,a=I("spin",N),f=this.sSpinning,E=(i={},r(i,a,!0),r(i,"".concat(a,"-sm"),h==="small"),r(i,"".concat(a,"-lg"),h==="large"),r(i,"".concat(a,"-spinning"),f),r(i,"".concat(a,"-show-text"),!!l),r(i,"".concat(a,"-rtl"),D==="rtl"),r(i,S,!!S),i),y=s("div",b(b({},U),{},{style:C,class:E}),[this.renderIndicator(a),l?s("div",{class:"".concat(a,"-text")},[l]):null]),g=T(this);if(g&&g.length){var c,w=(c={},r(c,"".concat(a,"-container"),!0),r(c,"".concat(a,"-blur"),f),c);return s("div",{class:["".concat(a,"-nested-loading"),_]},[f&&s("div",{key:"loading"},[y]),s("div",{class:w,key:"container"},[g])])}return y}});d.setDefaultIndicator=q;d.install=function(t){return t.component(d.name,d),t};export{d as S};