import{e as N,r as m,n as w,c,a as g,b as s,w as n,m as i,o as l,d as u,F as k,k as f,t as z}from"./index-b65b5484.js";import{b as B,c as D}from"./index-fd6eec2a.js";import"./index-a2b98079.js";import"./index-3971b52c.js";import"./index-829d9efa.js";import{_ as T}from"./index-42da564f.js";import{_ as A}from"./index-c4c5f765.js";import{r as h}from"./index-c9997867.js";import{B as V}from"./index-3d98cbd4.js";import{_ as F}from"./_plugin-vue_export-helper-8f6a5dc0.js";import"./useMergedState-efc27634.js";import"./ActionButton-974fa8a9.js";import"./eagerComputed-85e21e12.js";import"./collapseMotion-09a61311.js";import"./css-aae66f86.js";import"./index-9c362c9d.js";import"./useFlexGapSupport-9980977a.js";const q={class:"table-container"},E={style:{width:"100%","text-align":"center"}},M={style:{width:"100%"},class:"table-opr"},$={style:{width:"100%"}},j={key:0,style:{color:"green"}},G={key:1,style:{color:"red"}},H=N({__name:"SunrunTaskLogs",setup(J){const a=m(!1),o=m({current:1,pageSize:50,total:0}),x=(e,r,t)=>{_(e.current??1,e.pageSize??50)},S=[{dataIndex:"id",key:"id",title:"日志ID"},{dataIndex:"createAt",key:"createAt",title:"记录于"},{dataIndex:"description",key:"description",title:"描述"},{dataIndex:"isSuccess",key:"isSuccess",title:"是否成功"},{dataIndex:"nickName",key:"nickName",title:"用户名"},{dataIndex:"schoolName",key:"schoolName",title:"学校名"}],y=m(),_=(e,r)=>(a.value=!0,h.get("/task/api/taskLog",{params:{page:e,limit:r}}).then(t=>{t.data.success?(o.value.total=t.data.data.total,y.value=t.data.data.rows):i.error(t.data.message??"请求数据失败，请刷新表格重试")}).catch(t=>{console.error(t),i.error("网络错误，请刷新表格重试")}).finally(()=>{a.value=!1})),b=()=>(a.value=!0,h.delete("/task/api/taskLog").then(e=>{if(a.value=!1,e.data.success)return i.success("删除成功"),_(o.value.current,o.value.pageSize);i.error(e.data.message??"删除任务日志失败，请重试")}).catch(e=>{a.value=!1,console.error(e),i.error("网络错误，请重试")}));return w(()=>{_(o.value.current,o.value.pageSize)}),(e,r)=>{const t=A,v=V,I=D,C=T,L=B;return l(),c("div",q,[g("div",E,[s(t,{level:3},{default:n(()=>[u("阳光体育任务日志")]),_:1})]),g("div",M,[s(C,{size:8},{default:n(()=>[s(v,{type:"primary",loading:a.value,onClick:r[0]||(r[0]=d=>_(o.value.current,o.value.pageSize))},{default:n(()=>[u("刷新表格")]),_:1},8,["loading"]),s(I,{title:"您确定要清空所有日志吗？","ok-text":"是的","cancel-text":"取消",onConfirm:b},{default:n(()=>[s(v,{type:"primary",loading:a.value,danger:""},{default:n(()=>[u("清空所有日志")]),_:1},8,["loading"])]),_:1})]),_:1})]),g("div",$,[s(L,{loading:a.value,columns:S,"data-source":y.value,scroll:{x:!0},pagination:o.value,onChange:x},{bodyCell:n(({column:d,record:p})=>[d.key==="isSuccess"?(l(),c(k,{key:0},[p.isSuccess?(l(),c("span",j,"成功")):f("",!0),p.isSuccess?f("",!0):(l(),c("span",G,"失败"))],64)):d.key==="createAt"?(l(),c(k,{key:1},[u(z(new Date(p.createAt*1e3).toLocaleString()),1)],64)):f("",!0)]),_:1},8,["loading","data-source","pagination"])])])}}});const ce=F(H,[["__scopeId","data-v-87e2a7b9"]]);export{ce as default};