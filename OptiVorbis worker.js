(()=>{"use strict";var e,n,t,r,o={4858:(e,n,t)=>{t.a(e,(async(e,n)=>{try{var r=t(1007),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const n=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),t=(new r.ao).remux(n);self.postMessage(new Blob([t],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),n()}catch(e){n(e)}}))},1007:(e,n,t)=>{t.a(e,(async(e,r)=>{try{t.d(n,{ao:()=>_.ao});var o=t(836),_=t(9409),i=e([o]);o=(i.then?(await i)():i)[0],(0,_.lI)(o),o.__wbindgen_start(),r()}catch(e){r(e)}}))},9409:(e,n,t)=>{let r;function o(e){r=e}t.d(n,{$P:()=>R,BZ:()=>te,Es:()=>z,Ev:()=>C,FG:()=>I,GJ:()=>S,Gh:()=>Q,Gu:()=>X,HO:()=>G,Hk:()=>W,Hr:()=>U,Lt:()=>M,NN:()=>N,Ot:()=>$,PR:()=>Z,Py:()=>ne,Qn:()=>_e,T9:()=>H,VE:()=>O,WY:()=>A,_d:()=>Y,ao:()=>v,bk:()=>re,c_:()=>T,cp:()=>L,ek:()=>P,fT:()=>B,il:()=>E,jy:()=>q,kI:()=>V,lI:()=>o,n7:()=>F,ne:()=>j,oo:()=>J,qv:()=>K,tH:()=>k,vU:()=>ee,x$:()=>D,yc:()=>oe,zS:()=>x});const _=new Array(128).fill(void 0);function i(e){return _[e]}_.push(void 0,null,!0,!1);let c=_.length;function a(e){c===_.length&&_.push(_.length+1);const n=c;return c=_[n],_[n]=e,n}function u(e,n){try{return e.apply(this,n)}catch(e){r.__wbindgen_export_0(a(e))}}let s=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});s.decode();let f=null;function b(){return null!==f&&0!==f.byteLength||(f=new Uint8Array(r.memory.buffer)),f}function g(e,n){return e>>>=0,s.decode(b().subarray(e,e+n))}function l(e){const n=i(e);return function(e){e<132||(_[e]=c,c=e)}(e),n}let d=0,w=new("undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder)("utf-8");const p="function"==typeof w.encodeInto?function(e,n){return w.encodeInto(e,n)}:function(e,n){const t=w.encode(e);return n.set(t),{read:e.length,written:t.length}};let y=null;function h(){return(null===y||!0===y.buffer.detached||void 0===y.buffer.detached&&y.buffer!==r.memory.buffer)&&(y=new DataView(r.memory.buffer)),y}const m="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>r.__wbg_oggtoogg_free(e>>>0,1)));class v{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,m.unregister(this),e}free(){const e=this.__destroy_into_raw();r.__wbg_oggtoogg_free(e,0)}constructor(){const e=r.oggtoogg_new_with_defaults();return this.__wbg_ptr=e>>>0,m.register(this,this.__wbg_ptr,this),this}remux(e){try{const a=r.__wbindgen_add_to_stack_pointer(-16),u=function(e,n){const t=n(1*e.length,1)>>>0;return b().set(e,t/1),d=e.length,t}(e,r.__wbindgen_export_2),s=d;r.oggtoogg_remux(a,this.__wbg_ptr,u,s);var n=h().getInt32(a+0,!0),t=h().getInt32(a+4,!0),o=h().getInt32(a+8,!0);if(h().getInt32(a+12,!0))throw l(o);var _=(i=n,c=t,i>>>=0,b().subarray(i/1,i/1+c)).slice();return r.__wbindgen_export_1(n,1*t,1),_}finally{r.__wbindgen_add_to_stack_pointer(16)}var i,c}}function x(e){return a(i(e).buffer)}function S(){return u((function(e,n){return a(i(e).call(i(n)))}),arguments)}function k(){return u((function(e,n,t){return a(i(e).call(i(n),i(t)))}),arguments)}function E(e){return a(i(e).crypto)}function T(e,n,t,r){console.debug(i(e),i(n),i(t),i(r))}function A(e,n){let t,o;try{t=e,o=n,console.error(g(e,n))}finally{r.__wbindgen_export_1(t,o,1)}}function j(e,n,t,r){console.error(i(e),i(n),i(t),i(r))}function I(){return u((function(e,n){i(e).getRandomValues(i(n))}),arguments)}function P(){return u((function(){return a(globalThis.globalThis)}),arguments)}function O(){return u((function(){return a(global.global)}),arguments)}function F(e,n,t,r){console.info(i(e),i(n),i(t),i(r))}function q(e,n,t,r){console.log(i(e),i(n),i(t),i(r))}function G(e){return a(i(e).msCrypto)}function H(e){return a(new Uint8Array(i(e)))}function R(){return a(new Error)}function U(e,n){return a(new Function(g(e,n)))}function B(e,n,t){return a(new Uint8Array(i(e),n>>>0,t>>>0))}function W(e){return a(new Uint8Array(e>>>0))}function $(e){return a(i(e).node)}function C(e){return a(i(e).process)}function N(){return u((function(e,n){i(e).randomFillSync(l(n))}),arguments)}function M(){return u((function(){return a(module.require)}),arguments)}function V(){return u((function(){return a(self.self)}),arguments)}function z(e,n,t){i(e).set(i(n),t>>>0)}function D(e,n){const t=function(e,n,t){if(void 0===t){const t=w.encode(e),r=n(t.length,1)>>>0;return b().subarray(r,r+t.length).set(t),d=t.length,r}let r=e.length,o=n(r,1)>>>0;const _=b();let i=0;for(;i<r;i++){const n=e.charCodeAt(i);if(n>127)break;_[o+i]=n}if(i!==r){0!==i&&(e=e.slice(i)),o=t(o,r,r=i+3*e.length,1)>>>0;const n=b().subarray(o+i,o+r);i+=p(e,n).written,o=t(o,r,i,1)>>>0}return d=i,o}(i(n).stack,r.__wbindgen_export_2,r.__wbindgen_export_3),o=d;h().setInt32(e+4,o,!0),h().setInt32(e+0,t,!0)}function L(e,n,t){return a(i(e).subarray(n>>>0,t>>>0))}function J(e){return a(i(e).versions)}function Q(e,n,t,r){console.warn(i(e),i(n),i(t),i(r))}function Y(){return u((function(){return a(window.window)}),arguments)}function Z(e){return"function"==typeof i(e)}function K(e){const n=i(e);return"object"==typeof n&&null!==n}function X(e){return"string"==typeof i(e)}function ee(e){return void 0===i(e)}function ne(){return a(r.memory)}function te(e){return a(i(e))}function re(e){l(e)}function oe(e,n){return a(g(e,n))}function _e(e,n){throw new Error(g(e,n))}},836:(e,n,t)=>{var r=t(9409);e.exports=t.v(n,e.id,"726d9e85de7d6a6676dd",{"./index_bg.js":{__wbindgen_string_new:r.yc,__wbindgen_object_drop_ref:r.bk,__wbg_debug_f201c091a5d2019b:r.c_,__wbg_error_94252a8e90b35b8e:r.ne,__wbg_info_493696cc38ae1ad0:r.n7,__wbg_log_0051d9677940f06b:r.jy,__wbg_warn_4f29d3e20ba97cd0:r.Gh,__wbg_new_8a6f238a6ece86ea:r.$P,__wbg_stack_0ed75d68575b0f3c:r.x$,__wbg_error_7534b8e9a36f1ab4:r.WY,__wbg_crypto_ed58b8e10a292839:r.il,__wbindgen_is_object:r.qv,__wbg_process_5c1d670bc53614b8:r.Ev,__wbg_versions_c71aa1626a93e0a1:r.oo,__wbg_node_02999533c4ea02e3:r.Ot,__wbindgen_is_string:r.Gu,__wbg_require_79b1e9274cde3c87:r.Lt,__wbindgen_is_function:r.PR,__wbg_msCrypto_0a36e2ec3a343d26:r.HO,__wbg_randomFillSync_ab2cfe79ebbf2740:r.NN,__wbg_getRandomValues_bcb4912f16000dc4:r.FG,__wbg_self_cca3ca60d61220f4:r.kI,__wbg_window_2aba046d3fc4ad7c:r._d,__wbg_globalThis_6b4d52a0b6aaeaea:r.ek,__wbg_global_49324ce12193de77:r.VE,__wbindgen_is_undefined:r.vU,__wbg_newnoargs_a136448eeb7d48ac:r.Hr,__wbg_call_0ad083564791763a:r.GJ,__wbindgen_object_clone_ref:r.BZ,__wbg_call_a34b6b4765f27be0:r.tH,__wbg_buffer_ef9774282e5dab94:r.zS,__wbg_newwithbyteoffsetandlength_84908302a4c137cf:r.fT,__wbg_new_59845962d1127937:r.T9,__wbg_set_5deee49b10b2b780:r.Es,__wbg_newwithlength_4c216eaaf23f2f9a:r.Hk,__wbg_subarray_2dc34705c0dc7cdb:r.cp,__wbindgen_throw:r.Qn,__wbindgen_memory:r.Py}})}},_={};function i(e){var n=_[e];if(void 0!==n)return n.exports;var t=_[e]={id:e,exports:{}};return o[e](t,t.exports,i),t.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},i.a=(o,_,i)=>{var c;i&&((c=[]).d=-1);var a,u,s,f=new Set,b=o.exports,g=new Promise(((e,n)=>{s=n,u=e}));g[n]=b,g[e]=e=>(c&&e(c),f.forEach(e),g.catch((e=>{}))),o.exports=g,_((o=>{var _;a=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var _=[];_.d=0,o.then((e=>{i[n]=e,r(_)}),(e=>{i[t]=e,r(_)}));var i={};return i[e]=e=>e(_),i}}var c={};return c[e]=e=>{},c[n]=o,c})))(o);var i=()=>a.map((e=>{if(e[t])throw e[t];return e[n]})),u=new Promise((n=>{(_=()=>n(i)).r=0;var t=e=>e!==c&&!f.has(e)&&(f.add(e),e&&!e.d&&(_.r++,e.push(_)));a.map((n=>n[e](t)))}));return _.r?u:i()}),(e=>(e?s(g[t]=e):u(b),r(c)))),c&&c.d<0&&(c.d=0)},i.d=(e,n)=>{for(var t in n)i.o(n,t)&&!i.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:n[t]})},i.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),i.o=(e,n)=>Object.prototype.hasOwnProperty.call(e,n),i.v=(e,n,t,r)=>{var o=fetch(i.p+""+t+".module.wasm"),_=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((n=>Object.assign(e,n.instance.exports)));return o.then((n=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(n,r).then((n=>Object.assign(e,n.instance.exports)),(e=>{if("application/wasm"!==n.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),_();throw e})):_()))},(()=>{var e;i.g.importScripts&&(e=i.g.location+"");var n=i.g.document;if(!e&&n&&(n.currentScript&&"SCRIPT"===n.currentScript.tagName.toUpperCase()&&(e=n.currentScript.src),!e)){var t=n.getElementsByTagName("script");if(t.length)for(var r=t.length-1;r>-1&&(!e||!/^http(s?):/.test(e));)e=t[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),i.p=e})(),i(4858)})();