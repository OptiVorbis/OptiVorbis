(()=>{"use strict";var e,n,t,r,o={3118:(e,n,t)=>{t.a(e,(async(r,o)=>{try{t.d(n,{At:()=>X,Bm:()=>U,CU:()=>N,FS:()=>oe,G0:()=>re,Hc:()=>Q,Ih:()=>P,Jm:()=>$,Km:()=>ie,Or:()=>ae,SH:()=>C,UL:()=>q,Vz:()=>V,Wl:()=>D,XP:()=>te,Yw:()=>_e,Z5:()=>Y,Zu:()=>j,_J:()=>ce,_d:()=>W,_f:()=>R,cA:()=>M,eA:()=>K,eY:()=>J,fh:()=>H,gk:()=>I,h4:()=>k,hF:()=>Z,lH:()=>ee,m_:()=>G,oH:()=>ue,oc:()=>F,qN:()=>ne,ug:()=>O,vv:()=>T,wF:()=>L,yq:()=>B,z1:()=>z});var _=t(2449);e=t.hmd(e);var c=r([_]);_=(c.then?(await c)():c)[0];let i=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});i.decode();let a=new Uint8Array;function u(){return 0===a.byteLength&&(a=new Uint8Array(_.memory.buffer)),a}function f(e,n){return i.decode(u().subarray(e,e+n))}const s=new Array(32).fill(void 0);s.push(void 0,null,!0,!1);let d=s.length;function b(e){d===s.length&&s.push(s.length+1);const n=d;return d=s[n],s[n]=e,n}function g(e){return s[e]}function w(e){e<36||(s[e]=d,d=e)}function l(e){const n=g(e);return w(e),n}let p=0;function h(e,n){const t=n(1*e.length);return u().set(e,t/1),p=e.length,t}let y=new Int32Array;function m(){return 0===y.byteLength&&(y=new Int32Array(_.memory.buffer)),y}function v(e,n){return u().subarray(e/1,e/1+n)}let x=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const S="function"==typeof x.encodeInto?function(e,n){return x.encodeInto(e,n)}:function(e,n){const t=x.encode(e);return n.set(t),{read:e.length,written:t.length}};function A(e,n,t){if(void 0===t){const t=x.encode(e),r=n(t.length);return u().subarray(r,r+t.length).set(t),p=t.length,r}let r=e.length,o=n(r);const _=u();let c=0;for(;c<r;c++){const n=e.charCodeAt(c);if(n>127)break;_[o+c]=n}if(c!==r){0!==c&&(e=e.slice(c)),o=t(o,r,r=c+3*e.length);const n=u().subarray(o+c,o+r);c+=S(e,n).written}return p=c,o}function E(e,n){try{return e.apply(this,n)}catch(e){_.__wbindgen_export_3(b(e))}}class j{static __wrap(e){const n=Object.create(j.prototype);return n.ptr=e,n}__destroy_into_raw(){const e=this.ptr;return this.ptr=0,e}free(){const e=this.__destroy_into_raw();_.__wbg_oggtoogg_free(e)}constructor(){const e=_.oggtoogg_new_with_defaults();return j.__wrap(e)}remux(e){try{const c=_.__wbindgen_add_to_stack_pointer(-16),i=h(e,_.__wbindgen_export_0),a=p;_.oggtoogg_remux(c,this.ptr,i,a);var n=m()[c/4+0],t=m()[c/4+1],r=m()[c/4+2];if(m()[c/4+3])throw l(r);var o=v(n,t).slice();return _.__wbindgen_export_1(n,1*t),o}finally{_.__wbindgen_add_to_stack_pointer(16)}}}function k(e,n){return b(f(e,n))}function O(e){l(e)}function T(e,n,t,r){console.debug(g(e),g(n),g(t),g(r))}function F(e,n,t,r){console.error(g(e),g(n),g(t),g(r))}function U(e,n,t,r){console.info(g(e),g(n),g(t),g(r))}function q(e,n,t,r){console.log(g(e),g(n),g(t),g(r))}function H(e,n,t,r){console.warn(g(e),g(n),g(t),g(r))}function P(){return b(new Error)}function B(e,n){const t=A(g(n).stack,_.__wbindgen_export_0,_.__wbindgen_export_2),r=p;m()[e/4+1]=r,m()[e/4+0]=t}function I(e,n){try{console.error(f(e,n))}finally{_.__wbindgen_export_1(e,n)}}function L(){return E((function(e,n,t){g(e).randomFillSync(v(n,t))}),arguments)}function M(){return E((function(e,n){g(e).getRandomValues(g(n))}),arguments)}function C(e){return b(g(e).process)}function D(e){const n=g(e);return"object"==typeof n&&null!==n}function W(e){return b(g(e).versions)}function z(e){return b(g(e).node)}function J(e){return"string"==typeof g(e)}function N(e){return b(g(e).crypto)}function V(e){return b(g(e).msCrypto)}function Y(){return b(e)}function Z(){return E((function(e,n,t){return b(g(e).require(f(n,t)))}),arguments)}function R(e,n){return b(new Function(f(e,n)))}function $(){return E((function(e,n){return b(g(e).call(g(n)))}),arguments)}function G(e){return b(g(e))}function K(e){return b(g(e).buffer)}function X(){return E((function(){return b(self.self)}),arguments)}function Q(){return E((function(){return b(window.window)}),arguments)}function ee(){return E((function(){return b(globalThis.globalThis)}),arguments)}function ne(){return E((function(){return b(t.g.global)}),arguments)}function te(e){return void 0===g(e)}function re(e){return b(new Uint8Array(g(e)))}function oe(e,n,t){g(e).set(g(n),t>>>0)}function _e(e){return g(e).length}function ce(e){return b(new Uint8Array(e>>>0))}function ie(e,n,t){return b(g(e).subarray(n>>>0,t>>>0))}function ae(e,n){throw new Error(f(e,n))}function ue(){return b(_.memory)}o()}catch(fe){o(fe)}}))},3881:(e,n,t)=>{t.a(e,(async(e,n)=>{try{var r=t(3118),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const n=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),t=(new r.Zu).remux(n);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),n()}catch(e){n(e)}}))},2449:(e,n,t)=>{t.a(e,(async(r,o)=>{try{var _,c=r([_=t(3118)]),[_]=c.then?(await c)():c;await t.v(n,e.id,"5423d1329799f1edba0b",{"./index_bg.js":{__wbindgen_string_new:_.h4,__wbindgen_object_drop_ref:_.ug,__wbg_debug_1dccd22b8a8988e1:_.vv,__wbg_error_d539c0f5eafe6a31:_.oc,__wbg_info_17d18b9f8eaab7d9:_.Bm,__wbg_log_f286f3fe4aad906d:_.UL,__wbg_warn_3d6689f77cb29c86:_.fh,__wbg_new_693216e109162396:_.Ih,__wbg_stack_0ddaca5d1abfb52f:_.yq,__wbg_error_09919627ac0992f5:_.gk,__wbg_randomFillSync_91e2b39becca6147:_.wF,__wbg_getRandomValues_b14734aa289bc356:_.cA,__wbg_process_e56fd54cf6319b6c:_.SH,__wbindgen_is_object:_.Wl,__wbg_versions_77e21455908dad33:_._d,__wbg_node_0dd25d832e4785d5:_.z1,__wbindgen_is_string:_.eY,__wbg_crypto_b95d7173266618a9:_.CU,__wbg_msCrypto_5a86d77a66230f81:_.Vz,__wbg_static_accessor_NODE_MODULE_26b231378c1be7dd:_.Z5,__wbg_require_0db1598d9ccecb30:_.hF,__wbg_newnoargs_971e9a5abe185139:_._f,__wbg_call_33d7bcddbbfa394a:_.Jm,__wbindgen_object_clone_ref:_.m_,__wbg_buffer_34f5ec9f8a838ba0:_.eA,__wbg_self_fd00a1ef86d1b2ed:_.At,__wbg_window_6f6e346d8bbd61d7:_.Hc,__wbg_globalThis_3348936ac49df00a:_.lH,__wbg_global_67175caf56f55ca9:_.qN,__wbindgen_is_undefined:_.XP,__wbg_new_cda198d9dbc6d7ea:_.G0,__wbg_set_1a930cfcda1a8067:_.FS,__wbg_length_51f19f73d6d9eff3:_.Yw,__wbg_newwithlength_66e5530e7079ea1b:_._J,__wbg_subarray_270ff8dd5582c1ac:_.Km,__wbindgen_throw:_.Or,__wbindgen_memory:_.oH}}),o()}catch(e){o(e)}}),1)}},_={};function c(e){var n=_[e];if(void 0!==n)return n.exports;var t=_[e]={id:e,loaded:!1,exports:{}};return o[e](t,t.exports,c),t.loaded=!0,t.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&!e.d&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},c.a=(o,_,c)=>{var i;c&&((i=[]).d=1);var a,u,f,s=new Set,d=o.exports,b=new Promise(((e,n)=>{f=n,u=e}));b[n]=d,b[e]=e=>(i&&e(i),s.forEach(e),b.catch((e=>{}))),o.exports=b,_((o=>{var _;a=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var _=[];_.d=0,o.then((e=>{c[n]=e,r(_)}),(e=>{c[t]=e,r(_)}));var c={};return c[e]=e=>e(_),c}}var i={};return i[e]=e=>{},i[n]=o,i})))(o);var c=()=>a.map((e=>{if(e[t])throw e[t];return e[n]})),u=new Promise((n=>{(_=()=>n(c)).r=0;var t=e=>e!==i&&!s.has(e)&&(s.add(e),e&&!e.d&&(_.r++,e.push(_)));a.map((n=>n[e](t)))}));return _.r?u:c()}),(e=>(e?f(b[t]=e):u(d),r(i)))),i&&(i.d=0)},c.d=(e,n)=>{for(var t in n)c.o(n,t)&&!c.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},c.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),c.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),c.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),c.v=(e,n,t,r)=>{var o=fetch(c.p+""+t+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,r).then((n=>Object.assign(e,n.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)))},(()=>{var e;c.g.importScripts&&(e=c.g.location+"");var n=c.g.document;if(!e&&n&&(n.currentScript&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");t.length&&(e=t[t.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),c.p=e})(),c(3881)})();