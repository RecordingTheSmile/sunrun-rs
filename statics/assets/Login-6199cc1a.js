import{e as k,r as g,f as I,u as B,g as S,h as q,o as v,c as C,a as d,b as t,w as s,i as F,j as L,d as c,m as N,B as U,p as V,k as z}from"./index-ead633b2.js";import{_ as E,F as R}from"./index-5cb42d46.js";import{I as T}from"./index-88f2ff4c.js";import{_ as j}from"./index-4b0dfc7e.js";import{r as A}from"./index-c4ebfd02.js";import{_ as M}from"./index-6e6a2308.js";import{_ as P}from"./_plugin-vue_export-helper-c27b6911.js";import"./collapseMotion-090e2670.js";import"./useFlexGapSupport-e64096c7.js";import"./useSize-108b0685.js";const Q=u=>(V("data-v-951473e9"),u=u(),z(),u),D={class:"outer"},G={class:"main"},H={class:"form-container"},J=Q(()=>d("h1",{style:{"text-align":"center","margin-bottom":"40px","padding-top":"20px"}},"登录",-1)),K=k({__name:"Login",setup(u){const n=g(""),m=g(!1),r=I({username:"",password:""}),h=B(),i=S(),y=_=>(m.value=!0,n.value="",A.post("/user/api/login",_).then(e=>{if(!e.data.success){const a=e.data.message??"登录失败，请重试";n.value=a;return}h.setUserInfo(_.username,e.data.data.email,e.data.data.token,e.data.data.isAdmin),N.success("登录成功");const o=i.currentRoute.value.query.from;o?i.push(o):i.push({name:"consoleIndex"})}).catch(e=>{var a;const o=((a=e.response)==null?void 0:a.data).message??"登录失败，请重试";n.value=o}).finally(()=>{m.value=!1}));return(_,e)=>{const o=j,a=T,l=E,x=M,f=q("router-link"),w=U,b=R;return v(),C("div",D,[d("div",G,[d("div",H,[J,t(b,{"label-col":{style:{width:"6em"}},class:"user-layout-login",model:r,autocomplete:!1,onFinish:y},{default:s(()=>[n.value!==""?(v(),F(o,{key:0,type:"error",showIcon:"",style:{"margin-bottom":"24px"},message:n.value},null,8,["message"])):L("",!0),t(l,{rules:[{required:!0,message:"请输入用户名"}],label:"用户名",name:"username"},{default:s(()=>[t(a,{value:r.username,"onUpdate:value":e[0]||(e[0]=p=>r.username=p),size:"large",type:"text",placeholder:"请输入用户名",rules:[]},null,8,["value"])]),_:1}),t(l,{label:"密码",name:"password",rules:[{required:!0,message:"请输入密码"}]},{default:s(()=>[t(x,{size:"large",placeholder:"请输入密码",value:r.password,"onUpdate:value":e[1]||(e[1]=p=>r.password=p)},null,8,["value"])]),_:1}),t(l,null,{default:s(()=>[t(f,{to:{name:"forgetPassword"},class:"forge-password",style:{float:"right"}},{default:s(()=>[c("忘记密码")]),_:1})]),_:1}),t(l,{style:{"margin-top":"24px"}},{default:s(()=>[t(w,{size:"large",type:"primary",htmlType:"submit",class:"login-button",loading:m.value,disabled:m.value},{default:s(()=>[c("登录")]),_:1},8,["loading","disabled"])]),_:1}),t(l,null,{default:s(()=>[t(f,{style:{float:"right"},type:"link",to:{name:"register"}},{default:s(()=>[c("去注册 →")]),_:1})]),_:1})]),_:1},8,["model"])])])])}}});const ae=P(K,[["__scopeId","data-v-951473e9"]]);export{ae as default};