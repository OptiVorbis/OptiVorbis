(()=>{"use strict";var e,n,t,r,o={331:(e,n,t)=>{t.a(e,(async(e,r)=>{try{t.d(n,{u6:()=>_.u6});var o=t(4464),_=t(5856),c=e([o]);o=(c.then?(await c)():c)[0],(0,_.o9)(o),o.__wbindgen_start(),r()}catch(e){r(e)}}))},5856:(e,n,t)=>{let r;function o(e){r=e}t.d(n,{AD:()=>X,AV:()=>Z,Ax:()=>Y,Az:()=>K,CK:()=>B,E5:()=>H,Gy:()=>F,IP:()=>R,Ij:()=>oe,Ir:()=>S,KK:()=>j,KY:()=>J,M$:()=>O,MV:()=>ne,OE:()=>$,QB:()=>N,Qp:()=>T,SW:()=>M,Sk:()=>q,UB:()=>W,Up:()=>ee,WU:()=>L,_8:()=>k,aw:()=>te,cK:()=>V,cb:()=>U,cf:()=>I,cm:()=>z,d0:()=>re,g$:()=>x,h7:()=>E,im:()=>P,me:()=>v,o9:()=>o,qs:()=>Q,sH:()=>A,u6:()=>m,w1:()=>G,wE:()=>D,zz:()=>C}),e=t.hmd(e);let _=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});_.decode();let c=null;function i(){return null!==c&&0!==c.byteLength||(c=new Uint8Array(r.memory.buffer)),c}function a(e,n){return e>>>=0,_.decode(i().subarray(e,e+n))}const u=new Array(128).fill(void 0);u.push(void 0,null,!0,!1);let s=u.length;function f(e){s===u.length&&u.push(u.length+1);const n=s;return s=u[n],u[n]=e,n}function b(e){return u[e]}function d(e){const n=b(e);return function(e){e<132||(u[e]=s,s=e)}(e),n}let l=0,w=null;function g(){return null!==w&&0!==w.byteLength||(w=new Int32Array(r.memory.buffer)),w}let p=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const y="function"==typeof p.encodeInto?function(e,n){return p.encodeInto(e,n)}:function(e,n){const t=p.encode(e);return n.set(t),{read:e.length,written:t.length}};function h(e,n){try{return e.apply(this,n)}catch(e){r.__wbindgen_export_3(f(e))}}class m{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,e}free(){const e=this.__destroy_into_raw();r.__wbg_oggtoogg_free(e)}constructor(){const e=r.oggtoogg_new_with_defaults();return this.__wbg_ptr=e>>>0,this}remux(e){try{const u=r.__wbindgen_add_to_stack_pointer(-16),s=function(e,n){const t=n(1*e.length,1)>>>0;return i().set(e,t/1),l=e.length,t}(e,r.__wbindgen_export_0),f=l;r.oggtoogg_remux(u,this.__wbg_ptr,s,f);var n=g()[u/4+0],t=g()[u/4+1],o=g()[u/4+2];if(g()[u/4+3])throw d(o);var _=(c=n,a=t,c>>>=0,i().subarray(c/1,c/1+a)).slice();return r.__wbindgen_export_1(n,1*t,1),_}finally{r.__wbindgen_add_to_stack_pointer(16)}var c,a}}function v(e,n){return f(a(e,n))}function x(e){d(e)}function A(e,n,t,r){console.debug(b(e),b(n),b(t),b(r))}function S(e,n,t,r){console.error(b(e),b(n),b(t),b(r))}function E(e,n,t,r){console.info(b(e),b(n),b(t),b(r))}function j(e,n,t,r){console.log(b(e),b(n),b(t),b(r))}function k(e,n,t,r){console.warn(b(e),b(n),b(t),b(r))}function T(){return f(new Error)}function U(e,n){const t=function(e,n,t){if(void 0===t){const t=p.encode(e),r=n(t.length,1)>>>0;return i().subarray(r,r+t.length).set(t),l=t.length,r}let r=e.length,o=n(r,1)>>>0;const _=i();let c=0;for(;c<r;c++){const n=e.charCodeAt(c);if(n>127)break;_[o+c]=n}if(c!==r){0!==c&&(e=e.slice(c)),o=t(o,r,r=c+3*e.length,1)>>>0;const n=i().subarray(o+c,o+r);c+=y(e,n).written}return l=c,o}(b(n).stack,r.__wbindgen_export_0,r.__wbindgen_export_2),o=l;g()[e/4+1]=o,g()[e/4+0]=t}function M(e,n){let t,o;try{t=e,o=n,console.error(a(e,n))}finally{r.__wbindgen_export_1(t,o,1)}}function O(){return h((function(e,n){b(e).getRandomValues(b(n))}),arguments)}function I(){return h((function(e,n){b(e).randomFillSync(d(n))}),arguments)}function K(e){return f(b(e).crypto)}function B(e){const n=b(e);return"object"==typeof n&&null!==n}function W(e){return f(b(e).process)}function q(e){return f(b(e).versions)}function P(e){return f(b(e).node)}function $(e){return"string"==typeof b(e)}function z(e){return f(b(e).msCrypto)}function C(){return h((function(){return f(e.require)}),arguments)}function F(e){return"function"==typeof b(e)}function V(e,n){return f(new Function(a(e,n)))}function D(){return h((function(e,n){return f(b(e).call(b(n)))}),arguments)}function Q(e){return f(b(e))}function L(){return h((function(e,n,t){return f(b(e).call(b(n),b(t)))}),arguments)}function R(e){return f(b(e).buffer)}function G(){return h((function(){return f(self.self)}),arguments)}function H(){return h((function(){return f(window.window)}),arguments)}function Y(){return h((function(){return f(globalThis.globalThis)}),arguments)}function N(){return h((function(){return f(t.g.global)}),arguments)}function J(e){return void 0===b(e)}function X(e,n,t){return f(new Uint8Array(b(e),n>>>0,t>>>0))}function Z(e){return f(new Uint8Array(b(e)))}function ee(e,n,t){b(e).set(b(n),t>>>0)}function ne(e){return f(new Uint8Array(e>>>0))}function te(e,n,t){return f(b(e).subarray(n>>>0,t>>>0))}function re(e,n){throw new Error(a(e,n))}function oe(){return f(r.memory)}},9064:(e,n,t)=>{t.a(e,(async(e,n)=>{try{var r=t(331),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const n=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),t=(new r.u6).remux(n);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),n()}catch(e){n(e)}}))},4464:(e,n,t)=>{var r=t(5856);e.exports=t.v(n,e.id,"a2220b35b337714ca2b4",{"./index_bg.js":{__wbindgen_string_new:r.me,__wbindgen_object_drop_ref:r.g$,__wbg_debug_9b8701f894da9929:r.sH,__wbg_error_d9bce418caafb712:r.Ir,__wbg_info_bb52f40b06f679de:r.h7,__wbg_log_ea7093e35e3efd07:r.KK,__wbg_warn_dfc0e0cf544a13bd:r._8,__wbg_new_abda76e883ba8a5f:r.Qp,__wbg_stack_658279fe44541cf6:r.cb,__wbg_error_f851667af71bcfc6:r.SW,__wbg_getRandomValues_7e42b4fb8779dc6d:r.M$,__wbg_randomFillSync_b70ccbdf4926a99d:r.cf,__wbg_crypto_d05b68a3572bb8ca:r.Az,__wbindgen_is_object:r.CK,__wbg_process_b02b3570280d0366:r.UB,__wbg_versions_c1cb42213cedf0f5:r.Sk,__wbg_node_43b1089f407e4ec2:r.im,__wbindgen_is_string:r.OE,__wbg_msCrypto_10fc94afee92bd76:r.cm,__wbg_require_9a7e0f667ead4995:r.zz,__wbindgen_is_function:r.Gy,__wbg_newnoargs_581967eacc0e2604:r.cK,__wbg_call_cb65541d95d71282:r.wE,__wbindgen_object_clone_ref:r.qs,__wbg_call_01734de55d61e11d:r.WU,__wbg_buffer_085ec1f694018c4f:r.IP,__wbg_self_1ff1d729e9aae938:r.w1,__wbg_window_5f4faef6c12b79ec:r.E5,__wbg_globalThis_1d39714405582d3c:r.Ax,__wbg_global_651f05c6a0944d1c:r.QB,__wbindgen_is_undefined:r.KY,__wbg_newwithbyteoffsetandlength_6da8e527659b86aa:r.AD,__wbg_new_8125e318e6245eed:r.AV,__wbg_set_5cf90238115182c3:r.Up,__wbg_newwithlength_e5d69174d6984cd7:r.MV,__wbg_subarray_13db269f57aa838d:r.aw,__wbindgen_throw:r.d0,__wbindgen_memory:r.Ij}})}},_={};function c(e){var n=_[e];if(void 0!==n)return n.exports;var t=_[e]={id:e,loaded:!1,exports:{}};return o[e](t,t.exports,c),t.loaded=!0,t.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},c.a=(o,_,c)=>{var i;c&&((i=[]).d=-1);var a,u,s,f=new Set,b=o.exports,d=new Promise(((e,n)=>{s=n,u=e}));d[n]=b,d[e]=e=>(i&&e(i),f.forEach(e),d.catch((e=>{}))),o.exports=d,_((o=>{var _;a=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var _=[];_.d=0,o.then((e=>{c[n]=e,r(_)}),(e=>{c[t]=e,r(_)}));var c={};return c[e]=e=>e(_),c}}var i={};return i[e]=e=>{},i[n]=o,i})))(o);var c=()=>a.map((e=>{if(e[t])throw e[t];return e[n]})),u=new Promise((n=>{(_=()=>n(c)).r=0;var t=e=>e!==i&&!f.has(e)&&(f.add(e),e&&!e.d&&(_.r++,e.push(_)));a.map((n=>n[e](t)))}));return _.r?u:c()}),(e=>(e?s(d[t]=e):u(b),r(i)))),i&&i.d<0&&(i.d=0)},c.d=(e,n)=>{for(var t in n)c.o(n,t)&&!c.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},c.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),c.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),c.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),c.v=(e,n,t,r)=>{var o=fetch(c.p+""+t+".module.wasm"),_=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)));return o.then((n=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(n,r).then((n=>Object.assign(e,n.instance.exports)),(e=>{if("application/wasm"!==n.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),_();throw e})):_()))},(()=>{var e;c.g.importScripts&&(e=c.g.location+"");var n=c.g.document;if(!e&&n&&(n.currentScript&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");if(t.length)for(var r=t.length-1;r>-1&&!e;)e=t[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),c.p=e})(),c(9064)})();