(()=>{"use strict";var e,t,n,r,o={836:(e,t,n)=>{var r=n(9409);e.exports=n.v(t,e.id,"6780accbbdd9486d653a",{"./index_bg.js":{__wbindgen_string_new:r.yc,__wbindgen_object_drop_ref:r.bk,__wbg_debug_e17b51583ca6a632:r.$U,__wbg_error_80de38b3f7cc3c3c:r.Z8,__wbg_info_033d8b8a0838f1d3:r.RK,__wbg_log_cad59bb680daec67:r.rQ,__wbg_warn_aaf1f4664a035bd6:r.ZD,__wbg_new_8a6f238a6ece86ea:r.$P,__wbg_stack_0ed75d68575b0f3c:r.x$,__wbg_error_7534b8e9a36f1ab4:r.WY,__wbg_getRandomValues_78e016fdd1d721cf:r.fs,__wbindgen_throw:r.Qn}})},1007:(e,t,n)=>{n.a(e,(async(e,r)=>{try{n.d(t,{ao:()=>i.ao});var o=n(836),i=n(9409),a=e([o]);o=(a.then?(await a)():a)[0],(0,i.lI)(o),o.__wbindgen_start(),r()}catch(e){r(e)}}))},4858:(e,t,n)=>{n.a(e,(async(e,t)=>{try{var r=n(1007),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const t=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),n=(new r.ao).remux(t);self.postMessage(new Blob([n],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),t()}catch(e){t(e)}}))},9409:(e,t,n)=>{let r;function o(e){r=e}n.d(t,{$P:()=>T,$U:()=>x,Qn:()=>O,RK:()=>E,WY:()=>S,Z8:()=>k,ZD:()=>$,ao:()=>v,bk:()=>P,fs:()=>A,lI:()=>o,rQ:()=>I,x$:()=>j,yc:()=>R});const i=new Array(128).fill(void 0);function a(e){return i[e]}i.push(void 0,null,!0,!1);let c=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});c.decode();let s=null;function _(){return null!==s&&0!==s.byteLength||(s=new Uint8Array(r.memory.buffer)),s}function u(e,t){return e>>>=0,c.decode(_().subarray(e,e+t))}function f(e,t){return e>>>=0,_().subarray(e/1,e/1+t)}let l=i.length;function g(e){l===i.length&&i.push(i.length+1);const t=l;return l=i[t],i[t]=e,t}let d=0,b=new("undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder)("utf-8");const w="function"==typeof b.encodeInto?function(e,t){return b.encodeInto(e,t)}:function(e,t){const n=b.encode(e);return t.set(n),{read:e.length,written:n.length}};let p=null;function h(){return(null===p||!0===p.buffer.detached||void 0===p.buffer.detached&&p.buffer!==r.memory.buffer)&&(p=new DataView(r.memory.buffer)),p}function y(e){const t=a(e);return function(e){e<132||(i[e]=l,l=e)}(e),t}const m="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>r.__wbg_oggtoogg_free(e>>>0,1)));class v{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,m.unregister(this),e}free(){const e=this.__destroy_into_raw();r.__wbg_oggtoogg_free(e,0)}constructor(){const e=r.oggtoogg_new_with_defaults();return this.__wbg_ptr=e>>>0,m.register(this,this.__wbg_ptr,this),this}remux(e){try{const a=r.__wbindgen_add_to_stack_pointer(-16),c=function(e,t){const n=t(1*e.length,1)>>>0;return _().set(e,n/1),d=e.length,n}(e,r.__wbindgen_export_2),s=d;r.oggtoogg_remux(a,this.__wbg_ptr,c,s);var t=h().getInt32(a+0,!0),n=h().getInt32(a+4,!0),o=h().getInt32(a+8,!0);if(h().getInt32(a+12,!0))throw y(o);var i=f(t,n).slice();return r.__wbindgen_export_0(t,1*n,1),i}finally{r.__wbindgen_add_to_stack_pointer(16)}}}function x(e,t,n,r){console.debug(a(e),a(t),a(n),a(r))}function S(e,t){let n,o;try{n=e,o=t,console.error(u(e,t))}finally{r.__wbindgen_export_0(n,o,1)}}function k(e,t,n,r){console.error(a(e),a(t),a(n),a(r))}function A(){return function(e,t){try{return function(e,t){globalThis.crypto.getRandomValues(f(e,t))}.apply(this,t)}catch(e){r.__wbindgen_export_1(g(e))}}(0,arguments)}function E(e,t,n,r){console.info(a(e),a(t),a(n),a(r))}function I(e,t,n,r){console.log(a(e),a(t),a(n),a(r))}function T(){return g(new Error)}function j(e,t){const n=function(e,t,n){if(void 0===n){const n=b.encode(e),r=t(n.length,1)>>>0;return _().subarray(r,r+n.length).set(n),d=n.length,r}let r=e.length,o=t(r,1)>>>0;const i=_();let a=0;for(;a<r;a++){const t=e.charCodeAt(a);if(t>127)break;i[o+a]=t}if(a!==r){0!==a&&(e=e.slice(a)),o=n(o,r,r=a+3*e.length,1)>>>0;const t=_().subarray(o+a,o+r);a+=w(e,t).written,o=n(o,r,a,1)>>>0}return d=a,o}(a(t).stack,r.__wbindgen_export_2,r.__wbindgen_export_3),o=d;h().setInt32(e+4,o,!0),h().setInt32(e+0,n,!0)}function $(e,t,n,r){console.warn(a(e),a(t),a(n),a(r))}function P(e){y(e)}function R(e,t){return g(u(e,t))}function O(e,t){throw new Error(u(e,t))}}},i={};function a(e){var t=i[e];if(void 0!==t)return t.exports;var n=i[e]={id:e,exports:{}};return o[e](n,n.exports,a),n.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",n="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},a.a=(o,i,a)=>{var c;a&&((c=[]).d=-1);var s,_,u,f=new Set,l=o.exports,g=new Promise(((e,t)=>{u=t,_=e}));g[t]=l,g[e]=e=>(c&&e(c),f.forEach(e),g.catch((e=>{}))),o.exports=g,i((o=>{var i;s=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var i=[];i.d=0,o.then((e=>{a[t]=e,r(i)}),(e=>{a[n]=e,r(i)}));var a={};return a[e]=e=>e(i),a}}var c={};return c[e]=e=>{},c[t]=o,c})))(o);var a=()=>s.map((e=>{if(e[n])throw e[n];return e[t]})),_=new Promise((t=>{(i=()=>t(a)).r=0;var n=e=>e!==c&&!f.has(e)&&(f.add(e),e&&!e.d&&(i.r++,e.push(i)));s.map((t=>t[e](n)))}));return i.r?_:a()}),(e=>(e?u(g[n]=e):_(l),r(c)))),c&&c.d<0&&(c.d=0)},a.d=(e,t)=>{for(var n in t)a.o(t,n)&&!a.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},a.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),a.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),a.v=(e,t,n,r)=>{var o=fetch(a.p+""+n+".module.wasm"),i=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((t=>Object.assign(e,t.instance.exports)));return o.then((t=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(t,r).then((t=>Object.assign(e,t.instance.exports)),(e=>{if("application/wasm"!==t.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),i();throw e})):i()))},(()=>{var e;a.g.importScripts&&(e=a.g.location+"");var t=a.g.document;if(!e&&t&&(t.currentScript&&"SCRIPT"===t.currentScript.tagName.toUpperCase()&&(e=t.currentScript.src),!e)){var n=t.getElementsByTagName("script");if(n.length)for(var r=n.length-1;r>-1&&(!e||!/^http(s?):/.test(e));)e=n[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/^blob:/,"").replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),a.p=e})(),a(4858)})();