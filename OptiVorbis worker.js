(()=>{"use strict";var e,n,t,r,o={3118:(e,n,t)=>{t.a(e,(async(r,o)=>{try{t.d(n,{$r:()=>G,CU:()=>$,Ce:()=>F,DX:()=>J,Ih:()=>M,Mz:()=>q,Nu:()=>ce,Or:()=>ue,QZ:()=>X,Rh:()=>oe,SH:()=>D,VU:()=>Q,Vz:()=>H,Wl:()=>R,XP:()=>te,Z5:()=>L,Zu:()=>E,_d:()=>z,aB:()=>ne,cA:()=>C,dx:()=>ie,eY:()=>W,en:()=>ee,gk:()=>V,h4:()=>k,hF:()=>N,j:()=>T,m_:()=>Y,oH:()=>ae,q5:()=>U,uV:()=>_e,ug:()=>O,wF:()=>B,xR:()=>K,xe:()=>re,y8:()=>I,yq:()=>P,z1:()=>Z});var _=t(2449);e=t.hmd(e);var c=r([_]);_=(c.then?(await c)():c)[0];let i,u=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});function a(){return 0===i.byteLength&&(i=new Uint8Array(_.memory.buffer)),i}function s(e,n){return u.decode(a().subarray(e,e+n))}u.decode();const f=new Array(32).fill(void 0);f.push(void 0,null,!0,!1);let d=f.length;function b(e){d===f.length&&f.push(f.length+1);const n=d;return d=f[n],f[n]=e,n}function g(e){return f[e]}function w(e){e<36||(f[e]=d,d=e)}function l(e){const n=g(e);return w(e),n}let p,y=0;function h(e,n){const t=n(1*e.length);return a().set(e,t/1),y=e.length,t}function m(){return 0===p.byteLength&&(p=new Int32Array(_.memory.buffer)),p}function x(e,n){return a().subarray(e/1,e/1+n)}let v=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const S="function"==typeof v.encodeInto?function(e,n){return v.encodeInto(e,n)}:function(e,n){const t=v.encode(e);return n.set(t),{read:e.length,written:t.length}};function j(e,n,t){if(void 0===t){const t=v.encode(e),r=n(t.length);return a().subarray(r,r+t.length).set(t),y=t.length,r}let r=e.length,o=n(r);const _=a();let c=0;for(;c<r;c++){const n=e.charCodeAt(c);if(n>127)break;_[o+c]=n}if(c!==r){0!==c&&(e=e.slice(c)),o=t(o,r,r=c+3*e.length);const n=a().subarray(o+c,o+r);c+=S(e,n).written}return y=c,o}function A(e,n){try{return e.apply(this,n)}catch(e){_.__wbindgen_export_3(b(e))}}class E{static __wrap(e){const n=Object.create(E.prototype);return n.ptr=e,n}__destroy_into_raw(){const e=this.ptr;return this.ptr=0,e}free(){const e=this.__destroy_into_raw();_.__wbg_oggtoogg_free(e)}constructor(){const e=_.oggtoogg_new_with_defaults();return E.__wrap(e)}remux(e){try{const c=_.__wbindgen_add_to_stack_pointer(-16),i=h(e,_.__wbindgen_export_0),u=y;_.oggtoogg_remux(c,this.ptr,i,u);var n=m()[c/4+0],t=m()[c/4+1],r=m()[c/4+2];if(m()[c/4+3])throw l(r);var o=x(n,t).slice();return _.__wbindgen_export_1(n,1*t),o}finally{_.__wbindgen_add_to_stack_pointer(16)}}}function k(e,n){return b(s(e,n))}function O(e){l(e)}function T(e,n,t,r){console.debug(g(e),g(n),g(t),g(r))}function U(e,n,t,r){console.error(g(e),g(n),g(t),g(r))}function q(e,n,t,r){console.info(g(e),g(n),g(t),g(r))}function F(e,n,t,r){console.log(g(e),g(n),g(t),g(r))}function I(e,n,t,r){console.warn(g(e),g(n),g(t),g(r))}function M(){return b(new Error)}function P(e,n){const t=j(g(n).stack,_.__wbindgen_export_0,_.__wbindgen_export_2),r=y;m()[e/4+1]=r,m()[e/4+0]=t}function V(e,n){try{console.error(s(e,n))}finally{_.__wbindgen_export_1(e,n)}}function B(){return A((function(e,n,t){g(e).randomFillSync(x(n,t))}),arguments)}function C(){return A((function(e,n){g(e).getRandomValues(g(n))}),arguments)}function D(e){return b(g(e).process)}function R(e){const n=g(e);return"object"==typeof n&&null!==n}function z(e){return b(g(e).versions)}function Z(e){return b(g(e).node)}function W(e){return"string"==typeof g(e)}function $(e){return b(g(e).crypto)}function H(e){return b(g(e).msCrypto)}function L(){return b(e)}function N(){return A((function(e,n,t){return b(g(e).require(s(n,t)))}),arguments)}function X(e,n){return b(new Function(s(e,n)))}function Q(){return A((function(e,n){return b(g(e).call(g(n)))}),arguments)}function Y(e){return b(g(e))}function G(e){return b(g(e).buffer)}function J(){return A((function(){return b(self.self)}),arguments)}function K(){return A((function(){return b(window.window)}),arguments)}function ee(){return A((function(){return b(globalThis.globalThis)}),arguments)}function ne(){return A((function(){return b(t.g.global)}),arguments)}function te(e){return void 0===g(e)}function re(e){return b(new Uint8Array(g(e)))}function oe(e,n,t){g(e).set(g(n),t>>>0)}function _e(e){return g(e).length}function ce(e){return b(new Uint8Array(e>>>0))}function ie(e,n,t){return b(g(e).subarray(n>>>0,t>>>0))}function ue(e,n){throw new Error(s(e,n))}function ae(){return b(_.memory)}p=new Int32Array(_.memory.buffer),i=new Uint8Array(_.memory.buffer),o()}catch(se){o(se)}}))},3881:(e,n,t)=>{t.a(e,(async(e,n)=>{try{var r=t(3118),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const n=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),t=(new r.Zu).remux(n);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),n()}catch(e){n(e)}}))},2449:(e,n,t)=>{t.a(e,(async(r,o)=>{try{var _,c=r([_=t(3118)]),[_]=c.then?(await c)():c;await t.v(n,e.id,"96826cd815732e96e961",{"./index_bg.js":{__wbindgen_string_new:_.h4,__wbindgen_object_drop_ref:_.ug,__wbg_debug_8445d9e6649f346e:_.j,__wbg_error_c67ca41b33779f06:_.q5,__wbg_info_89e9ec243332464d:_.Mz,__wbg_log_4a85132d4d89d41e:_.Ce,__wbg_warn_022e4f61a9c7964f:_.y8,__wbg_new_693216e109162396:_.Ih,__wbg_stack_0ddaca5d1abfb52f:_.yq,__wbg_error_09919627ac0992f5:_.gk,__wbg_randomFillSync_91e2b39becca6147:_.wF,__wbg_getRandomValues_b14734aa289bc356:_.cA,__wbg_process_e56fd54cf6319b6c:_.SH,__wbindgen_is_object:_.Wl,__wbg_versions_77e21455908dad33:_._d,__wbg_node_0dd25d832e4785d5:_.z1,__wbindgen_is_string:_.eY,__wbg_crypto_b95d7173266618a9:_.CU,__wbg_msCrypto_5a86d77a66230f81:_.Vz,__wbg_static_accessor_NODE_MODULE_26b231378c1be7dd:_.Z5,__wbg_require_0db1598d9ccecb30:_.hF,__wbg_newnoargs_fc5356289219b93b:_.QZ,__wbg_call_4573f605ca4b5f10:_.VU,__wbindgen_object_clone_ref:_.m_,__wbg_buffer_de1150f91b23aa89:_.$r,__wbg_self_ba1ddafe9ea7a3a2:_.DX,__wbg_window_be3cc430364fd32c:_.xR,__wbg_globalThis_56d9c9f814daeeee:_.en,__wbg_global_8c35aeee4ac77f2b:_.aB,__wbindgen_is_undefined:_.XP,__wbg_new_97cf52648830a70d:_.xe,__wbg_set_a0172b213e2469e9:_.Rh,__wbg_length_e09c0b925ab8de5d:_.uV,__wbg_newwithlength_e833b89f9db02732:_.Nu,__wbg_subarray_9482ae5cd5cd99d3:_.dx,__wbindgen_throw:_.Or,__wbindgen_memory:_.oH}}),o()}catch(e){o(e)}}),1)}},_={};function c(e){var n=_[e];if(void 0!==n)return n.exports;var t=_[e]={id:e,loaded:!1,exports:{}};return o[e](t,t.exports,c),t.loaded=!0,t.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&!e.d&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},c.a=(o,_,c)=>{var i;c&&((i=[]).d=1),i&&(i.moduleId=o.id);var u,a,s,f=new Set,d=o.exports,b=new Promise(((e,n)=>{s=n,a=e}));b[n]=d,b[e]=e=>(i&&e(i),f.forEach(e),b.catch((e=>{}))),b.moduleId=o.id,o.exports=b,_((o=>{var _;u=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var _=[];_.d=0,o.then((e=>{c[n]=e,r(_)}),(e=>{c[t]=e,r(_)}));var c={};return c[e]=e=>e(_),c}}var i={};return i[e]=e=>{},i[n]=o,i})))(o);var c=()=>u.map((e=>{if(e[t])throw e[t];return e[n]})),a=new Promise((n=>{(_=()=>n(c)).r=0;var t=e=>e!==i&&!f.has(e)&&(f.add(e),e&&!e.d&&(_.r++,e.push(_)));u.map((n=>n[e](t)))}));return _.r?a:c()}),(e=>(e?s(b[t]=e):a(d),r(i)))),i&&(i.d=0)},c.d=(e,n)=>{for(var t in n)c.o(n,t)&&!c.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},c.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),c.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),c.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),c.v=(e,n,t,r)=>{var o=fetch(c.p+""+t+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,r).then((n=>Object.assign(e,n.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)))},(()=>{var e;c.g.importScripts&&(e=c.g.location+"");var n=c.g.document;if(!e&&n&&(n.currentScript&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");t.length&&(e=t[t.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),c.p=e})(),c(3881)})();