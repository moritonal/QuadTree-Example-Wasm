"use strict";(self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[]).push([[842],{771:(e,t,r)=>{var n=r(2613);t.X4=h,t.e$=g,t.eM=function(e,t){const r=f(e),n=f(t);return(Math.max(r,n)+.05)/(Math.min(r,n)+.05)},t.a=d;var l=n(r(2108)),a=n(r(4966));function s(e,t=0,r=1){return(0,a.default)(e,t,r)}function o(e){e=e.slice(1);const t=new RegExp(`.{1,${e.length>=6?2:1}}`,"g");let r=e.match(t);return r&&1===r[0].length&&(r=r.map((e=>e+e))),r?`rgb${4===r.length?"a":""}(${r.map(((e,t)=>t<3?parseInt(e,16):Math.round(parseInt(e,16)/255*1e3)/1e3)).join(", ")})`:""}function i(e){if(e.type)return e;if("#"===e.charAt(0))return i(o(e));const t=e.indexOf("("),r=e.substring(0,t);if(-1===["rgb","rgba","hsl","hsla","color"].indexOf(r))throw new Error((0,l.default)(9,e));let n,a=e.substring(t+1,e.length-1);if("color"===r){if(a=a.split(" "),n=a.shift(),4===a.length&&"/"===a[3].charAt(0)&&(a[3]=a[3].slice(1)),-1===["srgb","display-p3","a98-rgb","prophoto-rgb","rec-2020"].indexOf(n))throw new Error((0,l.default)(10,n))}else a=a.split(",");return a=a.map((e=>parseFloat(e))),{type:r,values:a,colorSpace:n}}const u=e=>{const t=i(e);return t.values.slice(0,3).map(((e,r)=>-1!==t.type.indexOf("hsl")&&0!==r?`${e}%`:e)).join(" ")};function p(e){const{type:t,colorSpace:r}=e;let{values:n}=e;return-1!==t.indexOf("rgb")?n=n.map(((e,t)=>t<3?parseInt(e,10):e)):-1!==t.indexOf("hsl")&&(n[1]=`${n[1]}%`,n[2]=`${n[2]}%`),n=-1!==t.indexOf("color")?`${r} ${n.join(" ")}`:`${n.join(", ")}`,`${t}(${n})`}function c(e){e=i(e);const{values:t}=e,r=t[0],n=t[1]/100,l=t[2]/100,a=n*Math.min(l,1-l),s=(e,t=(e+r/30)%12)=>l-a*Math.max(Math.min(t-3,9-t,1),-1);let o="rgb";const u=[Math.round(255*s(0)),Math.round(255*s(8)),Math.round(255*s(4))];return"hsla"===e.type&&(o+="a",u.push(t[3])),p({type:o,values:u})}function f(e){let t="hsl"===(e=i(e)).type||"hsla"===e.type?i(c(e)).values:e.values;return t=t.map((t=>("color"!==e.type&&(t/=255),t<=.03928?t/12.92:((t+.055)/1.055)**2.4))),Number((.2126*t[0]+.7152*t[1]+.0722*t[2]).toFixed(3))}function h(e,t){return e=i(e),t=s(t),"rgb"!==e.type&&"hsl"!==e.type||(e.type+="a"),"color"===e.type?e.values[3]=`/${t}`:e.values[3]=t,p(e)}function g(e,t){if(e=i(e),t=s(t),-1!==e.type.indexOf("hsl"))e.values[2]*=1-t;else if(-1!==e.type.indexOf("rgb")||-1!==e.type.indexOf("color"))for(let r=0;r<3;r+=1)e.values[r]*=1-t;return p(e)}function d(e,t){if(e=i(e),t=s(t),-1!==e.type.indexOf("hsl"))e.values[2]+=(100-e.values[2])*t;else if(-1!==e.type.indexOf("rgb"))for(let r=0;r<3;r+=1)e.values[r]+=(255-e.values[r])*t;else if(-1!==e.type.indexOf("color"))for(let r=0;r<3;r+=1)e.values[r]+=(1-e.values[r])*t;return p(e)}},4279:(e,t,r)=>{r.d(t,{X4:()=>i});var n=r(5697),l=r(6937);function a(e,t=0,r=1){return(0,l.A)(e,t,r)}function s(e){if(e.type)return e;if("#"===e.charAt(0))return s(function(e){e=e.slice(1);const t=new RegExp(`.{1,${e.length>=6?2:1}}`,"g");let r=e.match(t);return r&&1===r[0].length&&(r=r.map((e=>e+e))),r?`rgb${4===r.length?"a":""}(${r.map(((e,t)=>t<3?parseInt(e,16):Math.round(parseInt(e,16)/255*1e3)/1e3)).join(", ")})`:""}(e));const t=e.indexOf("("),r=e.substring(0,t);if(-1===["rgb","rgba","hsl","hsla","color"].indexOf(r))throw new Error((0,n.A)(9,e));let l,a=e.substring(t+1,e.length-1);if("color"===r){if(a=a.split(" "),l=a.shift(),4===a.length&&"/"===a[3].charAt(0)&&(a[3]=a[3].slice(1)),-1===["srgb","display-p3","a98-rgb","prophoto-rgb","rec-2020"].indexOf(l))throw new Error((0,n.A)(10,l))}else a=a.split(",");return a=a.map((e=>parseFloat(e))),{type:r,values:a,colorSpace:l}}function o(e){const{type:t,colorSpace:r}=e;let{values:n}=e;return-1!==t.indexOf("rgb")?n=n.map(((e,t)=>t<3?parseInt(e,10):e)):-1!==t.indexOf("hsl")&&(n[1]=`${n[1]}%`,n[2]=`${n[2]}%`),n=-1!==t.indexOf("color")?`${r} ${n.join(" ")}`:`${n.join(", ")}`,`${t}(${n})`}function i(e,t){return e=s(e),t=a(t),"rgb"!==e.type&&"hsl"!==e.type||(e.type+="a"),"color"===e.type?e.values[3]=`/${t}`:e.values[3]=t,o(e)}}}]);