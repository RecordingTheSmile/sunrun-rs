import{e as k,r as g,f as I,u as B,g as S,h as q,c as C,a as c,b as t,w as a,p as F,i as L,o as v,j as N,k as U,d}from"./index-b65b5484.js";import{F as V,_ as z}from"./index-45abc488.js";import{I as E}from"./index-829d9efa.js";import{_ as R}from"./index-ae0f4775.js";import{r as T}from"./index-c9997867.js";import{_ as j}from"./index-9c362c9d.js";import{B as A}from"./index-3d98cbd4.js";import{_ as M}from"./_plugin-vue_export-helper-8f6a5dc0.js";import"./collapseMotion-09a61311.js";import"./useFlexGapSupport-9980977a.js";import"./useSize-9ecf35dc.js";const P=u=>(F("data-v-102aa830"),u=u(),L(),u),Q={class:"outer"},D={class:"main"},G={class:"form-container"},H=P(()=>c("h1",{style:{"text-align":"center","margin-bottom":"40px","padding-top":"20px"}},"登录",-1)),J=k({__name:"Login",setup(u){const n=g(""),m=g(!1),r=I({username:"",password:""}),h=B(),i=S(),y=p=>(m.value=!0,n.value="",T.post("/user/api/login",p).then(e=>{if(!e.data.success){const s=e.data.message??"登录失败，请重试";n.value=s;return}h.setUserInfo(p.username,e.data.data.email,e.data.data.token,e.data.data.isAdmin);const o=i.currentRoute.value.query.from;o?i.push(o):i.push({name:"consoleIndex"})}).catch(e=>{var s;const o=((s=e.response)==null?void 0:s.data).message??"登录失败，请重试";n.value=o}).finally(()=>{m.value=!1}));return(p,e)=>{const o=R,s=E,l=z,x=j,f=q("router-link"),w=A,b=V;return v(),C("div",Q,[c("div",D,[c("div",G,[H,t(b,{"label-col":{style:{width:"6em"}},class:"user-layout-login",model:r,autocomplete:!1,onFinish:y},{default:a(()=>[n.value!==""?(v(),N(o,{key:0,type:"error",showIcon:"",style:{"margin-bottom":"24px"},message:n.value},null,8,["message"])):U("",!0),t(l,{rules:[{required:!0,message:"请输入用户名"}],label:"用户名",name:"username"},{default:a(()=>[t(s,{value:r.username,"onUpdate:value":e[0]||(e[0]=_=>r.username=_),size:"large",type:"text",placeholder:"请输入用户名",rules:[]},null,8,["value"])]),_:1}),t(l,{label:"密码",name:"password",rules:[{required:!0,message:"请输入密码"}]},{default:a(()=>[t(x,{size:"large",placeholder:"请输入密码",value:r.password,"onUpdate:value":e[1]||(e[1]=_=>r.password=_)},null,8,["value"])]),_:1}),t(l,null,{default:a(()=>[t(f,{to:{name:"forgetPassword"},class:"forge-password",style:{float:"right"}},{default:a(()=>[d("忘记密码")]),_:1})]),_:1}),t(l,{style:{"margin-top":"24px"}},{default:a(()=>[t(w,{size:"large",type:"primary",htmlType:"submit",class:"login-button",loading:m.value,disabled:m.value},{default:a(()=>[d("登录")]),_:1},8,["loading","disabled"])]),_:1}),t(l,null,{default:a(()=>[t(f,{style:{float:"right"},type:"link",to:{name:"register"}},{default:a(()=>[d("去注册 →")]),_:1})]),_:1})]),_:1},8,["model"])])])])}}});const se=M(J,[["__scopeId","data-v-102aa830"]]);export{se as default};
