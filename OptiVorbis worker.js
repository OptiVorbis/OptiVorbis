(()=>{"use strict";var e,n,t,r,o={4858:(e,n,t)=>{t.a(e,(async(e,n)=>{try{var r=t(1007),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const n=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),t=(new r.ao).remux(n);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),n()}catch(e){n(e)}}))},1007:(e,n,t)=>{t.a(e,(async(e,r)=>{try{t.d(n,{ao:()=>i.ao});var o=t(836),i=t(9409),_=e([o]);o=(_.then?(await _)():_)[0],(0,i.lI)(o),o.__wbindgen_start(),r()}catch(e){r(e)}}))},9409:(e,n,t)=>{let r;function o(e){r=e}t.d(n,{$P:()=>q,BZ:()=>te,Ev:()=>D,FG:()=>E,Gu:()=>X,HO:()=>M,It:()=>k,Lt:()=>Y,M_:()=>j,NN:()=>N,On:()=>W,Ot:()=>$,PC:()=>C,PR:()=>J,Py:()=>ne,Qn:()=>ie,RM:()=>H,TY:()=>S,WV:()=>T,WY:()=>A,Zv:()=>G,ao:()=>v,bk:()=>re,hs:()=>U,il:()=>P,kR:()=>V,l1:()=>R,lI:()=>o,oP:()=>Q,oo:()=>z,pD:()=>F,qv:()=>K,ro:()=>O,s6:()=>Z,sj:()=>x,tY:()=>B,vU:()=>ee,wg:()=>I,x$:()=>L,yc:()=>oe});const i=new Array(128).fill(void 0);function _(e){return i[e]}i.push(void 0,null,!0,!1);let c=i.length;function a(e){c===i.length&&i.push(i.length+1);const n=c;return c=i[n],i[n]=e,n}function u(e,n){try{return e.apply(this,n)}catch(e){r.__wbindgen_export_0(a(e))}}let s=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});s.decode();let f=null;function b(){return null!==f&&0!==f.byteLength||(f=new Uint8Array(r.memory.buffer)),f}function g(e,n){return e>>>=0,s.decode(b().subarray(e,e+n))}function l(e){const n=_(e);return function(e){e<132||(i[e]=c,c=e)}(e),n}let w=0,d=new("undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder)("utf-8");const p="function"==typeof d.encodeInto?function(e,n){return d.encodeInto(e,n)}:function(e,n){const t=d.encode(e);return n.set(t),{read:e.length,written:t.length}};let y=null;function h(){return(null===y||!0===y.buffer.detached||void 0===y.buffer.detached&&y.buffer!==r.memory.buffer)&&(y=new DataView(r.memory.buffer)),y}const m="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>r.__wbg_oggtoogg_free(e>>>0,1)));class v{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,m.unregister(this),e}free(){const e=this.__destroy_into_raw();r.__wbg_oggtoogg_free(e,0)}constructor(){const e=r.oggtoogg_new_with_defaults();return this.__wbg_ptr=e>>>0,m.register(this,this.__wbg_ptr,this),this}remux(e){try{const a=r.__wbindgen_add_to_stack_pointer(-16),u=function(e,n){const t=n(1*e.length,1)>>>0;return b().set(e,t/1),w=e.length,t}(e,r.__wbindgen_export_2),s=w;r.oggtoogg_remux(a,this.__wbg_ptr,u,s);var n=h().getInt32(a+0,!0),t=h().getInt32(a+4,!0),o=h().getInt32(a+8,!0);if(h().getInt32(a+12,!0))throw l(o);var i=(_=n,c=t,_>>>=0,b().subarray(_/1,_/1+c)).slice();return r.__wbindgen_export_1(n,1*t,1),i}finally{r.__wbindgen_add_to_stack_pointer(16)}var _,c}}function x(e){return a(_(e).buffer)}function S(){return u((function(e,n,t){return a(_(e).call(_(n),_(t)))}),arguments)}function k(){return u((function(e,n){return a(_(e).call(_(n)))}),arguments)}function P(e){return a(_(e).crypto)}function T(e,n,t,r){console.debug(_(e),_(n),_(t),_(r))}function A(e,n){let t,o;try{t=e,o=n,console.error(g(e,n))}finally{r.__wbindgen_export_1(t,o,1)}}function j(e,n,t,r){console.error(_(e),_(n),_(t),_(r))}function E(){return u((function(e,n){_(e).getRandomValues(_(n))}),arguments)}function I(){return u((function(){return a(globalThis.globalThis)}),arguments)}function O(){return u((function(){return a(global.global)}),arguments)}function R(e,n,t,r){console.info(_(e),_(n),_(t),_(r))}function F(e,n,t,r){console.log(_(e),_(n),_(t),_(r))}function M(e){return a(_(e).msCrypto)}function W(e){return a(new Uint8Array(_(e)))}function q(){return a(new Error)}function C(e,n){return a(new Function(g(e,n)))}function U(e,n,t){return a(new Uint8Array(_(e),n>>>0,t>>>0))}function B(e){return a(new Uint8Array(e>>>0))}function $(e){return a(_(e).node)}function D(e){return a(_(e).process)}function N(){return u((function(e,n){_(e).randomFillSync(l(n))}),arguments)}function Y(){return u((function(){return a(module.require)}),arguments)}function V(){return u((function(){return a(self.self)}),arguments)}function G(e,n,t){_(e).set(_(n),t>>>0)}function L(e,n){const t=function(e,n,t){if(void 0===t){const t=d.encode(e),r=n(t.length,1)>>>0;return b().subarray(r,r+t.length).set(t),w=t.length,r}let r=e.length,o=n(r,1)>>>0;const i=b();let _=0;for(;_<r;_++){const n=e.charCodeAt(_);if(n>127)break;i[o+_]=n}if(_!==r){0!==_&&(e=e.slice(_)),o=t(o,r,r=_+3*e.length,1)>>>0;const n=b().subarray(o+_,o+r);_+=p(e,n).written,o=t(o,r,_,1)>>>0}return w=_,o}(_(n).stack,r.__wbindgen_export_2,r.__wbindgen_export_3),o=w;h().setInt32(e+4,o,!0),h().setInt32(e+0,t,!0)}function Z(e,n,t){return a(_(e).subarray(n>>>0,t>>>0))}function z(e){return a(_(e).versions)}function H(e,n,t,r){console.warn(_(e),_(n),_(t),_(r))}function Q(){return u((function(){return a(window.window)}),arguments)}function J(e){return"function"==typeof _(e)}function K(e){const n=_(e);return"object"==typeof n&&null!==n}function X(e){return"string"==typeof _(e)}function ee(e){return void 0===_(e)}function ne(){return a(r.memory)}function te(e){return a(_(e))}function re(e){l(e)}function oe(e,n){return a(g(e,n))}function ie(e,n){throw new Error(g(e,n))}},836:(e,n,t)=>{var r=t(9409);e.exports=t.v(n,e.id,"cc3cfbea218dbcec7f60",{"./index_bg.js":{__wbindgen_string_new:r.yc,__wbindgen_object_drop_ref:r.bk,__wbg_debug_27dd03b8945e39d7:r.WV,__wbg_error_818ac809371bfd77:r.M_,__wbg_info_ab0093df83f380f1:r.l1,__wbg_log_b4ad6746bfb50e80:r.pD,__wbg_warn_43ba5f40fc329fe9:r.RM,__wbg_new_8a6f238a6ece86ea:r.$P,__wbg_stack_0ed75d68575b0f3c:r.x$,__wbg_error_7534b8e9a36f1ab4:r.WY,__wbg_crypto_ed58b8e10a292839:r.il,__wbindgen_is_object:r.qv,__wbg_process_5c1d670bc53614b8:r.Ev,__wbg_versions_c71aa1626a93e0a1:r.oo,__wbg_node_02999533c4ea02e3:r.Ot,__wbindgen_is_string:r.Gu,__wbg_require_79b1e9274cde3c87:r.Lt,__wbindgen_is_function:r.PR,__wbg_msCrypto_0a36e2ec3a343d26:r.HO,__wbg_randomFillSync_ab2cfe79ebbf2740:r.NN,__wbg_getRandomValues_bcb4912f16000dc4:r.FG,__wbg_self_ac4343e4047b83cc:r.kR,__wbg_window_1a23defd102c72f4:r.oP,__wbg_globalThis_1e2ac1d6eee845b3:r.wg,__wbg_global_f25a574ae080367c:r.ro,__wbindgen_is_undefined:r.vU,__wbg_newnoargs_19a249f4eceaaac3:r.PC,__wbg_call_3114932863209ca6:r.It,__wbindgen_object_clone_ref:r.BZ,__wbg_call_0411c0c3c424db9a:r.TY,__wbg_buffer_6e1d53ff183194fc:r.sj,__wbg_newwithbyteoffsetandlength_ee8def7000b7b2be:r.hs,__wbg_new_23362fa370a0a372:r.On,__wbg_set_7b70226104a82921:r.Zv,__wbg_newwithlength_91de49dea5643c87:r.tY,__wbg_subarray_b4e9772c34a7f5ba:r.s6,__wbindgen_throw:r.Qn,__wbindgen_memory:r.Py}})}},i={};function _(e){var n=i[e];if(void 0!==n)return n.exports;var t=i[e]={id:e,exports:{}};return o[e](t,t.exports,_),t.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},_.a=(o,i,_)=>{var c;_&&((c=[]).d=-1);var a,u,s,f=new Set,b=o.exports,g=new Promise(((e,n)=>{s=n,u=e}));g[n]=b,g[e]=e=>(c&&e(c),f.forEach(e),g.catch((e=>{}))),o.exports=g,i((o=>{var i;a=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var i=[];i.d=0,o.then((e=>{_[n]=e,r(i)}),(e=>{_[t]=e,r(i)}));var _={};return _[e]=e=>e(i),_}}var c={};return c[e]=e=>{},c[n]=o,c})))(o);var _=()=>a.map((e=>{if(e[t])throw e[t];return e[n]})),u=new Promise((n=>{(i=()=>n(_)).r=0;var t=e=>e!==c&&!f.has(e)&&(f.add(e),e&&!e.d&&(i.r++,e.push(i)));a.map((n=>n[e](t)))}));return i.r?u:_()}),(e=>(e?s(g[t]=e):u(b),r(c)))),c&&c.d<0&&(c.d=0)},_.d=(e,n)=>{for(var t in n)_.o(n,t)&&!_.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},_.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),_.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),_.v=(e,n,t,r)=>{var o=fetch(_.p+""+t+".module.wasm"),i=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)));return o.then((n=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(n,r).then((n=>Object.assign(e,n.instance.exports)),(e=>{if("application/wasm"!==n.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),i();throw e})):i()))},(()=>{var e;_.g.importScripts&&(e=_.g.location+"");var n=_.g.document;if(!e&&n&&(n.currentScript&&"SCRIPT"===n.currentScript.tagName.toUpperCase()&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");if(t.length)for(var r=t.length-1;r>-1&&(!e||!/^http(s?):/.test(e));)e=t[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),_.p=e})(),_(4858)})();