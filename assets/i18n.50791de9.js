import{h as $,r as G,c as B,w as Fe,F as xe,i as kt,g as $e,a as pt,o as wt,b as St,d as ve,T as ke,e as Ot}from"./index.29b302c5.js";/*!
  * @intlify/shared v9.1.10
  * (c) 2022 kazuya kawaguchi
  * Released under the MIT License.
  */const Mt=typeof Symbol=="function"&&typeof Symbol.toStringTag=="symbol",D=e=>Mt?Symbol(e):e,It=(e,t,n)=>Lt({l:e,k:t,s:n}),Lt=e=>JSON.stringify(e).replace(/\u2028/g,"\\u2028").replace(/\u2029/g,"\\u2029").replace(/\u0027/g,"\\u0027"),O=e=>typeof e=="number"&&isFinite(e),Wt=e=>fe(e)==="[object Date]",X=e=>fe(e)==="[object RegExp]",ne=e=>y(e)&&Object.keys(e).length===0;function Ct(e,t){typeof console!="undefined"&&(console.warn("[intlify] "+e),t&&console.warn(t.stack))}const C=Object.assign;function pe(e){return e.replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&apos;")}const Tt=Object.prototype.hasOwnProperty;function Ue(e,t){return Tt.call(e,t)}const M=Array.isArray,L=e=>typeof e=="function",g=e=>typeof e=="string",S=e=>typeof e=="boolean",W=e=>e!==null&&typeof e=="object",He=Object.prototype.toString,fe=e=>He.call(e),y=e=>fe(e)==="[object Object]",Et=e=>e==null?"":M(e)||y(e)&&e.toString===He?JSON.stringify(e,null,2):String(e);/*!
  * @intlify/message-resolver v9.1.10
  * (c) 2022 kazuya kawaguchi
  * Released under the MIT License.
  */const Pt=Object.prototype.hasOwnProperty;function Nt(e,t){return Pt.call(e,t)}const Q=e=>e!==null&&typeof e=="object",R=[];R[0]={w:[0],i:[3,0],["["]:[4],o:[7]};R[1]={w:[1],["."]:[2],["["]:[4],o:[7]};R[2]={w:[2],i:[3,0],[0]:[3,0]};R[3]={i:[3,0],[0]:[3,0],w:[1,1],["."]:[2,1],["["]:[4,1],o:[7,1]};R[4]={["'"]:[5,0],['"']:[6,0],["["]:[4,2],["]"]:[1,3],o:8,l:[4,0]};R[5]={["'"]:[4,0],o:8,l:[5,0]};R[6]={['"']:[4,0],o:8,l:[6,0]};const Dt=/^\s?(?:true|false|-?[\d.]+|'[^']*'|"[^"]*")\s?$/;function Rt(e){return Dt.test(e)}function jt(e){const t=e.charCodeAt(0),n=e.charCodeAt(e.length-1);return t===n&&(t===34||t===39)?e.slice(1,-1):e}function At(e){if(e==null)return"o";switch(e.charCodeAt(0)){case 91:case 93:case 46:case 34:case 39:return e;case 95:case 36:case 45:return"i";case 9:case 10:case 13:case 160:case 65279:case 8232:case 8233:return"w"}return"i"}function xt(e){const t=e.trim();return e.charAt(0)==="0"&&isNaN(parseInt(e))?!1:Rt(t)?jt(t):"*"+t}function $t(e){const t=[];let n=-1,a=0,r=0,l,s,o,u,m,_,h;const b=[];b[0]=()=>{s===void 0?s=o:s+=o},b[1]=()=>{s!==void 0&&(t.push(s),s=void 0)},b[2]=()=>{b[0](),r++},b[3]=()=>{if(r>0)r--,a=4,b[0]();else{if(r=0,s===void 0||(s=xt(s),s===!1))return!1;b[1]()}};function v(){const k=e[n+1];if(a===5&&k==="'"||a===6&&k==='"')return n++,o="\\"+k,b[0](),!0}for(;a!==null;)if(n++,l=e[n],!(l==="\\"&&v())){if(u=At(l),h=R[a],m=h[u]||h.l||8,m===8||(a=m[0],m[1]!==void 0&&(_=b[m[1]],_&&(o=l,_()===!1))))return;if(a===7)return t}}const we=new Map;function ee(e,t){if(!Q(e))return null;let n=we.get(t);if(n||(n=$t(t),n&&we.set(t,n)),!n)return null;const a=n.length;let r=e,l=0;for(;l<a;){const s=r[n[l]];if(s===void 0)return null;r=s,l++}return r}function le(e){if(!Q(e))return e;for(const t in e)if(!!Nt(e,t))if(!t.includes("."))Q(e[t])&&le(e[t]);else{const n=t.split("."),a=n.length-1;let r=e;for(let l=0;l<a;l++)n[l]in r||(r[n[l]]={}),r=r[n[l]];r[n[a]]=e[t],delete e[t],Q(r[n[a]])&&le(r[n[a]])}return e}/*!
  * @intlify/runtime v9.1.10
  * (c) 2022 kazuya kawaguchi
  * Released under the MIT License.
  */const Ut=e=>e,Ht=e=>"",Vt="text",Gt=e=>e.length===0?"":e.join(""),Bt=Et;function Se(e,t){return e=Math.abs(e),t===2?e?e>1?1:0:1:e?Math.min(e,2):0}function zt(e){const t=O(e.pluralIndex)?e.pluralIndex:-1;return e.named&&(O(e.named.count)||O(e.named.n))?O(e.named.count)?e.named.count:O(e.named.n)?e.named.n:t:t}function Jt(e,t){t.count||(t.count=e),t.n||(t.n=e)}function Kt(e={}){const t=e.locale,n=zt(e),a=W(e.pluralRules)&&g(t)&&L(e.pluralRules[t])?e.pluralRules[t]:Se,r=W(e.pluralRules)&&g(t)&&L(e.pluralRules[t])?Se:void 0,l=d=>d[a(n,d.length,r)],s=e.list||[],o=d=>s[d],u=e.named||{};O(e.pluralIndex)&&Jt(n,u);const m=d=>u[d];function _(d){const F=L(e.messages)?e.messages(d):W(e.messages)?e.messages[d]:!1;return F||(e.parent?e.parent.message(d):Ht)}const h=d=>e.modifiers?e.modifiers[d]:Ut,b=y(e.processor)&&L(e.processor.normalize)?e.processor.normalize:Gt,v=y(e.processor)&&L(e.processor.interpolate)?e.processor.interpolate:Bt,k=y(e.processor)&&g(e.processor.type)?e.processor.type:Vt,p={list:o,named:m,plural:l,linked:(d,F)=>{const T=_(d)(p);return g(F)?h(F)(T):T},message:_,type:k,interpolate:v,normalize:b};return p}/*!
  * @intlify/message-compiler v9.1.10
  * (c) 2022 kazuya kawaguchi
  * Released under the MIT License.
  */function Ve(e,t,n={}){const{domain:a,messages:r,args:l}=n,s=e,o=new SyntaxError(String(s));return o.code=e,t&&(o.location=t),o.domain=a,o}/*!
  * @intlify/devtools-if v9.1.10
  * (c) 2022 kazuya kawaguchi
  * Released under the MIT License.
  */const Yt={I18nInit:"i18n:init",FunctionTranslate:"function:translate"};/*!
  * @intlify/core-base v9.1.10
  * (c) 2022 kazuya kawaguchi
  * Released under the MIT License.
  */let qt=null;Yt.FunctionTranslate;function Zt(e){return t=>qt}const Qt="9.1.10",ae=-1,Oe="";function Xt(){return{upper:e=>g(e)?e.toUpperCase():e,lower:e=>g(e)?e.toLowerCase():e,capitalize:e=>g(e)?`${e.charAt(0).toLocaleUpperCase()}${e.substr(1)}`:e}}let en,Me=0;function tn(e={}){const t=g(e.version)?e.version:Qt,n=g(e.locale)?e.locale:"en-US",a=M(e.fallbackLocale)||y(e.fallbackLocale)||g(e.fallbackLocale)||e.fallbackLocale===!1?e.fallbackLocale:n,r=y(e.messages)?e.messages:{[n]:{}},l=y(e.datetimeFormats)?e.datetimeFormats:{[n]:{}},s=y(e.numberFormats)?e.numberFormats:{[n]:{}},o=C({},e.modifiers||{},Xt()),u=e.pluralRules||{},m=L(e.missing)?e.missing:null,_=S(e.missingWarn)||X(e.missingWarn)?e.missingWarn:!0,h=S(e.fallbackWarn)||X(e.fallbackWarn)?e.fallbackWarn:!0,b=!!e.fallbackFormat,v=!!e.unresolving,k=L(e.postTranslation)?e.postTranslation:null,p=y(e.processor)?e.processor:null,d=S(e.warnHtmlMessage)?e.warnHtmlMessage:!0,F=!!e.escapeParameter,T=L(e.messageCompiler)?e.messageCompiler:en,w=L(e.onWarn)?e.onWarn:Ct,f=e,E=W(f.__datetimeFormatters)?f.__datetimeFormatters:new Map,U=W(f.__numberFormatters)?f.__numberFormatters:new Map,H=W(f.__meta)?f.__meta:{};return Me++,{version:t,cid:Me,locale:n,fallbackLocale:a,messages:r,datetimeFormats:l,numberFormats:s,modifiers:o,pluralRules:u,missing:m,missingWarn:_,fallbackWarn:h,fallbackFormat:b,unresolving:v,postTranslation:k,processor:p,warnHtmlMessage:d,escapeParameter:F,messageCompiler:T,onWarn:w,__datetimeFormatters:E,__numberFormatters:U,__meta:H}}function ge(e,t,n,a,r){const{missing:l,onWarn:s}=e;if(l!==null){const o=l(e,n,t,r);return g(o)?o:t}else return t}function J(e,t,n){const a=e;a.__localeChainCache||(a.__localeChainCache=new Map);let r=a.__localeChainCache.get(n);if(!r){r=[];let l=[n];for(;M(l);)l=Ie(r,l,t);const s=M(t)?t:y(t)?t.default?t.default:null:t;l=g(s)?[s]:s,M(l)&&Ie(r,l,!1),a.__localeChainCache.set(n,r)}return r}function Ie(e,t,n){let a=!0;for(let r=0;r<t.length&&S(a);r++){const l=t[r];g(l)&&(a=nn(e,t[r],n))}return a}function nn(e,t,n){let a;const r=t.split("-");do{const l=r.join("-");a=an(e,l,n),r.splice(-1,1)}while(r.length&&a===!0);return a}function an(e,t,n){let a=!1;if(!e.includes(t)&&(a=!0,t)){a=t[t.length-1]!=="!";const r=t.replace(/!/g,"");e.push(r),(M(n)||y(n))&&n[r]&&(a=n[r])}return a}function z(e,t,n){const a=e;a.__localeChainCache=new Map,J(e,n,t)}function x(e){return Ve(e,null,void 0)}const Le=()=>"",j=e=>L(e);function We(e,...t){const{fallbackFormat:n,postTranslation:a,unresolving:r,fallbackLocale:l,messages:s}=e,[o,u]=se(...t),m=S(u.missingWarn)?u.missingWarn:e.missingWarn,_=S(u.fallbackWarn)?u.fallbackWarn:e.fallbackWarn,h=S(u.escapeParameter)?u.escapeParameter:e.escapeParameter,b=!!u.resolvedMessage,v=g(u.default)||S(u.default)?S(u.default)?o:u.default:n?o:"",k=n||v!=="",p=g(u.locale)?u.locale:e.locale;h&&rn(u);let[d,F,T]=b?[o,p,s[p]||{}]:ln(e,o,p,l,_,m),w=o;if(!b&&!(g(d)||j(d))&&k&&(d=v,w=d),!b&&(!(g(d)||j(d))||!g(F)))return r?ae:o;let f=!1;const E=()=>{f=!0},U=j(d)?d:Ge(e,o,F,d,w,E);if(f)return d;const H=on(e,F,T,u),K=Kt(H),Y=sn(e,U,K);return a?a(Y):Y}function rn(e){M(e.list)?e.list=e.list.map(t=>g(t)?pe(t):t):W(e.named)&&Object.keys(e.named).forEach(t=>{g(e.named[t])&&(e.named[t]=pe(e.named[t]))})}function ln(e,t,n,a,r,l){const{messages:s,onWarn:o}=e,u=J(e,a,n);let m={},_,h=null;const b="translate";for(let v=0;v<u.length&&(_=u[v],m=s[_]||{},(h=ee(m,t))===null&&(h=m[t]),!(g(h)||L(h)));v++){const k=ge(e,t,_,l,b);k!==t&&(h=k)}return[h,_,m]}function Ge(e,t,n,a,r,l){const{messageCompiler:s,warnHtmlMessage:o}=e;if(j(a)){const m=a;return m.locale=m.locale||n,m.key=m.key||t,m}const u=s(a,cn(e,n,r,a,o,l));return u.locale=n,u.key=t,u.source=a,u}function sn(e,t,n){return t(n)}function se(...e){const[t,n,a]=e,r={};if(!g(t)&&!O(t)&&!j(t))throw x(14);const l=O(t)?String(t):(j(t),t);return O(n)?r.plural=n:g(n)?r.default=n:y(n)&&!ne(n)?r.named=n:M(n)&&(r.list=n),O(a)?r.plural=a:g(a)?r.default=a:y(a)&&C(r,a),[l,r]}function cn(e,t,n,a,r,l){return{warnHtmlMessage:r,onError:s=>{throw l&&l(s),s},onCacheKey:s=>It(t,n,s)}}function on(e,t,n,a){const{modifiers:r,pluralRules:l}=e,o={locale:t,modifiers:r,pluralRules:l,messages:u=>{const m=ee(n,u);if(g(m)){let _=!1;const b=Ge(e,u,t,m,u,()=>{_=!0});return _?Le:b}else return j(m)?m:Le}};return e.processor&&(o.processor=e.processor),a.list&&(o.list=a.list),a.named&&(o.named=a.named),O(a.plural)&&(o.pluralIndex=a.plural),o}function Ce(e,...t){const{datetimeFormats:n,unresolving:a,fallbackLocale:r,onWarn:l}=e,{__datetimeFormatters:s}=e,[o,u,m,_]=ce(...t),h=S(m.missingWarn)?m.missingWarn:e.missingWarn;S(m.fallbackWarn)?m.fallbackWarn:e.fallbackWarn;const b=!!m.part,v=g(m.locale)?m.locale:e.locale,k=J(e,r,v);if(!g(o)||o==="")return new Intl.DateTimeFormat(v).format(u);let p={},d,F=null;const T="datetime format";for(let E=0;E<k.length&&(d=k[E],p=n[d]||{},F=p[o],!y(F));E++)ge(e,o,d,h,T);if(!y(F)||!g(d))return a?ae:o;let w=`${d}__${o}`;ne(_)||(w=`${w}__${JSON.stringify(_)}`);let f=s.get(w);return f||(f=new Intl.DateTimeFormat(d,C({},F,_)),s.set(w,f)),b?f.formatToParts(u):f.format(u)}function ce(...e){const[t,n,a,r]=e;let l={},s={},o;if(g(t)){if(!/\d{4}-\d{2}-\d{2}(T.*)?/.test(t))throw x(16);o=new Date(t);try{o.toISOString()}catch{throw x(16)}}else if(Wt(t)){if(isNaN(t.getTime()))throw x(15);o=t}else if(O(t))o=t;else throw x(14);return g(n)?l.key=n:y(n)&&(l=n),g(a)?l.locale=a:y(a)&&(s=a),y(r)&&(s=r),[l.key||"",o,l,s]}function Te(e,t,n){const a=e;for(const r in n){const l=`${t}__${r}`;!a.__datetimeFormatters.has(l)||a.__datetimeFormatters.delete(l)}}function Ee(e,...t){const{numberFormats:n,unresolving:a,fallbackLocale:r,onWarn:l}=e,{__numberFormatters:s}=e,[o,u,m,_]=oe(...t),h=S(m.missingWarn)?m.missingWarn:e.missingWarn;S(m.fallbackWarn)?m.fallbackWarn:e.fallbackWarn;const b=!!m.part,v=g(m.locale)?m.locale:e.locale,k=J(e,r,v);if(!g(o)||o==="")return new Intl.NumberFormat(v).format(u);let p={},d,F=null;const T="number format";for(let E=0;E<k.length&&(d=k[E],p=n[d]||{},F=p[o],!y(F));E++)ge(e,o,d,h,T);if(!y(F)||!g(d))return a?ae:o;let w=`${d}__${o}`;ne(_)||(w=`${w}__${JSON.stringify(_)}`);let f=s.get(w);return f||(f=new Intl.NumberFormat(d,C({},F,_)),s.set(w,f)),b?f.formatToParts(u):f.format(u)}function oe(...e){const[t,n,a,r]=e;let l={},s={};if(!O(t))throw x(14);const o=t;return g(n)?l.key=n:y(n)&&(l=n),g(a)?l.locale=a:y(a)&&(s=a),y(r)&&(s=r),[l.key||"",o,l,s]}function Pe(e,t,n){const a=e;for(const r in n){const l=`${t}__${r}`;!a.__numberFormatters.has(l)||a.__numberFormatters.delete(l)}}/*!
  * vue-i18n v9.1.10
  * (c) 2022 kazuya kawaguchi
  * Released under the MIT License.
  */const un="9.1.10";function P(e,...t){return Ve(e,null,void 0)}const ie=D("__transrateVNode"),ue=D("__datetimeParts"),me=D("__numberParts");D("__enableEmitter");D("__disableEmitter");const mn=D("__setPluralRules");D("__intlifyMeta");const Be=D("__injectWithOption");let Ne=0;function De(e){return(t,n,a,r)=>e(n,a,$e()||void 0,r)}function ze(e,t){const{messages:n,__i18n:a}=t,r=y(n)?n:M(a)?{}:{[e]:{}};if(M(a)&&a.forEach(({locale:l,resource:s})=>{l?(r[l]=r[l]||{},te(s,r[l])):te(s,r)}),t.flatJson)for(const l in r)Ue(r,l)&&le(r[l]);return r}const Z=e=>!W(e)||M(e);function te(e,t){if(Z(e)||Z(t))throw P(20);for(const n in e)Ue(e,n)&&(Z(e[n])||Z(t[n])?t[n]=e[n]:te(e[n],t[n]))}function Je(e={}){const{__root:t}=e,n=t===void 0;let a=S(e.inheritLocale)?e.inheritLocale:!0;const r=G(t&&a?t.locale.value:g(e.locale)?e.locale:"en-US"),l=G(t&&a?t.fallbackLocale.value:g(e.fallbackLocale)||M(e.fallbackLocale)||y(e.fallbackLocale)||e.fallbackLocale===!1?e.fallbackLocale:r.value),s=G(ze(r.value,e)),o=G(y(e.datetimeFormats)?e.datetimeFormats:{[r.value]:{}}),u=G(y(e.numberFormats)?e.numberFormats:{[r.value]:{}});let m=t?t.missingWarn:S(e.missingWarn)||X(e.missingWarn)?e.missingWarn:!0,_=t?t.fallbackWarn:S(e.fallbackWarn)||X(e.fallbackWarn)?e.fallbackWarn:!0,h=t?t.fallbackRoot:S(e.fallbackRoot)?e.fallbackRoot:!0,b=!!e.fallbackFormat,v=L(e.missing)?e.missing:null,k=L(e.missing)?De(e.missing):null,p=L(e.postTranslation)?e.postTranslation:null,d=S(e.warnHtmlMessage)?e.warnHtmlMessage:!0,F=!!e.escapeParameter;const T=t?t.modifiers:y(e.modifiers)?e.modifiers:{};let w=e.pluralRules||t&&t.pluralRules,f;function E(){return tn({version:un,locale:r.value,fallbackLocale:l.value,messages:s.value,datetimeFormats:o.value,numberFormats:u.value,modifiers:T,pluralRules:w,missing:k===null?void 0:k,missingWarn:m,fallbackWarn:_,fallbackFormat:b,unresolving:!0,postTranslation:p===null?void 0:p,warnHtmlMessage:d,escapeParameter:F,__datetimeFormatters:y(f)?f.__datetimeFormatters:void 0,__numberFormatters:y(f)?f.__numberFormatters:void 0,__v_emitter:y(f)?f.__v_emitter:void 0,__meta:{framework:"vue"}})}f=E(),z(f,r.value,l.value);function U(){return[r.value,l.value,s.value,o.value,u.value]}const H=B({get:()=>r.value,set:c=>{r.value=c,f.locale=r.value}}),K=B({get:()=>l.value,set:c=>{l.value=c,f.fallbackLocale=l.value,z(f,r.value,c)}}),Y=B(()=>s.value),be=B(()=>o.value),Ye=B(()=>u.value);function qe(){return L(p)?p:null}function Ze(c){p=c,f.postTranslation=c}function Qe(){return v}function Xe(c){c!==null&&(k=De(c)),v=c,f.missing=k}function A(c,i,N,I,re,q){U();let V;if(V=c(f),O(V)&&V===ae){const[vt,Cn]=i();return t&&h?I(t):re(vt)}else{if(q(V))return V;throw P(14)}}function ye(...c){return A(i=>We(i,...c),()=>se(...c),"translate",i=>i.t(...c),i=>i,i=>g(i))}function et(...c){const[i,N,I]=c;if(I&&!W(I))throw P(15);return ye(i,N,C({resolvedMessage:!0},I||{}))}function tt(...c){return A(i=>Ce(i,...c),()=>ce(...c),"datetime format",i=>i.d(...c),()=>Oe,i=>g(i))}function nt(...c){return A(i=>Ee(i,...c),()=>oe(...c),"number format",i=>i.n(...c),()=>Oe,i=>g(i))}function at(c){return c.map(i=>g(i)?ve(ke,null,i,0):i)}const rt={normalize:at,interpolate:c=>c,type:"vnode"};function lt(...c){return A(i=>{let N;const I=i;try{I.processor=rt,N=We(I,...c)}finally{I.processor=null}return N},()=>se(...c),"translate",i=>i[ie](...c),i=>[ve(ke,null,i,0)],i=>M(i))}function st(...c){return A(i=>Ee(i,...c),()=>oe(...c),"number format",i=>i[me](...c),()=>[],i=>g(i)||M(i))}function ct(...c){return A(i=>Ce(i,...c),()=>ce(...c),"datetime format",i=>i[ue](...c),()=>[],i=>g(i)||M(i))}function ot(c){w=c,f.pluralRules=w}function it(c,i){const N=g(i)?i:r.value,I=he(N);return ee(I,c)!==null}function ut(c){let i=null;const N=J(f,l.value,r.value);for(let I=0;I<N.length;I++){const re=s.value[N[I]]||{},q=ee(re,c);if(q!=null){i=q;break}}return i}function mt(c){const i=ut(c);return i!=null?i:t?t.tm(c)||{}:{}}function he(c){return s.value[c]||{}}function ft(c,i){s.value[c]=i,f.messages=s.value}function gt(c,i){s.value[c]=s.value[c]||{},te(i,s.value[c]),f.messages=s.value}function dt(c){return o.value[c]||{}}function _t(c,i){o.value[c]=i,f.datetimeFormats=o.value,Te(f,c,i)}function bt(c,i){o.value[c]=C(o.value[c]||{},i),f.datetimeFormats=o.value,Te(f,c,i)}function yt(c){return u.value[c]||{}}function ht(c,i){u.value[c]=i,f.numberFormats=u.value,Pe(f,c,i)}function Ft(c,i){u.value[c]=C(u.value[c]||{},i),f.numberFormats=u.value,Pe(f,c,i)}return Ne++,t&&(Fe(t.locale,c=>{a&&(r.value=c,f.locale=c,z(f,r.value,l.value))}),Fe(t.fallbackLocale,c=>{a&&(l.value=c,f.fallbackLocale=c,z(f,r.value,l.value))})),{id:Ne,locale:H,fallbackLocale:K,get inheritLocale(){return a},set inheritLocale(c){a=c,c&&t&&(r.value=t.locale.value,l.value=t.fallbackLocale.value,z(f,r.value,l.value))},get availableLocales(){return Object.keys(s.value).sort()},messages:Y,datetimeFormats:be,numberFormats:Ye,get modifiers(){return T},get pluralRules(){return w||{}},get isGlobal(){return n},get missingWarn(){return m},set missingWarn(c){m=c,f.missingWarn=m},get fallbackWarn(){return _},set fallbackWarn(c){_=c,f.fallbackWarn=_},get fallbackRoot(){return h},set fallbackRoot(c){h=c},get fallbackFormat(){return b},set fallbackFormat(c){b=c,f.fallbackFormat=b},get warnHtmlMessage(){return d},set warnHtmlMessage(c){d=c,f.warnHtmlMessage=c},get escapeParameter(){return F},set escapeParameter(c){F=c,f.escapeParameter=c},t:ye,rt:et,d:tt,n:nt,te:it,tm:mt,getLocaleMessage:he,setLocaleMessage:ft,mergeLocaleMessage:gt,getDateTimeFormat:dt,setDateTimeFormat:_t,mergeDateTimeFormat:bt,getNumberFormat:yt,setNumberFormat:ht,mergeNumberFormat:Ft,getPostTranslationHandler:qe,setPostTranslationHandler:Ze,getMissingHandler:Qe,setMissingHandler:Xe,[ie]:lt,[me]:st,[ue]:ct,[mn]:ot,[Be]:e.__injectWithOption}}const de={tag:{type:[String,Object]},locale:{type:String},scope:{type:String,validator:e=>e==="parent"||e==="global",default:"parent"},i18n:{type:Object}},Re={name:"i18n-t",props:C({keypath:{type:String,required:!0},plural:{type:[Number,String],validator:e=>O(e)||!isNaN(e)}},de),setup(e,t){const{slots:n,attrs:a}=t,r=e.i18n||_e({useScope:e.scope,__useComponent:!0}),l=Object.keys(n).filter(s=>s!=="_");return()=>{const s={};e.locale&&(s.locale=e.locale),e.plural!==void 0&&(s.plural=g(e.plural)?+e.plural:e.plural);const o=fn(t,l),u=r[ie](e.keypath,o,s),m=C({},a);return g(e.tag)?$(e.tag,m,u):W(e.tag)?$(e.tag,m,u):$(xe,m,u)}}};function fn({slots:e},t){return t.length===1&&t[0]==="default"?e.default?e.default():[]:t.reduce((n,a)=>{const r=e[a];return r&&(n[a]=r()),n},{})}function Ke(e,t,n,a){const{slots:r,attrs:l}=t;return()=>{const s={part:!0};let o={};e.locale&&(s.locale=e.locale),g(e.format)?s.key=e.format:W(e.format)&&(g(e.format.key)&&(s.key=e.format.key),o=Object.keys(e.format).reduce((h,b)=>n.includes(b)?C({},h,{[b]:e.format[b]}):h,{}));const u=a(e.value,s,o);let m=[s.key];M(u)?m=u.map((h,b)=>{const v=r[h.type];return v?v({[h.type]:h.value,index:b,parts:u}):[h.value]}):g(u)&&(m=[u]);const _=C({},l);return g(e.tag)?$(e.tag,_,m):W(e.tag)?$(e.tag,_,m):$(xe,_,m)}}const gn=["localeMatcher","style","unit","unitDisplay","currency","currencyDisplay","useGrouping","numberingSystem","minimumIntegerDigits","minimumFractionDigits","maximumFractionDigits","minimumSignificantDigits","maximumSignificantDigits","notation","formatMatcher"],je={name:"i18n-n",props:C({value:{type:Number,required:!0},format:{type:[String,Object]}},de),setup(e,t){const n=e.i18n||_e({useScope:"parent",__useComponent:!0});return Ke(e,t,gn,(...a)=>n[me](...a))}},dn=["dateStyle","timeStyle","fractionalSecondDigits","calendar","dayPeriod","numberingSystem","localeMatcher","timeZone","hour12","hourCycle","formatMatcher","weekday","era","year","month","day","hour","minute","second","timeZoneName"],Ae={name:"i18n-d",props:C({value:{type:[Number,Date],required:!0},format:{type:[String,Object]}},de),setup(e,t){const n=e.i18n||_e({useScope:"parent",__useComponent:!0});return Ke(e,t,dn,(...a)=>n[ue](...a))}};function _n(e,t){const n=e;if(e.mode==="composition")return n.__getInstance(t)||e.global;{const a=n.__getInstance(t);return a!=null?a.__composer:e.global.__composer}}function bn(e){const t=(n,{instance:a,value:r,modifiers:l})=>{if(!a||!a.$)throw P(22);const s=_n(e,a.$),o=yn(r);n.textContent=s.t(...hn(o))};return{beforeMount:t,beforeUpdate:t}}function yn(e){if(g(e))return{path:e};if(y(e)){if(!("path"in e))throw P(19,"path");return e}else throw P(20)}function hn(e){const{path:t,locale:n,args:a,choice:r,plural:l}=e,s={},o=a||{};return g(n)&&(s.locale=n),O(r)&&(s.plural=r),O(l)&&(s.plural=l),[t,o,s]}function Fn(e,t,...n){const a=y(n[0])?n[0]:{},r=!!a.useI18nComponentName;(S(a.globalInstall)?a.globalInstall:!0)&&(e.component(r?"i18n":Re.name,Re),e.component(je.name,je),e.component(Ae.name,Ae)),e.directive("t",bn(t))}function vn(e={}){const t=!!e.globalInjection,n=new Map,a=Je(e),r=D(""),l={get mode(){return"composition"},async install(s,...o){s.__VUE_I18N_SYMBOL__=r,s.provide(s.__VUE_I18N_SYMBOL__,l),t&&On(s,l.global),Fn(s,l,...o)},get global(){return a},__instances:n,__getInstance(s){return n.get(s)||null},__setInstance(s,o){n.set(s,o)},__deleteInstance(s){n.delete(s)}};return l}function _e(e={}){const t=$e();if(t==null)throw P(16);if(!t.appContext.app.__VUE_I18N_SYMBOL__)throw P(17);const n=pt(t.appContext.app.__VUE_I18N_SYMBOL__);if(!n)throw P(22);const a=n.mode==="composition"?n.global:n.global.__composer,r=ne(e)?"__i18n"in t.type?"local":"global":e.useScope?e.useScope:"local";if(r==="global"){let o=W(e.messages)?e.messages:{};"__i18nGlobal"in t.type&&(o=ze(a.locale.value,{messages:o,__i18n:t.type.__i18nGlobal}));const u=Object.keys(o);if(u.length&&u.forEach(m=>{a.mergeLocaleMessage(m,o[m])}),W(e.datetimeFormats)){const m=Object.keys(e.datetimeFormats);m.length&&m.forEach(_=>{a.mergeDateTimeFormat(_,e.datetimeFormats[_])})}if(W(e.numberFormats)){const m=Object.keys(e.numberFormats);m.length&&m.forEach(_=>{a.mergeNumberFormat(_,e.numberFormats[_])})}return a}if(r==="parent"){let o=kn(n,t,e.__useComponent);return o==null&&(o=a),o}if(n.mode==="legacy")throw P(18);const l=n;let s=l.__getInstance(t);if(s==null){const o=t.type,u=C({},e);o.__i18n&&(u.__i18n=o.__i18n),a&&(u.__root=a),s=Je(u),pn(l,t),l.__setInstance(t,s)}return s}function kn(e,t,n=!1){let a=null;const r=t.root;let l=t.parent;for(;l!=null;){const s=e;if(e.mode==="composition")a=s.__getInstance(l);else{const o=s.__getInstance(l);o!=null&&(a=o.__composer),n&&a&&!a[Be]&&(a=null)}if(a!=null||r===l)break;l=l.parent}return a}function pn(e,t,n){wt(()=>{},t),St(()=>{e.__deleteInstance(t)},t)}const wn=["locale","fallbackLocale","availableLocales"],Sn=["t","rt","d","n","tm"];function On(e,t){const n=Object.create(null);wn.forEach(a=>{const r=Object.getOwnPropertyDescriptor(t,a);if(!r)throw P(22);const l=kt(r.value)?{get(){return r.value.value},set(s){r.value.value=s}}:{get(){return r.get&&r.get()}};Object.defineProperty(n,a,l)}),e.config.globalProperties.$i18n=n,Sn.forEach(a=>{const r=Object.getOwnPropertyDescriptor(t,a);if(!r||!r.value)throw P(22);Object.defineProperty(e.config.globalProperties,`$${a}`,r)})}var Mn={failed:"Action failed",success:"Action was successful"},In={"en-US":Mn},En=Ot(({app:e})=>{const t=vn({locale:"en-US",messages:In});e.use(t)});export{En as default};