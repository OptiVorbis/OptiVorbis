(()=>{"use strict";var n,e,t,r,o={800:(n,e,t)=>{t.a(n,(async(n,r)=>{try{t.d(e,{Zu:()=>_.Zu});var o=t(2449),_=t(3118),c=n([_,o]);[_,o]=c.then?(await c)():c,o.__wbindgen_start(),r()}catch(n){r(n)}}))},3118:(n,e,t)=>{t.a(n,(async(r,o)=>{try{t.d(e,{$L:()=>en,DI:()=>F,Ev:()=>P,KM:()=>N,NW:()=>B,Ni:()=>J,Nk:()=>U,Or:()=>fn,Q2:()=>D,QT:()=>L,S6:()=>W,Sn:()=>nn,VD:()=>Y,V_:()=>O,Wl:()=>V,XP:()=>rn,Zu:()=>k,a2:()=>M,bj:()=>cn,cr:()=>X,e:()=>I,eY:()=>Q,ex:()=>q,f5:()=>H,fP:()=>_n,gW:()=>R,go:()=>Z,h4:()=>A,iX:()=>$,ib:()=>an,jp:()=>z,kH:()=>un,lB:()=>on,m_:()=>K,o$:()=>C,oH:()=>sn,ug:()=>T,wJ:()=>tn,yB:()=>G});var _=t(2449);n=t.hmd(n);var c=r([_]);_=(c.then?(await c)():c)[0];let i=new("undefined"==typeof TextDecoder?(0,n.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});i.decode();let a=new Uint8Array;function u(){return 0===a.byteLength&&(a=new Uint8Array(_.memory.buffer)),a}function f(n,e){return i.decode(u().subarray(n,n+e))}const s=new Array(32).fill(void 0);s.push(void 0,null,!0,!1);let b=s.length;function d(n){b===s.length&&s.push(s.length+1);const e=b;return b=s[e],s[e]=n,e}function g(n){return s[n]}function l(n){n<36||(s[n]=b,b=n)}function w(n){const e=g(n);return l(n),e}let p=0;function y(n,e){const t=e(1*n.length);return u().set(n,t/1),p=n.length,t}let h=new Int32Array;function m(){return 0===h.byteLength&&(h=new Int32Array(_.memory.buffer)),h}function v(n,e){return u().subarray(n/1,n/1+e)}let x=new("undefined"==typeof TextEncoder?(0,n.require)("util").TextEncoder:TextEncoder)("utf-8");const S="function"==typeof x.encodeInto?function(n,e){return x.encodeInto(n,e)}:function(n,e){const t=x.encode(n);return e.set(t),{read:n.length,written:t.length}};function j(n,e,t){if(void 0===t){const t=x.encode(n),r=e(t.length);return u().subarray(r,r+t.length).set(t),p=t.length,r}let r=n.length,o=e(r);const _=u();let c=0;for(;c<r;c++){const e=n.charCodeAt(c);if(e>127)break;_[o+c]=e}if(c!==r){0!==c&&(n=n.slice(c)),o=t(o,r,r=c+3*n.length);const e=u().subarray(o+c,o+r);c+=S(n,e).written}return p=c,o}function E(n,e){try{return n.apply(this,e)}catch(n){_.__wbindgen_export_3(d(n))}}class k{static __wrap(n){const e=Object.create(k.prototype);return e.ptr=n,e}__destroy_into_raw(){const n=this.ptr;return this.ptr=0,n}free(){const n=this.__destroy_into_raw();_.__wbg_oggtoogg_free(n)}constructor(){const n=_.oggtoogg_new_with_defaults();return k.__wrap(n)}remux(n){try{const c=_.__wbindgen_add_to_stack_pointer(-16),i=y(n,_.__wbindgen_export_0),a=p;_.oggtoogg_remux(c,this.ptr,i,a);var e=m()[c/4+0],t=m()[c/4+1],r=m()[c/4+2];if(m()[c/4+3])throw w(r);var o=v(e,t).slice();return _.__wbindgen_export_1(e,1*t),o}finally{_.__wbindgen_add_to_stack_pointer(16)}}}function A(n,e){return d(f(n,e))}function T(n){w(n)}function O(n,e,t,r){console.debug(g(n),g(e),g(t),g(r))}function P(n,e,t,r){console.error(g(n),g(e),g(t),g(r))}function B(n,e,t,r){console.info(g(n),g(e),g(t),g(r))}function W(n,e,t,r){console.log(g(n),g(e),g(t),g(r))}function D(n,e,t,r){console.warn(g(n),g(e),g(t),g(r))}function M(){return d(new Error)}function N(n,e){const t=j(g(e).stack,_.__wbindgen_export_0,_.__wbindgen_export_2),r=p;m()[n/4+1]=r,m()[n/4+0]=t}function $(n,e){try{console.error(f(n,e))}finally{_.__wbindgen_export_1(n,e)}}function q(){return E((function(n,e,t){g(n).randomFillSync(v(e,t))}),arguments)}function I(){return E((function(n,e){g(n).getRandomValues(g(e))}),arguments)}function U(n){return d(g(n).crypto)}function V(n){const e=g(n);return"object"==typeof e&&null!==e}function F(n){return d(g(n).process)}function L(n){return d(g(n).versions)}function H(n){return d(g(n).node)}function Q(n){return"string"==typeof g(n)}function X(n){return d(g(n).msCrypto)}function Z(){return E((function(){return d(n.require)}),arguments)}function C(n){return"function"==typeof g(n)}function R(n,e){return d(new Function(f(n,e)))}function J(){return E((function(n,e){return d(g(n).call(g(e)))}),arguments)}function K(n){return d(g(n))}function Y(){return E((function(n,e,t){return d(g(n).call(g(e),g(t)))}),arguments)}function z(n){return d(g(n).buffer)}function G(){return E((function(){return d(self.self)}),arguments)}function nn(){return E((function(){return d(window.window)}),arguments)}function en(){return E((function(){return d(globalThis.globalThis)}),arguments)}function tn(){return E((function(){return d(t.g.global)}),arguments)}function rn(n){return void 0===g(n)}function on(n){return d(new Uint8Array(g(n)))}function _n(n,e,t){g(n).set(g(e),t>>>0)}function cn(n){return g(n).length}function an(n){return d(new Uint8Array(n>>>0))}function un(n,e,t){return d(g(n).subarray(e>>>0,t>>>0))}function fn(n,e){throw new Error(f(n,e))}function sn(){return d(_.memory)}o()}catch(bn){o(bn)}}))},3881:(n,e,t)=>{t.a(n,(async(n,e)=>{try{var r=t(800),o=n([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(n=>{try{const e=new Uint8Array((new FileReaderSync).readAsArrayBuffer(n.data)),t=(new r.Zu).remux(e);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(n){self.postMessage(n.toString())}})),e()}catch(n){e(n)}}))},2449:(n,e,t)=>{t.a(n,(async(r,o)=>{try{var _,c=r([_=t(3118)]),[_]=c.then?(await c)():c;await t.v(e,n.id,"415784c0a8731f3f9905",{"./index_bg.js":{__wbindgen_string_new:_.h4,__wbindgen_object_drop_ref:_.ug,__wbg_debug_64711eb2fc6980ef:_.V_,__wbg_error_00c5d571f754f629:_.Ev,__wbg_info_d60a960a4e955dc2:_.NW,__wbg_log_de258f66ad9eb784:_.S6,__wbg_warn_be542501a57387a5:_.Q2,__wbg_new_abda76e883ba8a5f:_.a2,__wbg_stack_658279fe44541cf6:_.KM,__wbg_error_f851667af71bcfc6:_.iX,__wbg_randomFillSync_6894564c2c334c42:_.ex,__wbg_getRandomValues_805f1c3d65988a5a:_.e,__wbg_crypto_e1d53a1d73fb10b8:_.Nk,__wbindgen_is_object:_.Wl,__wbg_process_038c26bf42b093f8:_.DI,__wbg_versions_ab37218d2f0b24a8:_.QT,__wbg_node_080f4b19d15bc1fe:_.f5,__wbindgen_is_string:_.eY,__wbg_msCrypto_6e7d3e1f92610cbb:_.cr,__wbg_require_78a3dcfbdba9cbce:_.go,__wbindgen_is_function:_.o$,__wbg_newnoargs_b5b063fc6c2f0376:_.gW,__wbg_call_97ae9d8645dc388b:_.Ni,__wbindgen_object_clone_ref:_.m_,__wbg_call_168da88779e35f61:_.VD,__wbg_buffer_3f3d764d4747d564:_.jp,__wbg_self_6d479506f72c6a71:_.yB,__wbg_window_f2557cc78490aceb:_.Sn,__wbg_globalThis_7f206bda628d5286:_.$L,__wbg_global_ba75c50d1cf384f4:_.wJ,__wbindgen_is_undefined:_.XP,__wbg_new_8c3f0052272a457a:_.lB,__wbg_set_83db9690f9353e79:_.fP,__wbg_length_9e1ae1900cb0fbd5:_.bj,__wbg_newwithlength_f5933855e4f48a19:_.ib,__wbg_subarray_58ad4efbb5bcb886:_.kH,__wbindgen_throw:_.Or,__wbindgen_memory:_.oH}}),o()}catch(n){o(n)}}),1)}},_={};function c(n){var e=_[n];if(void 0!==e)return e.exports;var t=_[n]={id:n,loaded:!1,exports:{}};return o[n](t,t.exports,c),t.loaded=!0,t.exports}n="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",e="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=n=>{n&&!n.d&&(n.d=1,n.forEach((n=>n.r--)),n.forEach((n=>n.r--?n.r++:n())))},c.a=(o,_,c)=>{var i;c&&((i=[]).d=1);var a,u,f,s=new Set,b=o.exports,d=new Promise(((n,e)=>{f=e,u=n}));d[e]=b,d[n]=n=>(i&&n(i),s.forEach(n),d.catch((n=>{}))),o.exports=d,_((o=>{var _;a=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[n])return o;if(o.then){var _=[];_.d=0,o.then((n=>{c[e]=n,r(_)}),(n=>{c[t]=n,r(_)}));var c={};return c[n]=n=>n(_),c}}var i={};return i[n]=n=>{},i[e]=o,i})))(o);var c=()=>a.map((n=>{if(n[t])throw n[t];return n[e]})),u=new Promise((e=>{(_=()=>e(c)).r=0;var t=n=>n!==i&&!s.has(n)&&(s.add(n),n&&!n.d&&(_.r++,n.push(_)));a.map((e=>e[n](t)))}));return _.r?u:c()}),(n=>(n?f(d[t]=n):u(b),r(i)))),i&&(i.d=0)},c.d=(n,e)=>{for(var t in e)c.o(e,t)&&!c.o(n,t)&&Object.defineProperty(n,t,{enumerable:!0,get:e[t]})},c.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(n){if("object"==typeof window)return window}}(),c.hmd=n=>((n=Object.create(n)).children||(n.children=[]),Object.defineProperty(n,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+n.id)}}),n),c.o=(n,e)=>Object.prototype.hasOwnProperty.call(n,e),c.v=(n,e,t,r)=>{var o=fetch(c.p+""+t+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,r).then((e=>Object.assign(n,e.instance.exports))):o.then((n=>n.arrayBuffer())).then((n=>WebAssembly.instantiate(n,r))).then((e=>Object.assign(n,e.instance.exports)))},(()=>{var n;c.g.importScripts&&(n=c.g.location+"");var e=c.g.document;if(!n&&e&&(e.currentScript&&(n=e.currentScript.src),!n)){var t=e.getElementsByTagName("script");t.length&&(n=t[t.length-1].src)}if(!n)throw new Error("Automatic publicPath is not supported in this browser");n=n.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),c.p=n})(),c(3881)})();