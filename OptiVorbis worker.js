(()=>{"use strict";var e,n,t,r,o={8235:(e,n,t)=>{t.a(e,(async(e,r)=>{try{t.d(n,{ao:()=>i.ao});var o=t(836),i=t(3044),_=e([o]);o=(_.then?(await _)():_)[0],(0,i.lI)(o),o.__wbindgen_start(),r()}catch(e){r(e)}}))},3044:(e,n,t)=>{let r;function o(e){r=e}t.d(n,{$Z:()=>ee,A8:()=>I,BZ:()=>Z,Ch:()=>X,FT:()=>S,Fm:()=>z,Gu:()=>W,HX:()=>q,Js:()=>$,Kc:()=>K,Lo:()=>Y,PR:()=>C,Pf:()=>J,Py:()=>ie,Qn:()=>oe,V5:()=>F,Wv:()=>ne,Xu:()=>U,_J:()=>k,_U:()=>R,_m:()=>D,a0:()=>E,ao:()=>v,bk:()=>A,cX:()=>G,cq:()=>V,dl:()=>M,f1:()=>j,iU:()=>O,if:()=>re,kh:()=>H,lI:()=>o,qv:()=>B,sL:()=>L,u$:()=>P,uI:()=>T,v6:()=>te,vA:()=>Q,vU:()=>N,yc:()=>x}),e=t.hmd(e);let i=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});i.decode();let _=null;function c(){return null!==_&&0!==_.byteLength||(_=new Uint8Array(r.memory.buffer)),_}function a(e,n){return e>>>=0,i.decode(c().subarray(e,e+n))}const u=new Array(128).fill(void 0);u.push(void 0,null,!0,!1);let s=u.length;function f(e){s===u.length&&u.push(u.length+1);const n=s;return s=u[n],u[n]=e,n}function b(e){return u[e]}function d(e){const n=b(e);return function(e){e<132||(u[e]=s,s=e)}(e),n}let l=0,g=null;function w(){return null!==g&&0!==g.byteLength||(g=new Int32Array(r.memory.buffer)),g}let p=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const y="function"==typeof p.encodeInto?function(e,n){return p.encodeInto(e,n)}:function(e,n){const t=p.encode(e);return n.set(t),{read:e.length,written:t.length}};function h(e,n){try{return e.apply(this,n)}catch(e){r.__wbindgen_export_3(f(e))}}const m="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>r.__wbg_oggtoogg_free(e>>>0)));class v{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,m.unregister(this),e}free(){const e=this.__destroy_into_raw();r.__wbg_oggtoogg_free(e)}constructor(){const e=r.oggtoogg_new_with_defaults();return this.__wbg_ptr=e>>>0,this}remux(e){try{const u=r.__wbindgen_add_to_stack_pointer(-16),s=function(e,n){const t=n(1*e.length,1)>>>0;return c().set(e,t/1),l=e.length,t}(e,r.__wbindgen_export_0),f=l;r.oggtoogg_remux(u,this.__wbg_ptr,s,f);var n=w()[u/4+0],t=w()[u/4+1],o=w()[u/4+2];if(w()[u/4+3])throw d(o);var i=(_=n,a=t,_>>>=0,c().subarray(_/1,_/1+a)).slice();return r.__wbindgen_export_1(n,1*t,1),i}finally{r.__wbindgen_add_to_stack_pointer(16)}var _,a}}function x(e,n){return f(a(e,n))}function A(e){d(e)}function S(e,n,t,r){console.debug(b(e),b(n),b(t),b(r))}function k(e,n,t,r){console.error(b(e),b(n),b(t),b(r))}function E(e,n,t,r){console.info(b(e),b(n),b(t),b(r))}function T(e,n,t,r){console.log(b(e),b(n),b(t),b(r))}function j(e,n,t,r){console.warn(b(e),b(n),b(t),b(r))}function F(){return f(new Error)}function P(e,n){const t=function(e,n,t){if(void 0===t){const t=p.encode(e),r=n(t.length,1)>>>0;return c().subarray(r,r+t.length).set(t),l=t.length,r}let r=e.length,o=n(r,1)>>>0;const i=c();let _=0;for(;_<r;_++){const n=e.charCodeAt(_);if(n>127)break;i[o+_]=n}if(_!==r){0!==_&&(e=e.slice(_)),o=t(o,r,r=_+3*e.length,1)>>>0;const n=c().subarray(o+_,o+r);_+=y(e,n).written,o=t(o,r,_,1)>>>0}return l=_,o}(b(n).stack,r.__wbindgen_export_0,r.__wbindgen_export_2),o=l;w()[e/4+1]=o,w()[e/4+0]=t}function U(e,n){let t,o;try{t=e,o=n,console.error(a(e,n))}finally{r.__wbindgen_export_1(t,o,1)}}function q(){return h((function(e,n){b(e).getRandomValues(b(n))}),arguments)}function O(){return h((function(e,n){b(e).randomFillSync(d(n))}),arguments)}function I(e){return f(b(e).crypto)}function B(e){const n=b(e);return"object"==typeof n&&null!==n}function L(e){return f(b(e).process)}function M(e){return f(b(e).versions)}function R(e){return f(b(e).node)}function W(e){return"string"==typeof b(e)}function $(){return h((function(){return f(e.require)}),arguments)}function C(e){return"function"==typeof b(e)}function X(e){return f(b(e).msCrypto)}function J(e,n){return f(new Function(a(e,n)))}function V(){return h((function(e,n){return f(b(e).call(b(n)))}),arguments)}function Z(e){return f(b(e))}function D(){return h((function(e,n,t){return f(b(e).call(b(n),b(t)))}),arguments)}function z(e){return f(b(e).buffer)}function G(){return h((function(){return f(self.self)}),arguments)}function H(){return h((function(){return f(window.window)}),arguments)}function K(){return h((function(){return f(globalThis.globalThis)}),arguments)}function Q(){return h((function(){return f(t.g.global)}),arguments)}function N(e){return void 0===b(e)}function Y(e,n,t){return f(new Uint8Array(b(e),n>>>0,t>>>0))}function ee(e){return f(new Uint8Array(b(e)))}function ne(e,n,t){b(e).set(b(n),t>>>0)}function te(e){return f(new Uint8Array(e>>>0))}function re(e,n,t){return f(b(e).subarray(n>>>0,t>>>0))}function oe(e,n){throw new Error(a(e,n))}function ie(){return f(r.memory)}},4858:(e,n,t)=>{t.a(e,(async(e,n)=>{try{var r=t(8235),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const n=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),t=(new r.ao).remux(n);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),n()}catch(e){n(e)}}))},836:(e,n,t)=>{var r=t(3044);e.exports=t.v(n,e.id,"5305449f0d98e1692168",{"./index_bg.js":{__wbindgen_string_new:r.yc,__wbindgen_object_drop_ref:r.bk,__wbg_debug_7d879afce6cf56cb:r.FT,__wbg_error_696630710900ec44:r._J,__wbg_info_80803d9a3f0aad16:r.a0,__wbg_log_151eb4333ef0fe39:r.uI,__wbg_warn_5d3f783b0bae8943:r.f1,__wbg_new_abda76e883ba8a5f:r.V5,__wbg_stack_658279fe44541cf6:r.u$,__wbg_error_f851667af71bcfc6:r.Xu,__wbg_getRandomValues_260cc23a41afad9a:r.HX,__wbg_randomFillSync_290977693942bf03:r.iU,__wbg_crypto_566d7465cdbb6b7a:r.A8,__wbindgen_is_object:r.qv,__wbg_process_dc09a8c7d59982f6:r.sL,__wbg_versions_d98c6400c6ca2bd8:r.dl,__wbg_node_caaf83d002149bd5:r._U,__wbindgen_is_string:r.Gu,__wbg_require_94a9da52636aacbf:r.Js,__wbindgen_is_function:r.PR,__wbg_msCrypto_0b84745e9245cdf6:r.Ch,__wbg_newnoargs_e258087cd0daa0ea:r.Pf,__wbg_call_27c0f87801dedf93:r.cq,__wbindgen_object_clone_ref:r.BZ,__wbg_call_b3ca7c6051f9bec1:r._m,__wbg_buffer_12d079cc21e14bdb:r.Fm,__wbg_self_ce0dbfc45cf2f5be:r.cX,__wbg_window_c6fb939a7f436783:r.kh,__wbg_globalThis_d1e6af4856ba331b:r.Kc,__wbg_global_207b558942527489:r.vA,__wbindgen_is_undefined:r.vU,__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb:r.Lo,__wbg_new_63b92bc8671ed464:r.$Z,__wbg_set_a47bac70306a19a7:r.Wv,__wbg_newwithlength_e9b4878cebadb3d3:r.v6,__wbg_subarray_a1f73cd4b5b42fe1:r.if,__wbindgen_throw:r.Qn,__wbindgen_memory:r.Py}})}},i={};function _(e){var n=i[e];if(void 0!==n)return n.exports;var t=i[e]={id:e,loaded:!1,exports:{}};return o[e](t,t.exports,_),t.loaded=!0,t.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},_.a=(o,i,_)=>{var c;_&&((c=[]).d=-1);var a,u,s,f=new Set,b=o.exports,d=new Promise(((e,n)=>{s=n,u=e}));d[n]=b,d[e]=e=>(c&&e(c),f.forEach(e),d.catch((e=>{}))),o.exports=d,i((o=>{var i;a=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var i=[];i.d=0,o.then((e=>{_[n]=e,r(i)}),(e=>{_[t]=e,r(i)}));var _={};return _[e]=e=>e(i),_}}var c={};return c[e]=e=>{},c[n]=o,c})))(o);var _=()=>a.map((e=>{if(e[t])throw e[t];return e[n]})),u=new Promise((n=>{(i=()=>n(_)).r=0;var t=e=>e!==c&&!f.has(e)&&(f.add(e),e&&!e.d&&(i.r++,e.push(i)));a.map((n=>n[e](t)))}));return i.r?u:_()}),(e=>(e?s(d[t]=e):u(b),r(c)))),c&&c.d<0&&(c.d=0)},_.d=(e,n)=>{for(var t in n)_.o(n,t)&&!_.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},_.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),_.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),_.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),_.v=(e,n,t,r)=>{var o=fetch(_.p+""+t+".module.wasm"),i=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)));return o.then((n=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(n,r).then((n=>Object.assign(e,n.instance.exports)),(e=>{if("application/wasm"!==n.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),i();throw e})):i()))},(()=>{var e;_.g.importScripts&&(e=_.g.location+"");var n=_.g.document;if(!e&&n&&(n.currentScript&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");if(t.length)for(var r=t.length-1;r>-1&&(!e||!/^http(s?):/.test(e));)e=t[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),_.p=e})(),_(4858)})();