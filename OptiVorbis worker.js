(()=>{"use strict";var e,t,n,r,o={836:(e,t,n)=>{var r=n(3044);e.exports=n.v(t,e.id,"9a09005af8508330a7e5",{"./index_bg.js":{__wbindgen_string_new:r.yc,__wbindgen_object_drop_ref:r.bk,__wbg_debug_e17b51583ca6a632:r.$U,__wbg_error_80de38b3f7cc3c3c:r.Z8,__wbg_info_033d8b8a0838f1d3:r.RK,__wbg_log_cad59bb680daec67:r.rQ,__wbg_warn_aaf1f4664a035bd6:r.ZD,__wbg_new_8a6f238a6ece86ea:r.$P,__wbg_stack_0ed75d68575b0f3c:r.x$,__wbg_error_7534b8e9a36f1ab4:r.WY,__wbg_getRandomValues_78e016fdd1d721cf:r.fs,__wbindgen_throw:r.Qn}})},3044:(e,t,n)=>{let r;function o(e){r=e}n.d(t,{$P:()=>T,$U:()=>v,Qn:()=>R,RK:()=>A,WY:()=>S,Z8:()=>E,ZD:()=>O,ao:()=>x,bk:()=>P,fs:()=>k,lI:()=>o,rQ:()=>I,x$:()=>j,yc:()=>$}),e=n.hmd(e);const a=new Array(128).fill(void 0);function i(e){return a[e]}a.push(void 0,null,!0,!1);let s=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});s.decode();let c=null;function _(){return null!==c&&0!==c.byteLength||(c=new Uint8Array(r.memory.buffer)),c}function u(e,t){return e>>>=0,s.decode(_().subarray(e,e+t))}function f(e,t){return e>>>=0,_().subarray(e/1,e/1+t)}let l=a.length;function d(e){l===a.length&&a.push(a.length+1);const t=l;return l=a[t],a[t]=e,t}let g=0,b=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const p="function"==typeof b.encodeInto?function(e,t){return b.encodeInto(e,t)}:function(e,t){const n=b.encode(e);return t.set(n),{read:e.length,written:n.length}};let w=null;function h(){return(null===w||!0===w.buffer.detached||void 0===w.buffer.detached&&w.buffer!==r.memory.buffer)&&(w=new DataView(r.memory.buffer)),w}function y(e){const t=i(e);return function(e){e<132||(a[e]=l,l=e)}(e),t}const m="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>r.__wbg_oggtoogg_free(e>>>0,1)));class x{__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,m.unregister(this),e}free(){const e=this.__destroy_into_raw();r.__wbg_oggtoogg_free(e,0)}constructor(){const e=r.oggtoogg_new_with_defaults();return this.__wbg_ptr=e>>>0,m.register(this,this.__wbg_ptr,this),this}remux(e){try{const i=r.__wbindgen_add_to_stack_pointer(-16),s=function(e,t){const n=t(1*e.length,1)>>>0;return _().set(e,n/1),g=e.length,n}(e,r.__wbindgen_export_2),c=g;r.oggtoogg_remux(i,this.__wbg_ptr,s,c);var t=h().getInt32(i+0,!0),n=h().getInt32(i+4,!0),o=h().getInt32(i+8,!0);if(h().getInt32(i+12,!0))throw y(o);var a=f(t,n).slice();return r.__wbindgen_export_0(t,1*n,1),a}finally{r.__wbindgen_add_to_stack_pointer(16)}}}function v(e,t,n,r){console.debug(i(e),i(t),i(n),i(r))}function S(e,t){let n,o;try{n=e,o=t,console.error(u(e,t))}finally{r.__wbindgen_export_0(n,o,1)}}function E(e,t,n,r){console.error(i(e),i(t),i(n),i(r))}function k(){return function(e,t){try{return function(e,t){globalThis.crypto.getRandomValues(f(e,t))}.apply(this,t)}catch(e){r.__wbindgen_export_1(d(e))}}(0,arguments)}function A(e,t,n,r){console.info(i(e),i(t),i(n),i(r))}function I(e,t,n,r){console.log(i(e),i(t),i(n),i(r))}function T(){return d(new Error)}function j(e,t){const n=function(e,t,n){if(void 0===n){const n=b.encode(e),r=t(n.length,1)>>>0;return _().subarray(r,r+n.length).set(n),g=n.length,r}let r=e.length,o=t(r,1)>>>0;const a=_();let i=0;for(;i<r;i++){const t=e.charCodeAt(i);if(t>127)break;a[o+i]=t}if(i!==r){0!==i&&(e=e.slice(i)),o=n(o,r,r=i+3*e.length,1)>>>0;const t=_().subarray(o+i,o+r);i+=p(e,t).written,o=n(o,r,i,1)>>>0}return g=i,o}(i(t).stack,r.__wbindgen_export_2,r.__wbindgen_export_3),o=g;h().setInt32(e+4,o,!0),h().setInt32(e+0,n,!0)}function O(e,t,n,r){console.warn(i(e),i(t),i(n),i(r))}function P(e){y(e)}function $(e,t){return d(u(e,t))}function R(e,t){throw new Error(u(e,t))}},4858:(e,t,n)=>{n.a(e,(async(e,t)=>{try{var r=n(8235),o=e([r]);r=(o.then?(await o)():o)[0],self.addEventListener("message",(e=>{try{const t=new Uint8Array((new FileReaderSync).readAsArrayBuffer(e.data)),n=(new r.ao).remux(t);self.postMessage(new Blob([n],{type:"audio/ogg"}))}catch(e){self.postMessage(e.toString())}})),t()}catch(e){t(e)}}))},8235:(e,t,n)=>{n.a(e,(async(e,r)=>{try{n.d(t,{ao:()=>a.ao});var o=n(836),a=n(3044),i=e([o]);o=(i.then?(await i)():i)[0],(0,a.lI)(o),o.__wbindgen_start(),r()}catch(e){r(e)}}))}},a={};function i(e){var t=a[e];if(void 0!==t)return t.exports;var n=a[e]={id:e,loaded:!1,exports:{}};return o[e](n,n.exports,i),n.loaded=!0,n.exports}e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",n="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},i.a=(o,a,i)=>{var s;i&&((s=[]).d=-1);var c,_,u,f=new Set,l=o.exports,d=new Promise(((e,t)=>{u=t,_=e}));d[t]=l,d[e]=e=>(s&&e(s),f.forEach(e),d.catch((e=>{}))),o.exports=d,a((o=>{var a;c=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var a=[];a.d=0,o.then((e=>{i[t]=e,r(a)}),(e=>{i[n]=e,r(a)}));var i={};return i[e]=e=>e(a),i}}var s={};return s[e]=e=>{},s[t]=o,s})))(o);var i=()=>c.map((e=>{if(e[n])throw e[n];return e[t]})),_=new Promise((t=>{(a=()=>t(i)).r=0;var n=e=>e!==s&&!f.has(e)&&(f.add(e),e&&!e.d&&(a.r++,e.push(a)));c.map((t=>t[e](n)))}));return a.r?_:i()}),(e=>(e?u(d[n]=e):_(l),r(s)))),s&&s.d<0&&(s.d=0)},i.d=(e,t)=>{for(var n in t)i.o(t,n)&&!i.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},i.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),i.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),i.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),i.v=(e,t,n,r)=>{var o=fetch(i.p+""+n+".module.wasm"),a=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,r))).then((t=>Object.assign(e,t.instance.exports)));return o.then((t=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(t,r).then((t=>Object.assign(e,t.instance.exports)),(e=>{if("application/wasm"!==t.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),a();throw e})):a()))},(()=>{var e;i.g.importScripts&&(e=i.g.location+"");var t=i.g.document;if(!e&&t&&(t.currentScript&&"SCRIPT"===t.currentScript.tagName.toUpperCase()&&(e=t.currentScript.src),!e)){var n=t.getElementsByTagName("script");if(n.length)for(var r=n.length-1;r>-1&&(!e||!/^http(s?):/.test(e));)e=n[r--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/^blob:/,"").replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),i.p=e})(),i(4858)})();