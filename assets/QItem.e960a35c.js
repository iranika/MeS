import{c as B,h as x,a as I}from"./render.83a94de3.js";import{c as i,h as k,g as L,w,o as R,n as C,r as M,V as T,s as P}from"./index.e70cde7f.js";import{v as S,u as $,b as A}from"./QBtn.802e3ea2.js";var D=B({name:"QItemLabel",props:{overline:Boolean,caption:Boolean,header:Boolean,lines:[Number,String]},setup(e,{slots:l}){const u=i(()=>parseInt(e.lines,10)),o=i(()=>"q-item__label"+(e.overline===!0?" q-item__label--overline text-overline":"")+(e.caption===!0?" q-item__label--caption text-caption":"")+(e.header===!0?" q-item__label--header":"")+(u.value===1?" ellipsis":"")),d=i(()=>e.lines!==void 0&&u.value>1?{overflow:"hidden",display:"-webkit-box","-webkit-box-orient":"vertical","-webkit-line-clamp":u.value}:null);return()=>k("div",{style:d.value,class:o.value},x(l.default))}});const Q={dark:{type:Boolean,default:null}};function E(e,l){return i(()=>e.dark===null?l.dark.isActive:e.dark)}var F=B({name:"QList",props:{...Q,bordered:Boolean,dense:Boolean,separator:Boolean,padding:Boolean},setup(e,{slots:l}){const u=L(),o=E(e,u.proxy.$q),d=i(()=>"q-list"+(e.bordered===!0?" q-list--bordered":"")+(e.dense===!0?" q-list--dense":"")+(e.separator===!0?" q-list--separator":"")+(o.value===!0?" q-list--dark":"")+(e.padding===!0?" q-list--padding":""));return()=>k("div",{class:d.value},x(l.default))}});const N={modelValue:{type:Boolean,default:null},"onUpdate:modelValue":[Function,Array]},H=["before-show","show","before-hide","hide"];function W({showing:e,canShow:l,hideOnRouteChange:u,handleShow:o,handleHide:d,processOnMount:p}){const b=L(),{props:n,emit:s,proxy:h}=b;let r;function m(t){e.value===!0?c(t):v(t)}function v(t){if(n.disable===!0||t!==void 0&&t.qAnchorHandled===!0||l!==void 0&&l(t)!==!0)return;const f=n["onUpdate:modelValue"]!==void 0;f===!0&&(s("update:modelValue",!0),r=t,C(()=>{r===t&&(r=void 0)})),(n.modelValue===null||f===!1)&&q(t)}function q(t){e.value!==!0&&(e.value=!0,s("before-show",t),o!==void 0?o(t):s("show",t))}function c(t){if(n.disable===!0)return;const f=n["onUpdate:modelValue"]!==void 0;f===!0&&(s("update:modelValue",!1),r=t,C(()=>{r===t&&(r=void 0)})),(n.modelValue===null||f===!1)&&y(t)}function y(t){e.value!==!1&&(e.value=!1,s("before-hide",t),d!==void 0?d(t):s("hide",t))}function g(t){n.disable===!0&&t===!0?n["onUpdate:modelValue"]!==void 0&&s("update:modelValue",!1):t===!0!==e.value&&(t===!0?q:y)(r)}w(()=>n.modelValue,g),u!==void 0&&S(b)===!0&&w(()=>h.$route.fullPath,()=>{u.value===!0&&e.value===!0&&c()}),p===!0&&R(()=>{g(n.modelValue)});const _={show:v,hide:c,toggle:m};return Object.assign(h,_),_}function O(e,l,u){return u<=l?l:Math.min(u,Math.max(l,e))}var z=B({name:"QItemSection",props:{avatar:Boolean,thumbnail:Boolean,side:Boolean,top:Boolean,noWrap:Boolean},setup(e,{slots:l}){const u=i(()=>`q-item__section column q-item__section--${e.avatar===!0||e.side===!0||e.thumbnail===!0?"side":"main"}`+(e.top===!0?" q-item__section--top justify-start":" justify-center")+(e.avatar===!0?" q-item__section--avatar":"")+(e.thumbnail===!0?" q-item__section--thumbnail":"")+(e.noWrap===!0?" q-item__section--nowrap":""));return()=>k("div",{class:u.value},x(l.default))}}),G=B({name:"QItem",props:{...Q,...$,tag:{type:String,default:"div"},active:{type:Boolean,default:null},clickable:Boolean,dense:Boolean,insetLevel:Number,tabindex:[String,Number],focused:Boolean,manualFocus:Boolean},emits:["click","keyup"],setup(e,{slots:l,emit:u}){const{proxy:{$q:o}}=L(),d=E(e,o),{hasRouterLink:p,hasLink:b,linkProps:n,linkClass:s,linkTag:h,navigateToRouterLink:r}=A(),m=M(null),v=M(null),q=i(()=>e.clickable===!0||b.value===!0||e.tag==="label"),c=i(()=>e.disable!==!0&&q.value===!0),y=i(()=>"q-item q-item-type row no-wrap"+(e.dense===!0?" q-item--dense":"")+(d.value===!0?" q-item--dark":"")+(b.value===!0&&e.active===null?s.value:e.active===!0?`${e.activeClass!==void 0?` ${e.activeClass}`:""} q-item--active`:"")+(e.disable===!0?" disabled":"")+(c.value===!0?" q-item--clickable q-link cursor-pointer "+(e.manualFocus===!0?"q-manual-focusable":"q-focusable q-hoverable")+(e.focused===!0?" q-manual-focusable--focused":""):"")),g=i(()=>{if(e.insetLevel===void 0)return null;const a=o.lang.rtl===!0?"Right":"Left";return{["padding"+a]:16+e.insetLevel*56+"px"}});function _(a){c.value===!0&&(v.value!==null&&(a.qKeyEvent!==!0&&document.activeElement===m.value?v.value.focus():document.activeElement===v.value&&m.value.focus()),p.value===!0&&r(a),u("click",a))}function t(a){if(c.value===!0&&T(a,13)===!0){P(a),a.qKeyEvent=!0;const V=new MouseEvent("click",a);V.qKeyEvent=!0,m.value.dispatchEvent(V)}u("keyup",a)}function f(){const a=I(l.default,[]);return c.value===!0&&a.unshift(k("div",{class:"q-focus-helper",tabindex:-1,ref:v})),a}return()=>{const a={ref:m,class:y.value,style:g.value,onClick:_,onKeyup:t};return c.value===!0?(a.tabindex=e.tabindex||"0",Object.assign(a,n.value)):q.value===!0&&(a["aria-disabled"]="true"),k(h.value,a,f())}}});export{z as Q,Q as a,H as b,E as c,W as d,O as e,D as f,G as g,F as h,N as u};