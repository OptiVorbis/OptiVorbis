(()=>{"use strict";var e,n,t,r,o={331:(e,n,t)=>{t.a(e,(async(e,r)=>{try{t.d(n,{u6:()=>i.u6});var o=t(4464),i=t(5856),_=e([o]);o=(_.then?(await _)():_)[0],(0,i.o9)(o),o.__wbindgen_start(),r()}catch(e){r(e)}}))},5856:(e,n,t)=>{let r;function o(e){r=e}t.d(n,{AD:()=>Z,AV:()=>ee,Ax:()=>N,Az:()=>B,CK:()=>W,E5:()=>Y,Gy:()=>V,IP:()=>G,Ij:()=>ie,Ir:()=>E,KK:()=>k,KY:()=>X,M$:()=>I,MV:()=>te,OE:()=>P,QB:()=>J,Qp:()=>U,SW:()=>O,Sk:()=>z,UB:()=>q,Up:()=>ne,WU:()=>L,_8:()=>T,aw:()=>re,cK:()=>D,cb:()=>M,cf:()=>K,cm:()=>$,d0:()=>oe,g$:()=>A,h7:()=>j,im:()=>F,me:()=>x,o9:()=>o,qs:()=>Q,sH:()=>S,u6:()=>v,w1:()=>H,wE:()=>R,zz:()=>C}),e=t.hmd(e);let i=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});i.decode();let _=null;function c(){return null!==_&&0!==_.byteLength||(_=new Uint8Array(r.memory.buffer)),_}function a(e,n){return e>>>=0,i.decode(c().subarray(e,e+n))}const u=new Array(128).fill(void 0);u.push(void 0,null,!0,!1);let s=u.length;function f(e){s===u.length&&u.push(u.length+1);const n=s;return s=u[n],u[n]=e,n}function b(e){return u[e]}function d(e){const n=b(e);return function(e){e<132||(u[e]=s,s=e)}(e),n}let g=0,l=null;function w(){return null!==l&&0!==l.byteLength||(l=new Int32Array(r.memory.buffer)),l}let p=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const y="function"==typeof p.encodeInto?function(e,n){return p.encodeInto(e,n)}:function(e,n){const t=p.encode(e);return n.set(t),{read:e.length,written:t.length}};function h(e,n){try{return e.apply(this,n)}catch(e){r.__wbindgen_export_3(f(e))}}const m="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>r.__wbg_oggtoogg_free(e>>>0)));class v{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,m.unregister(this),e}free(){const e=this.__destroy_into_raw();r.__wbg_oggtoogg_free(e)}constructor(){const e=r.oggtoogg_new_with_defaults();return this.__wbg_ptr=e>>>0,this}remux(e){try{const u=r.__wbindgen_add_to_stack_pointer(-16),s=function(e,n){const t=n(1*e.length,1)>>>0;return c().set(e,t/1),g=e.length,t}(e,r.__wbindgen_export_0),f=g;r.oggtoogg_remux(u,this.__wbg_ptr,s,f);var n=w()[u/4+0],t=w()[u/4+1],o=w()[u/4+2];if(w()[u/4+3])throw d(o);var i=(_=n,a=t,_>>>=0,c().subarray(_/1,_/1+a)).slice();return r.__wbindgen_export_1(n,1*t,1),i}finally{r.__wbindgen_add_to_stack_pointer(16)}var _,a}}function x(e,n){return f(a(e,n))}function A(e){d(e)}function S(e,n,t,r){console.debug(b(e),b(n),b(t),b(r))}function E(e,n,t,r){console.error(b(e),b(n),b(t),b(r))}function j(e,n,t,r){console.info(b(e),b(n),b(t),b(r))}function k(e,n,t,r){console.log(b(e),b(n),b(t),b(r))}function T(e,n,t,r){console.warn(b(e),b(n),b(t),b(r))}function U(){return f(new Error)}function M(e,n){const t=function(e,n,t){if(void 0===t){const t=p.encode(e),r=n(t.length,1)>>>0;return c().subarray(r,r+t.length).set(t),g=t.length,r}let r=e.length,o=n(r,1)>>>0;const i=c();let _=0;for(;_<r;_++){const n=e.charCodeAt(_);if(n>127)break;i[o+_]=n}if(_!==r){0!==_&&(e=e.slice(_)),o=t(o,r,r=_+3*e.length,1)>>>0;const n=c().subarray(o+_,o+r);_+=y(e,n).written,o=t(o,r,_,1)>>>0}return g=_,o}(b(n).stack,r.__wbindgen_export_0,r.__wbindgen_export_2),o=g;w()[e/4+1]=o,w()[e/4+0]=t}function O(e,n){let t,o;try{t=e,o=n,console.error(a(e,n))}finally{r.__wbindgen_export_1(t,o,1)}}function I(){return h((function(e,n){b(e).getRandomValues(b(n))}),arguments)}function K(){return h((function(e,n){b(e).randomFillSync(d(n))}),arguments)}function B(e){return f(b(e).crypto)}function W(e){const n=b(e);return"object"==typeof n&&null!==n}function q(e){return f(b(e).process)}function z(e){return f(b(e).versions)}function F(e){return f(b(e).node)}function P(e){return"string"==typeof b(e)}function $(e){return f(b(e).msCrypto)}function C(){return h((function(){return f(e.require)}),arguments)}function V(e){return"function"==typeof b(e)}function D(e,n){return f(new Function(a(e,n)))}function R(){return h((function(e,n){return f(b(e).call(b(n)))}),arguments)}function Q(e){return f(b(e))}function L(){return h((function(e,n,t){return f(b(e).call(b(n),b(t)))}),arguments)}function G(e){return f(b(e).buffer)}function H(){return h((function(){return f(self.self)}),arguments)}function Y(){return h((function(){return f(window.window)}),arguments)}function N(){return h((function(){return f(globalThis.globalThis)}),arguments)}function J(){return h((function(){return f(t.g.global)}),arguments)}function X(e){return void 0===b(e)}function Z(e,n,t){return f(new Uint8Array(b(e),n>>>0,t>>>0))}function ee(e){return f(new Uint8Array(b(e)))}function ne(e,n,t){b(e).set(b(n),t>>>0)}function te(e){return f(new Uint8Array(e>>>0))}function re(e,n,t){return f(b(e).subarray(n>>>0,t>>>0))}function oe(e,n){throw new Error(a(e,n))}function ie(){return f(r.memory)}},9064:(e,n,t)=>{t.a(e,(async(e,n)=>{try{var r=t(331),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const n=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),t=(new r.u6).remux(n);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),n()}catch(e){n(e)}}))},4464:(e,n,t)=>{var r=t(5856);e.exports=t.v(n,e.id,"93ffef85f602c65505ca",{"./index_bg.js":{__wbindgen_string_new:r.me,__wbindgen_object_drop_ref:r.g$,__wbg_debug_9b8701f894da9929:r.sH,__wbg_error_d9bce418caafb712:r.Ir,__wbg_info_bb52f40b06f679de:r.h7,__wbg_log_ea7093e35e3efd07:r.KK,__wbg_warn_dfc0e0cf544a13bd:r._8,__wbg_new_abda76e883ba8a5f:r.Qp,__wbg_stack_658279fe44541cf6:r.cb,__wbg_error_f851667af71bcfc6:r.SW,__wbg_getRandomValues_7e42b4fb8779dc6d:r.M$,__wbg_randomFillSync_b70ccbdf4926a99d:r.cf,__wbg_crypto_d05b68a3572bb8ca:r.Az,__wbindgen_is_object:r.CK,__wbg_process_b02b3570280d0366:r.UB,__wbg_versions_c1cb42213cedf0f5:r.Sk,__wbg_node_43b1089f407e4ec2:r.im,__wbindgen_is_string:r.OE,__wbg_msCrypto_10fc94afee92bd76:r.cm,__wbg_require_9a7e0f667ead4995:r.zz,__wbindgen_is_function:r.Gy,__wbg_newnoargs_581967eacc0e2604:r.cK,__wbg_call_cb65541d95d71282:r.wE,__wbindgen_object_clone_ref:r.qs,__wbg_call_01734de55d61e11d:r.WU,__wbg_buffer_085ec1f694018c4f:r.IP,__wbg_self_1ff1d729e9aae938:r.w1,__wbg_window_5f4faef6c12b79ec:r.E5,__wbg_globalThis_1d39714405582d3c:r.Ax,__wbg_global_651f05c6a0944d1c:r.QB,__wbindgen_is_undefined:r.KY,__wbg_newwithbyteoffsetandlength_6da8e527659b86aa:r.AD,__wbg_new_8125e318e6245eed:r.AV,__wbg_set_5cf90238115182c3:r.Up,__wbg_newwithlength_e5d69174d6984cd7:r.MV,__wbg_subarray_13db269f57aa838d:r.aw,__wbindgen_throw:r.d0,__wbindgen_memory:r.Ij}})}},i={};function _(e){var n=i[e];if(void 0!==n)return n.exports;var t=i[e]={id:e,loaded:!1,exports:{}};return o[e](t,t.exports,_),t.loaded=!0,t.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},_.a=(o,i,_)=>{var c;_&&((c=[]).d=-1);var a,u,s,f=new Set,b=o.exports,d=new Promise(((e,n)=>{s=n,u=e}));d[n]=b,d[e]=e=>(c&&e(c),f.forEach(e),d.catch((e=>{}))),o.exports=d,i((o=>{var i;a=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var i=[];i.d=0,o.then((e=>{_[n]=e,r(i)}),(e=>{_[t]=e,r(i)}));var _={};return _[e]=e=>e(i),_}}var c={};return c[e]=e=>{},c[n]=o,c})))(o);var _=()=>a.map((e=>{if(e[t])throw e[t];return e[n]})),u=new Promise((n=>{(i=()=>n(_)).r=0;var t=e=>e!==c&&!f.has(e)&&(f.add(e),e&&!e.d&&(i.r++,e.push(i)));a.map((n=>n[e](t)))}));return i.r?u:_()}),(e=>(e?s(d[t]=e):u(b),r(c)))),c&&c.d<0&&(c.d=0)},_.d=(e,n)=>{for(var t in n)_.o(n,t)&&!_.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},_.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),_.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),_.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),_.v=(e,n,t,r)=>{var o=fetch(_.p+""+t+".module.wasm"),i=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)));return o.then((n=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(n,r).then((n=>Object.assign(e,n.instance.exports)),(e=>{if("application/wasm"!==n.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),i();throw e})):i()))},(()=>{var e;_.g.importScripts&&(e=_.g.location+"");var n=_.g.document;if(!e&&n&&(n.currentScript&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");if(t.length)for(var r=t.length-1;r>-1&&!e;)e=t[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),_.p=e})(),_(9064)})();