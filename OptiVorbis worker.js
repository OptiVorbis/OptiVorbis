(()=>{"use strict";var e,n,t,r,o={800:(e,n,t)=>{t.a(e,(async(e,r)=>{try{t.d(n,{Zu:()=>_.Zu});var o=t(2449),_=t(3118),c=e([o]);o=(c.then?(await c)():c)[0],(0,_.oT)(o),o.__wbindgen_start(),r()}catch(e){r(e)}}))},3118:(e,n,t)=>{let r;function o(e){r=e}t.d(n,{$3:()=>W,AC:()=>q,CN:()=>ne,E$:()=>R,H6:()=>ee,JN:()=>D,KM:()=>k,Lx:()=>j,N5:()=>U,Nl:()=>Q,Od:()=>z,Or:()=>re,PY:()=>te,Qz:()=>I,Sv:()=>E,Wl:()=>$,XP:()=>K,Zf:()=>X,Zu:()=>m,a2:()=>T,c7:()=>J,eC:()=>M,eY:()=>B,ey:()=>Y,fr:()=>V,h4:()=>v,iX:()=>P,k4:()=>A,m_:()=>H,o$:()=>L,oH:()=>oe,oM:()=>F,oO:()=>N,oT:()=>o,rU:()=>G,sQ:()=>S,ug:()=>x,vD:()=>C,y1:()=>O,zw:()=>Z}),e=t.hmd(e);let _=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});_.decode();let c=null;function i(){return null!==c&&0!==c.byteLength||(c=new Uint8Array(r.memory.buffer)),c}function u(e,n){return e>>>=0,_.decode(i().subarray(e,e+n))}const a=new Array(128).fill(void 0);a.push(void 0,null,!0,!1);let f=a.length;function s(e){f===a.length&&a.push(a.length+1);const n=f;return f=a[n],a[n]=e,n}function b(e){return a[e]}function d(e){const n=b(e);return function(e){e<132||(a[e]=f,f=e)}(e),n}let l=0,g=null;function w(){return null!==g&&0!==g.byteLength||(g=new Int32Array(r.memory.buffer)),g}let p=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const y="function"==typeof p.encodeInto?function(e,n){return p.encodeInto(e,n)}:function(e,n){const t=p.encode(e);return n.set(t),{read:e.length,written:t.length}};function h(e,n){try{return e.apply(this,n)}catch(e){r.__wbindgen_export_3(s(e))}}class m{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,e}free(){const e=this.__destroy_into_raw();r.__wbg_oggtoogg_free(e)}constructor(){const e=r.oggtoogg_new_with_defaults();return this.__wbg_ptr=e>>>0,this}remux(e){try{const a=r.__wbindgen_add_to_stack_pointer(-16),f=function(e,n){const t=n(1*e.length,1)>>>0;return i().set(e,t/1),l=e.length,t}(e,r.__wbindgen_export_0),s=l;r.oggtoogg_remux(a,this.__wbg_ptr,f,s);var n=w()[a/4+0],t=w()[a/4+1],o=w()[a/4+2];if(w()[a/4+3])throw d(o);var _=(c=n,u=t,c>>>=0,i().subarray(c/1,c/1+u)).slice();return r.__wbindgen_export_1(n,1*t,1),_}finally{r.__wbindgen_add_to_stack_pointer(16)}var c,u}}function v(e,n){return s(u(e,n))}function x(e){d(e)}function S(e,n,t,r){console.debug(b(e),b(n),b(t),b(r))}function A(e,n,t,r){console.error(b(e),b(n),b(t),b(r))}function E(e,n,t,r){console.info(b(e),b(n),b(t),b(r))}function j(e,n,t,r){console.log(b(e),b(n),b(t),b(r))}function O(e,n,t,r){console.warn(b(e),b(n),b(t),b(r))}function T(){return s(new Error)}function k(e,n){const t=function(e,n,t){if(void 0===t){const t=p.encode(e),r=n(t.length,1)>>>0;return i().subarray(r,r+t.length).set(t),l=t.length,r}let r=e.length,o=n(r,1)>>>0;const _=i();let c=0;for(;c<r;c++){const n=e.charCodeAt(c);if(n>127)break;_[o+c]=n}if(c!==r){0!==c&&(e=e.slice(c)),o=t(o,r,r=c+3*e.length,1)>>>0;const n=i().subarray(o+c,o+r);c+=y(e,n).written}return l=c,o}(b(n).stack,r.__wbindgen_export_0,r.__wbindgen_export_2),o=l;w()[e/4+1]=o,w()[e/4+0]=t}function P(e,n){let t,o;try{t=e,o=n,console.error(u(e,n))}finally{r.__wbindgen_export_1(t,o,1)}}function C(){return h((function(e,n){b(e).getRandomValues(b(n))}),arguments)}function M(){return h((function(e,n){b(e).randomFillSync(d(n))}),arguments)}function N(e){return s(b(e).crypto)}function $(e){const n=b(e);return"object"==typeof n&&null!==n}function U(e){return s(b(e).process)}function q(e){return s(b(e).versions)}function Z(e){return s(b(e).node)}function B(e){return"string"==typeof b(e)}function D(e){return s(b(e).msCrypto)}function F(){return h((function(){return s(e.require)}),arguments)}function L(e){return"function"==typeof b(e)}function W(e,n){return s(new Function(u(e,n)))}function z(){return h((function(e,n){return s(b(e).call(b(n)))}),arguments)}function H(e){return s(b(e))}function Q(){return h((function(e,n,t){return s(b(e).call(b(n),b(t)))}),arguments)}function X(e){return s(b(e).buffer)}function Y(){return h((function(){return s(self.self)}),arguments)}function I(){return h((function(){return s(window.window)}),arguments)}function R(){return h((function(){return s(globalThis.globalThis)}),arguments)}function J(){return h((function(){return s(t.g.global)}),arguments)}function K(e){return void 0===b(e)}function V(e,n,t){return s(new Uint8Array(b(e),n>>>0,t>>>0))}function G(e){return s(new Uint8Array(b(e)))}function ee(e,n,t){b(e).set(b(n),t>>>0)}function ne(e){return s(new Uint8Array(e>>>0))}function te(e,n,t){return s(b(e).subarray(n>>>0,t>>>0))}function re(e,n){throw new Error(u(e,n))}function oe(){return s(r.memory)}},3881:(e,n,t)=>{t.a(e,(async(e,n)=>{try{var r=t(800),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const n=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),t=(new r.Zu).remux(n);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),n()}catch(e){n(e)}}))},2449:(e,n,t)=>{var r=t(3118);e.exports=t.v(n,e.id,"b60ebda2fc7fbba27f1c",{"./index_bg.js":{__wbindgen_string_new:r.h4,__wbindgen_object_drop_ref:r.ug,__wbg_debug_9b8701f894da9929:r.sQ,__wbg_error_d9bce418caafb712:r.k4,__wbg_info_bb52f40b06f679de:r.Sv,__wbg_log_ea7093e35e3efd07:r.Lx,__wbg_warn_dfc0e0cf544a13bd:r.y1,__wbg_new_abda76e883ba8a5f:r.a2,__wbg_stack_658279fe44541cf6:r.KM,__wbg_error_f851667af71bcfc6:r.iX,__wbg_getRandomValues_7e42b4fb8779dc6d:r.vD,__wbg_randomFillSync_b70ccbdf4926a99d:r.eC,__wbg_crypto_d05b68a3572bb8ca:r.oO,__wbindgen_is_object:r.Wl,__wbg_process_b02b3570280d0366:r.N5,__wbg_versions_c1cb42213cedf0f5:r.AC,__wbg_node_43b1089f407e4ec2:r.zw,__wbindgen_is_string:r.eY,__wbg_msCrypto_10fc94afee92bd76:r.JN,__wbg_require_9a7e0f667ead4995:r.oM,__wbindgen_is_function:r.o$,__wbg_newnoargs_581967eacc0e2604:r.$3,__wbg_call_cb65541d95d71282:r.Od,__wbindgen_object_clone_ref:r.m_,__wbg_call_01734de55d61e11d:r.Nl,__wbg_buffer_085ec1f694018c4f:r.Zf,__wbg_self_1ff1d729e9aae938:r.ey,__wbg_window_5f4faef6c12b79ec:r.Qz,__wbg_globalThis_1d39714405582d3c:r.E$,__wbg_global_651f05c6a0944d1c:r.c7,__wbindgen_is_undefined:r.XP,__wbg_newwithbyteoffsetandlength_6da8e527659b86aa:r.fr,__wbg_new_8125e318e6245eed:r.rU,__wbg_set_5cf90238115182c3:r.H6,__wbg_newwithlength_e5d69174d6984cd7:r.CN,__wbg_subarray_13db269f57aa838d:r.PY,__wbindgen_throw:r.Or,__wbindgen_memory:r.oH}})}},_={};function c(e){var n=_[e];if(void 0!==n)return n.exports;var t=_[e]={id:e,loaded:!1,exports:{}};return o[e](t,t.exports,c),t.loaded=!0,t.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},c.a=(o,_,c)=>{var i;c&&((i=[]).d=-1);var u,a,f,s=new Set,b=o.exports,d=new Promise(((e,n)=>{f=n,a=e}));d[n]=b,d[e]=e=>(i&&e(i),s.forEach(e),d.catch((e=>{}))),o.exports=d,_((o=>{var _;u=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var _=[];_.d=0,o.then((e=>{c[n]=e,r(_)}),(e=>{c[t]=e,r(_)}));var c={};return c[e]=e=>e(_),c}}var i={};return i[e]=e=>{},i[n]=o,i})))(o);var c=()=>u.map((e=>{if(e[t])throw e[t];return e[n]})),a=new Promise((n=>{(_=()=>n(c)).r=0;var t=e=>e!==i&&!s.has(e)&&(s.add(e),e&&!e.d&&(_.r++,e.push(_)));u.map((n=>n[e](t)))}));return _.r?a:c()}),(e=>(e?f(d[t]=e):a(b),r(i)))),i&&i.d<0&&(i.d=0)},c.d=(e,n)=>{for(var t in n)c.o(n,t)&&!c.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},c.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),c.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),c.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),c.v=(e,n,t,r)=>{var o=fetch(c.p+""+t+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,r).then((n=>Object.assign(e,n.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)))},(()=>{var e;c.g.importScripts&&(e=c.g.location+"");var n=c.g.document;if(!e&&n&&(n.currentScript&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");if(t.length)for(var r=t.length-1;r>-1&&!e;)e=t[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),c.p=e})(),c(3881)})();