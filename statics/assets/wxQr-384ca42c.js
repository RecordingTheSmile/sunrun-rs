import{e as Q,r as c,g as C,n as I,l as U,o,c as n,a as s,b as r,w as u,d as m,q as B,t as k,m as N,p as R,k as T}from"./index-ead633b2.js";import{S as V}from"./index-71caf399.js";import{_ as z}from"./index-8410ae47.js";import"./index-88f2ff4c.js";import{_ as L}from"./index-0fd9cccf.js";import{r as w}from"./index-c4ebfd02.js";import{_ as M}from"./Paragraph-ea8c7c28.js";import"./index-5fb99269.js";import{_ as D}from"./_plugin-vue_export-helper-c27b6911.js";import"./useMergedState-426bca96.js";const E=l=>(R("data-v-d6f89c40"),l=l(),T(),l),W={class:"outer"},$={class:"main"},j={key:0,class:"form-container"},A={key:1,class:"form-container"},F={style:{"text-align":"center","margin-bottom":"40px","padding-top":"20px"}},G={key:0,style:{"text-align":"center"}},H={style:{"text-align":"center","margin-top":"20px","padding-bottom":"20px"}},J={key:2,class:"form-container"},K={class:"desc"},O=E(()=>s("p",null,"扫描二维码时发生错误，请刷新页面重试，或联系管理员重新获取二维码扫描网址！",-1)),P=Q({__name:"wxQr",setup(l){const f=c(""),g=c(!1),_=c(!0),t=c(""),d=C().currentRoute.value.query.token;let p="";const v=c("请使用手机微信扫描二维码");let a=!0;const S=()=>{if(!d){t.value="二维码请求非法，请使用正确的扫码URL";return}return _.value=!0,p="",w.get("/public/api/wxUuid",{params:{grantCode:d+""}}).then(e=>{if(e.data.success)p=e.data.data.uuid,f.value=e.data.data.url,setTimeout(()=>{a&&h()},2e3);else{const i=e.data.message??"获取二维码失败，未知错误";t.value=i}}).catch(e=>{console.error(e),t.value="网络错误导致二维码获取失败"}).finally(()=>{_.value=!1})},h=()=>w.get("/public/api/wxStatus",{params:{grantCode:d,uuid:p}}).then(e=>{if(e.data.success)switch(e.data.code){case 201:case 202:v.value=e.data.message,setTimeout(()=>{a&&h()},2e3);break;case 200:g.value=!0,a=!1}else{const i=e.data.message??"二维码已失效，请联系管理员重新获取";t.value=i,a=!1}}).catch(e=>{console.error(e),N.error("网络发生错误"),a=!1});return I(()=>{S()}),U(()=>{a=!1}),(e,i)=>{const x=L,y=z,b=M,q=V;return o(),n("div",W,[s("div",$,[g.value?(o(),n("div",j,[r(x,{status:"success",title:"登录成功","sub-title":"请联系管理员以确认登录是否完成！"})])):t.value===""?(o(),n("div",A,[s("div",F,[r(y,{level:3},{default:u(()=>[m("微信扫码登录")]),_:1})]),s("div",null,[r(b,{strong:"",style:{color:"red","text-align":"center"}},{default:u(()=>[m("不要截图！截图扫码无效！请使用其他设备展示该二维码后用手机扫描！")]),_:1})]),_.value?(o(),n("div",G,[r(q,{size:"large",tip:"正在加载二维码，请稍后",style:{margin:"auto"}})])):(o(),n("div",{key:1,class:"wx-qr",style:B({"--wx-qr-url":`url(${f.value})`})},null,4)),s("div",H,[r(y,{level:4},{default:u(()=>[m(k(v.value),1)]),_:1})])])):(o(),n("div",J,[r(x,{status:"error",title:"二维码扫描出错了","sub-title":t.value},{default:u(()=>[s("div",K,[O,s("strong",null,"错误原因："+k(t.value),1)])]),_:1},8,["sub-title"])]))])])}}});const le=D(P,[["__scopeId","data-v-d6f89c40"]]);export{le as default};
