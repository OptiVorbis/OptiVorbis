(()=>{"use strict";var n,e,t,r,o={800:(n,e,t)=>{t.a(n,(async(n,r)=>{try{t.d(e,{Zu:()=>_.Zu});var o=t(2449),_=t(3118),c=n([_,o]);[_,o]=c.then?(await c)():c,o.__wbindgen_start(),r()}catch(n){r(n)}}))},3118:(n,e,t)=>{t.a(n,(async(r,o)=>{try{t.d(e,{$L:()=>nn,Ev:()=>P,KM:()=>V,MH:()=>F,NW:()=>O,Ni:()=>C,Or:()=>un,Q2:()=>M,S6:()=>B,Sn:()=>G,T2:()=>D,TY:()=>rn,VD:()=>K,VJ:()=>q,V_:()=>k,Wl:()=>H,XP:()=>tn,ZP:()=>I,ZX:()=>U,Zu:()=>A,a2:()=>W,eY:()=>L,fP:()=>_n,gW:()=>Y,h4:()=>E,iX:()=>Z,ib:()=>cn,jp:()=>Q,kH:()=>an,k_:()=>N,lB:()=>on,m_:()=>R,o$:()=>J,oH:()=>fn,tn:()=>X,ug:()=>T,wJ:()=>en,wy:()=>$,yB:()=>z});var _=t(2449);n=t.hmd(n);var c=r([_]);_=(c.then?(await c)():c)[0];let i=new("undefined"==typeof TextDecoder?(0,n.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});i.decode();let a=new Uint8Array;function u(){return 0===a.byteLength&&(a=new Uint8Array(_.memory.buffer)),a}function f(n,e){return i.decode(u().subarray(n,n+e))}const s=new Array(32).fill(void 0);s.push(void 0,null,!0,!1);let b=s.length;function d(n){b===s.length&&s.push(s.length+1);const e=b;return b=s[e],s[e]=n,e}function w(n){return s[n]}function g(n){n<36||(s[n]=b,b=n)}function l(n){const e=w(n);return g(n),e}let p=0;function y(n,e){const t=e(1*n.length);return u().set(n,t/1),p=n.length,t}let h=new Int32Array;function m(){return 0===h.byteLength&&(h=new Int32Array(_.memory.buffer)),h}let v=new("undefined"==typeof TextEncoder?(0,n.require)("util").TextEncoder:TextEncoder)("utf-8");const x="function"==typeof v.encodeInto?function(n,e){return v.encodeInto(n,e)}:function(n,e){const t=v.encode(n);return e.set(t),{read:n.length,written:t.length}};function S(n,e,t){if(void 0===t){const t=v.encode(n),r=e(t.length);return u().subarray(r,r+t.length).set(t),p=t.length,r}let r=n.length,o=e(r);const _=u();let c=0;for(;c<r;c++){const e=n.charCodeAt(c);if(e>127)break;_[o+c]=e}if(c!==r){0!==c&&(n=n.slice(c)),o=t(o,r,r=c+3*n.length);const e=u().subarray(o+c,o+r);c+=x(n,e).written}return p=c,o}function j(n,e){try{return n.apply(this,e)}catch(n){_.__wbindgen_export_3(d(n))}}class A{static __wrap(n){const e=Object.create(A.prototype);return e.ptr=n,e}__destroy_into_raw(){const n=this.ptr;return this.ptr=0,n}free(){const n=this.__destroy_into_raw();_.__wbg_oggtoogg_free(n)}constructor(){const n=_.oggtoogg_new_with_defaults();return A.__wrap(n)}remux(n){try{const a=_.__wbindgen_add_to_stack_pointer(-16),f=y(n,_.__wbindgen_export_0),s=p;_.oggtoogg_remux(a,this.ptr,f,s);var e=m()[a/4+0],t=m()[a/4+1],r=m()[a/4+2];if(m()[a/4+3])throw l(r);var o=(c=e,i=t,u().subarray(c/1,c/1+i)).slice();return _.__wbindgen_export_1(e,1*t),o}finally{_.__wbindgen_add_to_stack_pointer(16)}var c,i}}function E(n,e){return d(f(n,e))}function T(n){l(n)}function k(n,e,t,r){console.debug(w(n),w(e),w(t),w(r))}function P(n,e,t,r){console.error(w(n),w(e),w(t),w(r))}function O(n,e,t,r){console.info(w(n),w(e),w(t),w(r))}function B(n,e,t,r){console.log(w(n),w(e),w(t),w(r))}function M(n,e,t,r){console.warn(w(n),w(e),w(t),w(r))}function W(){return d(new Error)}function V(n,e){const t=S(w(e).stack,_.__wbindgen_export_0,_.__wbindgen_export_2),r=p;m()[n/4+1]=r,m()[n/4+0]=t}function Z(n,e){try{console.error(f(n,e))}finally{_.__wbindgen_export_1(n,e)}}function U(){return j((function(n,e){w(n).getRandomValues(w(e))}),arguments)}function $(){return j((function(n,e){w(n).randomFillSync(l(e))}),arguments)}function q(n){return d(w(n).crypto)}function H(n){const e=w(n);return"object"==typeof e&&null!==e}function X(n){return d(w(n).process)}function D(n){return d(w(n).versions)}function F(n){return d(w(n).node)}function L(n){return"string"==typeof w(n)}function N(n){return d(w(n).msCrypto)}function I(){return j((function(){return d(n.require)}),arguments)}function J(n){return"function"==typeof w(n)}function Y(n,e){return d(new Function(f(n,e)))}function C(){return j((function(n,e){return d(w(n).call(w(e)))}),arguments)}function R(n){return d(w(n))}function K(){return j((function(n,e,t){return d(w(n).call(w(e),w(t)))}),arguments)}function Q(n){return d(w(n).buffer)}function z(){return j((function(){return d(self.self)}),arguments)}function G(){return j((function(){return d(window.window)}),arguments)}function nn(){return j((function(){return d(globalThis.globalThis)}),arguments)}function en(){return j((function(){return d(t.g.global)}),arguments)}function tn(n){return void 0===w(n)}function rn(n,e,t){return d(new Uint8Array(w(n),e>>>0,t>>>0))}function on(n){return d(new Uint8Array(w(n)))}function _n(n,e,t){w(n).set(w(e),t>>>0)}function cn(n){return d(new Uint8Array(n>>>0))}function an(n,e,t){return d(w(n).subarray(e>>>0,t>>>0))}function un(n,e){throw new Error(f(n,e))}function fn(){return d(_.memory)}o()}catch(sn){o(sn)}}))},3881:(n,e,t)=>{t.a(n,(async(n,e)=>{try{var r=t(800),o=n([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(n=>{try{const e=new Uint8Array((new FileReaderSync).readAsArrayBuffer(n.data)),t=(new r.Zu).remux(e);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(n){self.postMessage(n.toString())}})),e()}catch(n){e(n)}}))},2449:(n,e,t)=>{t.a(n,(async(r,o)=>{try{var _,c=r([_=t(3118)]),[_]=c.then?(await c)():c;await t.v(e,n.id,"9d83e03b986ab32c3c4f",{"./index_bg.js":{__wbindgen_string_new:_.h4,__wbindgen_object_drop_ref:_.ug,__wbg_debug_64711eb2fc6980ef:_.V_,__wbg_error_00c5d571f754f629:_.Ev,__wbg_info_d60a960a4e955dc2:_.NW,__wbg_log_de258f66ad9eb784:_.S6,__wbg_warn_be542501a57387a5:_.Q2,__wbg_new_abda76e883ba8a5f:_.a2,__wbg_stack_658279fe44541cf6:_.KM,__wbg_error_f851667af71bcfc6:_.iX,__wbg_getRandomValues_3774744e221a22ad:_.ZX,__wbg_randomFillSync_e950366c42764a07:_.wy,__wbg_crypto_70a96de3b6b73dac:_.VJ,__wbindgen_is_object:_.Wl,__wbg_process_dd1577445152112e:_.tn,__wbg_versions_58036bec3add9e6f:_.T2,__wbg_node_6a9d28205ed5b0d8:_.MH,__wbindgen_is_string:_.eY,__wbg_msCrypto_adbc770ec9eca9c7:_.k_,__wbg_require_f05d779769764e82:_.ZP,__wbindgen_is_function:_.o$,__wbg_newnoargs_b5b063fc6c2f0376:_.gW,__wbg_call_97ae9d8645dc388b:_.Ni,__wbindgen_object_clone_ref:_.m_,__wbg_call_168da88779e35f61:_.VD,__wbg_buffer_3f3d764d4747d564:_.jp,__wbg_self_6d479506f72c6a71:_.yB,__wbg_window_f2557cc78490aceb:_.Sn,__wbg_globalThis_7f206bda628d5286:_.$L,__wbg_global_ba75c50d1cf384f4:_.wJ,__wbindgen_is_undefined:_.XP,__wbg_newwithbyteoffsetandlength_d9aa266703cb98be:_.TY,__wbg_new_8c3f0052272a457a:_.lB,__wbg_set_83db9690f9353e79:_.fP,__wbg_newwithlength_f5933855e4f48a19:_.ib,__wbg_subarray_58ad4efbb5bcb886:_.kH,__wbindgen_throw:_.Or,__wbindgen_memory:_.oH}}),o()}catch(n){o(n)}}),1)}},_={};function c(n){var e=_[n];if(void 0!==e)return e.exports;var t=_[n]={id:n,loaded:!1,exports:{}};return o[n](t,t.exports,c),t.loaded=!0,t.exports}n="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",e="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=n=>{n&&!n.d&&(n.d=1,n.forEach((n=>n.r--)),n.forEach((n=>n.r--?n.r++:n())))},c.a=(o,_,c)=>{var i;c&&((i=[]).d=1);var a,u,f,s=new Set,b=o.exports,d=new Promise(((n,e)=>{f=e,u=n}));d[e]=b,d[n]=n=>(i&&n(i),s.forEach(n),d.catch((n=>{}))),o.exports=d,_((o=>{var _;a=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[n])return o;if(o.then){var _=[];_.d=0,o.then((n=>{c[e]=n,r(_)}),(n=>{c[t]=n,r(_)}));var c={};return c[n]=n=>n(_),c}}var i={};return i[n]=n=>{},i[e]=o,i})))(o);var c=()=>a.map((n=>{if(n[t])throw n[t];return n[e]})),u=new Promise((e=>{(_=()=>e(c)).r=0;var t=n=>n!==i&&!s.has(n)&&(s.add(n),n&&!n.d&&(_.r++,n.push(_)));a.map((e=>e[n](t)))}));return _.r?u:c()}),(n=>(n?f(d[t]=n):u(b),r(i)))),i&&(i.d=0)},c.d=(n,e)=>{for(var t in e)c.o(e,t)&&!c.o(n,t)&&Object.defineProperty(n,t,{enumerable:!0,get:e[t]})},c.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(n){if("object"==typeof window)return window}}(),c.hmd=n=>((n=Object.create(n)).children||(n.children=[]),Object.defineProperty(n,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+n.id)}}),n),c.o=(n,e)=>Object.prototype.hasOwnProperty.call(n,e),c.v=(n,e,t,r)=>{var o=fetch(c.p+""+t+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,r).then((e=>Object.assign(n,e.instance.exports))):o.then((n=>n.arrayBuffer())).then((n=>WebAssembly.instantiate(n,r))).then((e=>Object.assign(n,e.instance.exports)))},(()=>{var n;c.g.importScripts&&(n=c.g.location+"");var e=c.g.document;if(!n&&e&&(e.currentScript&&(n=e.currentScript.src),!n)){var t=e.getElementsByTagName("script");t.length&&(n=t[t.length-1].src)}if(!n)throw new Error("Automatic publicPath is not supported in this browser");n=n.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),c.p=n})(),c(3881)})();