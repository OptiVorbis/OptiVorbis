(()=>{"use strict";var n,e,t,r,o={4858:(n,e,t)=>{t.a(n,(async(n,e)=>{try{var r=t(1007),o=n([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(n=>{try{const e=new Uint8Array((new FileReaderSync).readAsArrayBuffer(n.data)),t=(new r.ao).remux(e);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(n){self.postMessage(n.toString())}})),e()}catch(n){e(n)}}))},1007:(n,e,t)=>{t.a(n,(async(n,r)=>{try{t.d(e,{ao:()=>_.ao});var o=t(836),_=t(9409),i=n([o]);o=(i.then?(await i)():i)[0],(0,_.lI)(o),o.__wbindgen_start(),r()}catch(n){r(n)}}))},9409:(n,e,t)=>{let r;function o(n){r=n}t.d(e,{$Z:()=>nn,BZ:()=>K,D1:()=>L,FT:()=>S,Fm:()=>H,Gu:()=>R,Kc:()=>Z,Lo:()=>Y,NL:()=>O,PR:()=>$,Pf:()=>G,Py:()=>_n,Qn:()=>on,V5:()=>E,VF:()=>M,Wv:()=>en,Xu:()=>q,_J:()=>k,_m:()=>Q,a0:()=>T,ao:()=>v,bk:()=>A,cA:()=>B,cX:()=>C,cl:()=>V,cq:()=>J,f1:()=>j,hW:()=>D,h_:()=>W,if:()=>rn,kh:()=>X,lI:()=>o,qv:()=>I,s:()=>U,u$:()=>P,uI:()=>F,v6:()=>tn,vA:()=>N,vU:()=>z,yc:()=>x});let _=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});_.decode();let i=null;function c(){return null!==i&&0!==i.byteLength||(i=new Uint8Array(r.memory.buffer)),i}function a(n,e){return n>>>=0,_.decode(c().subarray(n,n+e))}const u=new Array(128).fill(void 0);u.push(void 0,null,!0,!1);let f=u.length;function s(n){f===u.length&&u.push(u.length+1);const e=f;return f=u[e],u[e]=n,e}function b(n){return u[n]}function l(n){const e=b(n);return function(n){n<132||(u[n]=f,f=n)}(n),e}let g=0,w=null;function d(){return null!==w&&0!==w.byteLength||(w=new Int32Array(r.memory.buffer)),w}let p=new("undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder)("utf-8");const y="function"==typeof p.encodeInto?function(n,e){return p.encodeInto(n,e)}:function(n,e){const t=p.encode(n);return e.set(t),{read:n.length,written:t.length}};function h(n,e){try{return n.apply(this,e)}catch(n){r.__wbindgen_export_3(s(n))}}const m="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((n=>r.__wbg_oggtoogg_free(n>>>0)));class v{__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,m.unregister(this),n}free(){const n=this.__destroy_into_raw();r.__wbg_oggtoogg_free(n)}constructor(){const n=r.oggtoogg_new_with_defaults();return this.__wbg_ptr=n>>>0,this}remux(n){try{const u=r.__wbindgen_add_to_stack_pointer(-16),f=function(n,e){const t=e(1*n.length,1)>>>0;return c().set(n,t/1),g=n.length,t}(n,r.__wbindgen_export_0),s=g;r.oggtoogg_remux(u,this.__wbg_ptr,f,s);var e=d()[u/4+0],t=d()[u/4+1],o=d()[u/4+2];if(d()[u/4+3])throw l(o);var _=(i=e,a=t,i>>>=0,c().subarray(i/1,i/1+a)).slice();return r.__wbindgen_export_1(e,1*t,1),_}finally{r.__wbindgen_add_to_stack_pointer(16)}var i,a}}function x(n,e){return s(a(n,e))}function A(n){l(n)}function S(n,e,t,r){console.debug(b(n),b(e),b(t),b(r))}function k(n,e,t,r){console.error(b(n),b(e),b(t),b(r))}function T(n,e,t,r){console.info(b(n),b(e),b(t),b(r))}function F(n,e,t,r){console.log(b(n),b(e),b(t),b(r))}function j(n,e,t,r){console.warn(b(n),b(e),b(t),b(r))}function E(){return s(new Error)}function P(n,e){const t=function(n,e,t){if(void 0===t){const t=p.encode(n),r=e(t.length,1)>>>0;return c().subarray(r,r+t.length).set(t),g=t.length,r}let r=n.length,o=e(r,1)>>>0;const _=c();let i=0;for(;i<r;i++){const e=n.charCodeAt(i);if(e>127)break;_[o+i]=e}if(i!==r){0!==i&&(n=n.slice(i)),o=t(o,r,r=i+3*n.length,1)>>>0;const e=c().subarray(o+i,o+r);i+=y(n,e).written,o=t(o,r,i,1)>>>0}return g=i,o}(b(e).stack,r.__wbindgen_export_0,r.__wbindgen_export_2),o=g;d()[n/4+1]=o,d()[n/4+0]=t}function q(n,e){let t,o;try{t=n,o=e,console.error(a(n,e))}finally{r.__wbindgen_export_1(t,o,1)}}function W(n){return s(b(n).crypto)}function I(n){const e=b(n);return"object"==typeof e&&null!==e}function B(n){return s(b(n).process)}function L(n){return s(b(n).versions)}function O(n){return s(b(n).node)}function R(n){return"string"==typeof b(n)}function U(){return h((function(){return s(module.require)}),arguments)}function $(n){return"function"==typeof b(n)}function V(n){return s(b(n).msCrypto)}function D(){return h((function(n,e){b(n).randomFillSync(l(e))}),arguments)}function M(){return h((function(n,e){b(n).getRandomValues(b(e))}),arguments)}function C(){return h((function(){return s(self.self)}),arguments)}function X(){return h((function(){return s(window.window)}),arguments)}function Z(){return h((function(){return s(globalThis.globalThis)}),arguments)}function N(){return h((function(){return s(global.global)}),arguments)}function z(n){return void 0===b(n)}function G(n,e){return s(new Function(a(n,e)))}function J(){return h((function(n,e){return s(b(n).call(b(e)))}),arguments)}function K(n){return s(b(n))}function Q(){return h((function(n,e,t){return s(b(n).call(b(e),b(t)))}),arguments)}function H(n){return s(b(n).buffer)}function Y(n,e,t){return s(new Uint8Array(b(n),e>>>0,t>>>0))}function nn(n){return s(new Uint8Array(b(n)))}function en(n,e,t){b(n).set(b(e),t>>>0)}function tn(n){return s(new Uint8Array(n>>>0))}function rn(n,e,t){return s(b(n).subarray(e>>>0,t>>>0))}function on(n,e){throw new Error(a(n,e))}function _n(){return s(r.memory)}},836:(n,e,t)=>{var r=t(9409);n.exports=t.v(e,n.id,"77a7c9f716a23b959979",{"./index_bg.js":{__wbindgen_string_new:r.yc,__wbindgen_object_drop_ref:r.bk,__wbg_debug_7d879afce6cf56cb:r.FT,__wbg_error_696630710900ec44:r._J,__wbg_info_80803d9a3f0aad16:r.a0,__wbg_log_151eb4333ef0fe39:r.uI,__wbg_warn_5d3f783b0bae8943:r.f1,__wbg_new_abda76e883ba8a5f:r.V5,__wbg_stack_658279fe44541cf6:r.u$,__wbg_error_f851667af71bcfc6:r.Xu,__wbg_crypto_1d1f22824a6a080c:r.h_,__wbindgen_is_object:r.qv,__wbg_process_4a72847cc503995b:r.cA,__wbg_versions_f686565e586dd935:r.D1,__wbg_node_104a2ff8d6ea03a2:r.NL,__wbindgen_is_string:r.Gu,__wbg_require_cca90b1a94a0255b:r.s,__wbindgen_is_function:r.PR,__wbg_msCrypto_eb05e62b530a1508:r.cl,__wbg_randomFillSync_5c9c955aa56b6049:r.hW,__wbg_getRandomValues_3aa56aa6edec874c:r.VF,__wbg_self_ce0dbfc45cf2f5be:r.cX,__wbg_window_c6fb939a7f436783:r.kh,__wbg_globalThis_d1e6af4856ba331b:r.Kc,__wbg_global_207b558942527489:r.vA,__wbindgen_is_undefined:r.vU,__wbg_newnoargs_e258087cd0daa0ea:r.Pf,__wbg_call_27c0f87801dedf93:r.cq,__wbindgen_object_clone_ref:r.BZ,__wbg_call_b3ca7c6051f9bec1:r._m,__wbg_buffer_12d079cc21e14bdb:r.Fm,__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb:r.Lo,__wbg_new_63b92bc8671ed464:r.$Z,__wbg_set_a47bac70306a19a7:r.Wv,__wbg_newwithlength_e9b4878cebadb3d3:r.v6,__wbg_subarray_a1f73cd4b5b42fe1:r.if,__wbindgen_throw:r.Qn,__wbindgen_memory:r.Py}})}},_={};function i(n){var e=_[n];if(void 0!==e)return e.exports;var t=_[n]={id:n,exports:{}};return o[n](t,t.exports,i),t.exports}n="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",e="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=n=>{n&&n.d<1&&(n.d=1,n.forEach((n=>n.r--)),n.forEach((n=>n.r--?n.r++:n())))},i.a=(o,_,i)=>{var c;i&&((c=[]).d=-1);var a,u,f,s=new Set,b=o.exports,l=new Promise(((n,e)=>{f=e,u=n}));l[e]=b,l[n]=n=>(c&&n(c),s.forEach(n),l.catch((n=>{}))),o.exports=l,_((o=>{var _;a=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[n])return o;if(o.then){var _=[];_.d=0,o.then((n=>{i[e]=n,r(_)}),(n=>{i[t]=n,r(_)}));var i={};return i[n]=n=>n(_),i}}var c={};return c[n]=n=>{},c[e]=o,c})))(o);var i=()=>a.map((n=>{if(n[t])throw n[t];return n[e]})),u=new Promise((e=>{(_=()=>e(i)).r=0;var t=n=>n!==c&&!s.has(n)&&(s.add(n),n&&!n.d&&(_.r++,n.push(_)));a.map((e=>e[n](t)))}));return _.r?u:i()}),(n=>(n?f(l[t]=n):u(b),r(c)))),c&&c.d<0&&(c.d=0)},i.d=(n,e)=>{for(var t in e)i.o(e,t)&&!i.o(n,t)&&Object.defineProperty(n,t,{enumerable:!0,get:e[t]})},i.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(n){if("object"==typeof window)return window}}(),i.o=(n,e)=>Object.prototype.hasOwnProperty.call(n,e),i.v=(n,e,t,r)=>{var o=fetch(i.p+""+t+".module.wasm"),_=()=>o.then((n=>n.arrayBuffer())).then((n=>WebAssembly.instantiate(n,r))).then((e=>Object.assign(n,e.instance.exports)));return o.then((e=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(e,r).then((e=>Object.assign(n,e.instance.exports)),(n=>{if("application/wasm"!==e.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",n),_();throw n})):_()))},(()=>{var n;i.g.importScripts&&(n=i.g.location+"");var e=i.g.document;if(!n&&e&&(e.currentScript&&(n=e.currentScript.src),!n)){var t=e.getElementsByTagName("script");if(t.length)for(var r=t.length-1;r>-1&&(!n||!/^http(s?):/.test(n));)n=t[r--].src}if(!n)throw new Error("Automatic publicPath is not supported in this browser");n=n.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),i.p=n})(),i(4858)})();