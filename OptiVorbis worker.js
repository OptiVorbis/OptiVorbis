(()=>{"use strict";var e,n,t,r,o={4858:(e,n,t)=>{t.a(e,(async(e,n)=>{try{var r=t(1007),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const n=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),t=(new r.ao).remux(n);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),n()}catch(e){n(e)}}))},1007:(e,n,t)=>{t.a(e,(async(e,r)=>{try{t.d(n,{ao:()=>_.ao});var o=t(836),_=t(9409),i=e([o]);o=(i.then?(await i)():i)[0],(0,_.lI)(o),o.__wbindgen_start(),r()}catch(e){r(e)}}))},9409:(e,n,t)=>{let r;function o(e){r=e}t.d(n,{$Z:()=>ee,BZ:()=>K,D1:()=>U,FT:()=>S,Fm:()=>H,Gu:()=>O,Kc:()=>X,Lo:()=>Y,NL:()=>B,PR:()=>$,Pf:()=>G,Py:()=>_e,Qn:()=>oe,V5:()=>j,VF:()=>L,Wv:()=>ne,Xu:()=>P,_J:()=>k,_m:()=>Q,a0:()=>T,ao:()=>v,bk:()=>A,cA:()=>R,cX:()=>M,cl:()=>C,cq:()=>J,f1:()=>I,hW:()=>D,h_:()=>q,if:()=>re,kh:()=>N,lI:()=>o,qv:()=>W,s:()=>V,u$:()=>E,uI:()=>F,v6:()=>te,vA:()=>Z,vU:()=>z,yc:()=>x});let _=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});_.decode();let i=null;function c(){return null!==i&&0!==i.byteLength||(i=new Uint8Array(r.memory.buffer)),i}function a(e,n){return e>>>=0,_.decode(c().subarray(e,e+n))}const u=new Array(128).fill(void 0);u.push(void 0,null,!0,!1);let f=u.length;function s(e){f===u.length&&u.push(u.length+1);const n=f;return f=u[n],u[n]=e,n}function b(e){return u[e]}function g(e){const n=b(e);return function(e){e<132||(u[e]=f,f=e)}(e),n}let l=0,d=null;function w(){return(null===d||!0===d.buffer.detached||void 0===d.buffer.detached&&d.buffer!==r.memory.buffer)&&(d=new DataView(r.memory.buffer)),d}let p=new("undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder)("utf-8");const h="function"==typeof p.encodeInto?function(e,n){return p.encodeInto(e,n)}:function(e,n){const t=p.encode(e);return n.set(t),{read:e.length,written:t.length}};function y(e,n){try{return e.apply(this,n)}catch(e){r.__wbindgen_export_3(s(e))}}const m="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>r.__wbg_oggtoogg_free(e>>>0,1)));class v{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,m.unregister(this),e}free(){const e=this.__destroy_into_raw();r.__wbg_oggtoogg_free(e,0)}constructor(){const e=r.oggtoogg_new_with_defaults();return this.__wbg_ptr=e>>>0,m.register(this,this.__wbg_ptr,this),this}remux(e){try{const u=r.__wbindgen_add_to_stack_pointer(-16),f=function(e,n){const t=n(1*e.length,1)>>>0;return c().set(e,t/1),l=e.length,t}(e,r.__wbindgen_export_0),s=l;r.oggtoogg_remux(u,this.__wbg_ptr,f,s);var n=w().getInt32(u+0,!0),t=w().getInt32(u+4,!0),o=w().getInt32(u+8,!0);if(w().getInt32(u+12,!0))throw g(o);var _=(i=n,a=t,i>>>=0,c().subarray(i/1,i/1+a)).slice();return r.__wbindgen_export_1(n,1*t,1),_}finally{r.__wbindgen_add_to_stack_pointer(16)}var i,a}}function x(e,n){return s(a(e,n))}function A(e){g(e)}function S(e,n,t,r){console.debug(b(e),b(n),b(t),b(r))}function k(e,n,t,r){console.error(b(e),b(n),b(t),b(r))}function T(e,n,t,r){console.info(b(e),b(n),b(t),b(r))}function F(e,n,t,r){console.log(b(e),b(n),b(t),b(r))}function I(e,n,t,r){console.warn(b(e),b(n),b(t),b(r))}function j(){return s(new Error)}function E(e,n){const t=function(e,n,t){if(void 0===t){const t=p.encode(e),r=n(t.length,1)>>>0;return c().subarray(r,r+t.length).set(t),l=t.length,r}let r=e.length,o=n(r,1)>>>0;const _=c();let i=0;for(;i<r;i++){const n=e.charCodeAt(i);if(n>127)break;_[o+i]=n}if(i!==r){0!==i&&(e=e.slice(i)),o=t(o,r,r=i+3*e.length,1)>>>0;const n=c().subarray(o+i,o+r);i+=h(e,n).written,o=t(o,r,i,1)>>>0}return l=i,o}(b(n).stack,r.__wbindgen_export_0,r.__wbindgen_export_2),o=l;w().setInt32(e+4,o,!0),w().setInt32(e+0,t,!0)}function P(e,n){let t,o;try{t=e,o=n,console.error(a(e,n))}finally{r.__wbindgen_export_1(t,o,1)}}function q(e){return s(b(e).crypto)}function W(e){const n=b(e);return"object"==typeof n&&null!==n}function R(e){return s(b(e).process)}function U(e){return s(b(e).versions)}function B(e){return s(b(e).node)}function O(e){return"string"==typeof b(e)}function V(){return y((function(){return s(module.require)}),arguments)}function $(e){return"function"==typeof b(e)}function C(e){return s(b(e).msCrypto)}function D(){return y((function(e,n){b(e).randomFillSync(g(n))}),arguments)}function L(){return y((function(e,n){b(e).getRandomValues(b(n))}),arguments)}function M(){return y((function(){return s(self.self)}),arguments)}function N(){return y((function(){return s(window.window)}),arguments)}function X(){return y((function(){return s(globalThis.globalThis)}),arguments)}function Z(){return y((function(){return s(global.global)}),arguments)}function z(e){return void 0===b(e)}function G(e,n){return s(new Function(a(e,n)))}function J(){return y((function(e,n){return s(b(e).call(b(n)))}),arguments)}function K(e){return s(b(e))}function Q(){return y((function(e,n,t){return s(b(e).call(b(n),b(t)))}),arguments)}function H(e){return s(b(e).buffer)}function Y(e,n,t){return s(new Uint8Array(b(e),n>>>0,t>>>0))}function ee(e){return s(new Uint8Array(b(e)))}function ne(e,n,t){b(e).set(b(n),t>>>0)}function te(e){return s(new Uint8Array(e>>>0))}function re(e,n,t){return s(b(e).subarray(n>>>0,t>>>0))}function oe(e,n){throw new Error(a(e,n))}function _e(){return s(r.memory)}},836:(e,n,t)=>{var r=t(9409);e.exports=t.v(n,e.id,"8f5b456406cd08079b9a",{"./index_bg.js":{__wbindgen_string_new:r.yc,__wbindgen_object_drop_ref:r.bk,__wbg_debug_7d879afce6cf56cb:r.FT,__wbg_error_696630710900ec44:r._J,__wbg_info_80803d9a3f0aad16:r.a0,__wbg_log_151eb4333ef0fe39:r.uI,__wbg_warn_5d3f783b0bae8943:r.f1,__wbg_new_abda76e883ba8a5f:r.V5,__wbg_stack_658279fe44541cf6:r.u$,__wbg_error_f851667af71bcfc6:r.Xu,__wbg_crypto_1d1f22824a6a080c:r.h_,__wbindgen_is_object:r.qv,__wbg_process_4a72847cc503995b:r.cA,__wbg_versions_f686565e586dd935:r.D1,__wbg_node_104a2ff8d6ea03a2:r.NL,__wbindgen_is_string:r.Gu,__wbg_require_cca90b1a94a0255b:r.s,__wbindgen_is_function:r.PR,__wbg_msCrypto_eb05e62b530a1508:r.cl,__wbg_randomFillSync_5c9c955aa56b6049:r.hW,__wbg_getRandomValues_3aa56aa6edec874c:r.VF,__wbg_self_ce0dbfc45cf2f5be:r.cX,__wbg_window_c6fb939a7f436783:r.kh,__wbg_globalThis_d1e6af4856ba331b:r.Kc,__wbg_global_207b558942527489:r.vA,__wbindgen_is_undefined:r.vU,__wbg_newnoargs_e258087cd0daa0ea:r.Pf,__wbg_call_27c0f87801dedf93:r.cq,__wbindgen_object_clone_ref:r.BZ,__wbg_call_b3ca7c6051f9bec1:r._m,__wbg_buffer_12d079cc21e14bdb:r.Fm,__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb:r.Lo,__wbg_new_63b92bc8671ed464:r.$Z,__wbg_set_a47bac70306a19a7:r.Wv,__wbg_newwithlength_e9b4878cebadb3d3:r.v6,__wbg_subarray_a1f73cd4b5b42fe1:r.if,__wbindgen_throw:r.Qn,__wbindgen_memory:r.Py}})}},_={};function i(e){var n=_[e];if(void 0!==n)return n.exports;var t=_[e]={id:e,exports:{}};return o[e](t,t.exports,i),t.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},i.a=(o,_,i)=>{var c;i&&((c=[]).d=-1);var a,u,f,s=new Set,b=o.exports,g=new Promise(((e,n)=>{f=n,u=e}));g[n]=b,g[e]=e=>(c&&e(c),s.forEach(e),g.catch((e=>{}))),o.exports=g,_((o=>{var _;a=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var _=[];_.d=0,o.then((e=>{i[n]=e,r(_)}),(e=>{i[t]=e,r(_)}));var i={};return i[e]=e=>e(_),i}}var c={};return c[e]=e=>{},c[n]=o,c})))(o);var i=()=>a.map((e=>{if(e[t])throw e[t];return e[n]})),u=new Promise((n=>{(_=()=>n(i)).r=0;var t=e=>e!==c&&!s.has(e)&&(s.add(e),e&&!e.d&&(_.r++,e.push(_)));a.map((n=>n[e](t)))}));return _.r?u:i()}),(e=>(e?f(g[t]=e):u(b),r(c)))),c&&c.d<0&&(c.d=0)},i.d=(e,n)=>{for(var t in n)i.o(n,t)&&!i.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},i.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),i.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),i.v=(e,n,t,r)=>{var o=fetch(i.p+""+t+".module.wasm"),_=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)));return o.then((n=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(n,r).then((n=>Object.assign(e,n.instance.exports)),(e=>{if("application/wasm"!==n.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),_();throw e})):_()))},(()=>{var e;i.g.importScripts&&(e=i.g.location+"");var n=i.g.document;if(!e&&n&&(n.currentScript&&"SCRIPT"===n.currentScript.tagName.toUpperCase()&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");if(t.length)for(var r=t.length-1;r>-1&&(!e||!/^http(s?):/.test(e));)e=t[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),i.p=e})(),i(4858)})();