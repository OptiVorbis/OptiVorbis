(()=>{"use strict";var e,n,t,r,o={3118:(e,n,t)=>{t.a(e,(async(r,o)=>{try{t.d(n,{$H:()=>X,AM:()=>N,Ee:()=>ie,J2:()=>U,JC:()=>H,KM:()=>W,MF:()=>Y,Mk:()=>J,OP:()=>$,Or:()=>ae,PZ:()=>ee,R6:()=>k,T_:()=>I,U6:()=>z,WU:()=>re,Wl:()=>D,XP:()=>te,XU:()=>ne,Xf:()=>T,Zu:()=>A,a2:()=>F,dp:()=>B,eY:()=>G,ej:()=>C,fG:()=>Q,fd:()=>ce,h4:()=>O,iX:()=>q,jG:()=>K,lL:()=>P,m_:()=>V,mu:()=>_e,o2:()=>oe,oH:()=>ue,ug:()=>M,wm:()=>Z,xn:()=>R,yP:()=>L});var _=t(2449);e=t.hmd(e);var c=r([_]);_=(c.then?(await c)():c)[0];let i=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});i.decode();let a=new Uint8Array;function u(){return 0===a.byteLength&&(a=new Uint8Array(_.memory.buffer)),a}function f(e,n){return i.decode(u().subarray(e,e+n))}const s=new Array(32).fill(void 0);s.push(void 0,null,!0,!1);let d=s.length;function b(e){d===s.length&&s.push(s.length+1);const n=d;return d=s[n],s[n]=e,n}function g(e){return s[e]}function l(e){e<36||(s[e]=d,d=e)}function w(e){const n=g(e);return l(e),n}let p=0;function y(e,n){const t=n(1*e.length);return u().set(e,t/1),p=e.length,t}let h=new Int32Array;function m(){return 0===h.byteLength&&(h=new Int32Array(_.memory.buffer)),h}function x(e,n){return u().subarray(e/1,e/1+n)}let v=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const j="function"==typeof v.encodeInto?function(e,n){return v.encodeInto(e,n)}:function(e,n){const t=v.encode(e);return n.set(t),{read:e.length,written:t.length}};function E(e,n,t){if(void 0===t){const t=v.encode(e),r=n(t.length);return u().subarray(r,r+t.length).set(t),p=t.length,r}let r=e.length,o=n(r);const _=u();let c=0;for(;c<r;c++){const n=e.charCodeAt(c);if(n>127)break;_[o+c]=n}if(c!==r){0!==c&&(e=e.slice(c)),o=t(o,r,r=c+3*e.length);const n=u().subarray(o+c,o+r);c+=j(e,n).written}return p=c,o}function S(e,n){try{return e.apply(this,n)}catch(e){_.__wbindgen_export_3(b(e))}}class A{static __wrap(e){const n=Object.create(A.prototype);return n.ptr=e,n}__destroy_into_raw(){const e=this.ptr;return this.ptr=0,e}free(){const e=this.__destroy_into_raw();_.__wbg_oggtoogg_free(e)}constructor(){const e=_.oggtoogg_new_with_defaults();return A.__wrap(e)}remux(e){try{const c=_.__wbindgen_add_to_stack_pointer(-16),i=y(e,_.__wbindgen_export_0),a=p;_.oggtoogg_remux(c,this.ptr,i,a);var n=m()[c/4+0],t=m()[c/4+1],r=m()[c/4+2];if(m()[c/4+3])throw w(r);var o=x(n,t).slice();return _.__wbindgen_export_1(n,1*t),o}finally{_.__wbindgen_add_to_stack_pointer(16)}}}function O(e,n){return b(f(e,n))}function M(e){w(e)}function P(e,n,t,r){console.debug(g(e),g(n),g(t),g(r))}function T(e,n,t,r){console.error(g(e),g(n),g(t),g(r))}function k(e,n,t,r){console.info(g(e),g(n),g(t),g(r))}function U(e,n,t,r){console.log(g(e),g(n),g(t),g(r))}function X(e,n,t,r){console.warn(g(e),g(n),g(t),g(r))}function F(){return b(new Error)}function W(e,n){const t=E(g(n).stack,_.__wbindgen_export_0,_.__wbindgen_export_2),r=p;m()[e/4+1]=r,m()[e/4+0]=t}function q(e,n){try{console.error(f(e,n))}finally{_.__wbindgen_export_1(e,n)}}function L(){return S((function(e,n,t){g(e).randomFillSync(x(n,t))}),arguments)}function B(){return S((function(e,n){g(e).getRandomValues(g(n))}),arguments)}function C(e){return b(g(e).process)}function D(e){const n=g(e);return"object"==typeof n&&null!==n}function R(e){return b(g(e).versions)}function $(e){return b(g(e).node)}function G(e){return"string"==typeof g(e)}function H(e){return b(g(e).crypto)}function I(e){return b(g(e).msCrypto)}function J(){return b(e)}function Z(){return S((function(e,n,t){return b(g(e).require(f(n,t)))}),arguments)}function K(e,n){return b(new Function(f(e,n)))}function N(){return S((function(e,n){return b(g(e).call(g(n)))}),arguments)}function V(e){return b(g(e))}function Y(e){return b(g(e).buffer)}function z(){return S((function(){return b(self.self)}),arguments)}function Q(){return S((function(){return b(window.window)}),arguments)}function ee(){return S((function(){return b(globalThis.globalThis)}),arguments)}function ne(){return S((function(){return b(t.g.global)}),arguments)}function te(e){return void 0===g(e)}function re(e){return b(new Uint8Array(g(e)))}function oe(e,n,t){g(e).set(g(n),t>>>0)}function _e(e){return g(e).length}function ce(e){return b(new Uint8Array(e>>>0))}function ie(e,n,t){return b(g(e).subarray(n>>>0,t>>>0))}function ae(e,n){throw new Error(f(e,n))}function ue(){return b(_.memory)}o()}catch(fe){o(fe)}}))},3881:(e,n,t)=>{t.a(e,(async(e,n)=>{try{var r=t(3118),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const n=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),t=(new r.Zu).remux(n);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),n()}catch(e){n(e)}}))},2449:(e,n,t)=>{t.a(e,(async(r,o)=>{try{var _,c=r([_=t(3118)]),[_]=c.then?(await c)():c;await t.v(n,e.id,"4468f1e6217b4e510c3e",{"./index_bg.js":{__wbindgen_string_new:_.h4,__wbindgen_object_drop_ref:_.ug,__wbg_debug_07401d6dd085ab9f:_.lL,__wbg_error_671981bc444705ed:_.Xf,__wbg_info_9e1cab954aeb98ff:_.R6,__wbg_log_3ade350d48e895ee:_.J2,__wbg_warn_c7401f641d2733e0:_.$H,__wbg_new_abda76e883ba8a5f:_.a2,__wbg_stack_658279fe44541cf6:_.KM,__wbg_error_f851667af71bcfc6:_.iX,__wbg_randomFillSync_065afffde01daa66:_.yP,__wbg_getRandomValues_b99eec4244a475bb:_.dp,__wbg_process_0cc2ada8524d6f83:_.ej,__wbindgen_is_object:_.Wl,__wbg_versions_c11acceab27a6c87:_.xn,__wbg_node_7ff1ce49caf23815:_.OP,__wbindgen_is_string:_.eY,__wbg_crypto_2036bed7c44c25e7:_.JC,__wbg_msCrypto_a21fc88caf1ecdc8:_.T_,__wbg_static_accessor_NODE_MODULE_cf6401cc1091279e:_.Mk,__wbg_require_a746e79b322b9336:_.wm,__wbg_newnoargs_e1ddb03293334932:_.jG,__wbg_call_a6fa88c3302e8ad5:_.AM,__wbindgen_object_clone_ref:_.m_,__wbg_buffer_e8e1791d59230f6e:_.MF,__wbg_self_14408afdb5c69451:_.U6,__wbg_window_75b1f6151d589837:_.fG,__wbg_globalThis_e2d2385b94c810da:_.PZ,__wbg_global_3c19477360f9b641:_.XU,__wbindgen_is_undefined:_.XP,__wbg_new_d256fd368dc8455c:_.WU,__wbg_set_ff6a229de2633e38:_.o2,__wbg_length_8c589b0fd9987662:_.mu,__wbg_newwithlength_dc0752ff6d0d8cc2:_.fd,__wbg_subarray_fbf3eb17f25d3dd4:_.Ee,__wbindgen_throw:_.Or,__wbindgen_memory:_.oH}}),o()}catch(e){o(e)}}),1)}},_={};function c(e){var n=_[e];if(void 0!==n)return n.exports;var t=_[e]={id:e,loaded:!1,exports:{}};return o[e](t,t.exports,c),t.loaded=!0,t.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&!e.d&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},c.a=(o,_,c)=>{var i;c&&((i=[]).d=1);var a,u,f,s=new Set,d=o.exports,b=new Promise(((e,n)=>{f=n,u=e}));b[n]=d,b[e]=e=>(i&&e(i),s.forEach(e),b.catch((e=>{}))),o.exports=b,_((o=>{var _;a=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var _=[];_.d=0,o.then((e=>{c[n]=e,r(_)}),(e=>{c[t]=e,r(_)}));var c={};return c[e]=e=>e(_),c}}var i={};return i[e]=e=>{},i[n]=o,i})))(o);var c=()=>a.map((e=>{if(e[t])throw e[t];return e[n]})),u=new Promise((n=>{(_=()=>n(c)).r=0;var t=e=>e!==i&&!s.has(e)&&(s.add(e),e&&!e.d&&(_.r++,e.push(_)));a.map((n=>n[e](t)))}));return _.r?u:c()}),(e=>(e?f(b[t]=e):u(d),r(i)))),i&&(i.d=0)},c.d=(e,n)=>{for(var t in n)c.o(n,t)&&!c.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},c.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),c.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),c.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),c.v=(e,n,t,r)=>{var o=fetch(c.p+""+t+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,r).then((n=>Object.assign(e,n.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)))},(()=>{var e;c.g.importScripts&&(e=c.g.location+"");var n=c.g.document;if(!e&&n&&(n.currentScript&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");t.length&&(e=t[t.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),c.p=e})(),c(3881)})();