(()=>{"use strict";var t={3803:(t,e,r)=>{r(5398),t.exports=r(9037)},509:(t,e,r)=>{var n=r(9985),o=r(3691),i=TypeError;t.exports=function(t){if(n(t))return t;throw new i(o(t)+" is not a function")}},5027:(t,e,r)=>{var n=r(8999),o=String,i=TypeError;t.exports=function(t){if(n(t))return t;throw new i(o(t)+" is not an object")}},4328:(t,e,r)=>{var n=r(5290),o=r(7578),i=r(6310),a=function(t){return function(e,r,a){var c,u=n(e),s=i(u),l=o(a,s);if(t&&r!=r){for(;s>l;)if((c=u[l++])!=c)return!0}else for(;s>l;l++)if((t||l in u)&&u[l]===r)return t||l||0;return!t&&-1}};t.exports={includes:a(!0),indexOf:a(!1)}},6648:(t,e,r)=>{var n=r(8844),o=n({}.toString),i=n("".slice);t.exports=function(t){return i(o(t),8,-1)}},8758:(t,e,r)=>{var n=r(6812),o=r(9152),i=r(2474),a=r(2560);t.exports=function(t,e,r){for(var c=o(e),u=a.f,s=i.f,l=0;l<c.length;l++){var f=c[l];n(t,f)||r&&n(r,f)||u(t,f,s(e,f))}}},5773:(t,e,r)=>{var n=r(7697),o=r(2560),i=r(5684);t.exports=n?function(t,e,r){return o.f(t,e,i(1,r))}:function(t,e,r){return t[e]=r,t}},5684:t=>{t.exports=function(t,e){return{enumerable:!(1&t),configurable:!(2&t),writable:!(4&t),value:e}}},1880:(t,e,r)=>{var n=r(9985),o=r(2560),i=r(8702),a=r(5014);t.exports=function(t,e,r,c){c||(c={});var u=c.enumerable,s=void 0!==c.name?c.name:e;if(n(r)&&i(r,s,c),c.global)u?t[e]=r:a(e,r);else{try{c.unsafe?t[e]&&(u=!0):delete t[e]}catch(t){}u?t[e]=r:o.f(t,e,{value:r,enumerable:!1,configurable:!c.nonConfigurable,writable:!c.nonWritable})}return t}},5014:(t,e,r)=>{var n=r(9037),o=Object.defineProperty;t.exports=function(t,e){try{o(n,t,{value:e,configurable:!0,writable:!0})}catch(r){n[t]=e}return e}},7697:(t,e,r)=>{var n=r(3689);t.exports=!n((function(){return 7!==Object.defineProperty({},1,{get:function(){return 7}})[1]}))},2659:t=>{var e="object"==typeof document&&document.all,r=void 0===e&&void 0!==e;t.exports={all:e,IS_HTMLDDA:r}},6420:(t,e,r)=>{var n=r(9037),o=r(8999),i=n.document,a=o(i)&&o(i.createElement);t.exports=function(t){return a?i.createElement(t):{}}},71:t=>{t.exports="undefined"!=typeof navigator&&String(navigator.userAgent)||""},3615:(t,e,r)=>{var n,o,i=r(9037),a=r(71),c=i.process,u=i.Deno,s=c&&c.versions||u&&u.version,l=s&&s.v8;l&&(o=(n=l.split("."))[0]>0&&n[0]<4?1:+(n[0]+n[1])),!o&&a&&(!(n=a.match(/Edge\/(\d+)/))||n[1]>=74)&&(n=a.match(/Chrome\/(\d+)/))&&(o=+n[1]),t.exports=o},2739:t=>{t.exports=["constructor","hasOwnProperty","isPrototypeOf","propertyIsEnumerable","toLocaleString","toString","valueOf"]},9989:(t,e,r)=>{var n=r(9037),o=r(2474).f,i=r(5773),a=r(1880),c=r(5014),u=r(8758),s=r(5266);t.exports=function(t,e){var r,l,f,p,d,v=t.target,m=t.global,g=t.stat;if(r=m?n:g?n[v]||c(v,{}):(n[v]||{}).prototype)for(l in e){if(p=e[l],f=t.dontCallGetSet?(d=o(r,l))&&d.value:r[l],!s(m?l:v+(g?".":"#")+l,t.forced)&&void 0!==f){if(typeof p==typeof f)continue;u(p,f)}(t.sham||f&&f.sham)&&i(p,"sham",!0),a(r,l,p,t)}}},3689:t=>{t.exports=function(t){try{return!!t()}catch(t){return!0}}},7215:(t,e,r)=>{var n=r(3689);t.exports=!n((function(){var t=function(){}.bind();return"function"!=typeof t||t.hasOwnProperty("prototype")}))},2615:(t,e,r)=>{var n=r(7215),o=Function.prototype.call;t.exports=n?o.bind(o):function(){return o.apply(o,arguments)}},1236:(t,e,r)=>{var n=r(7697),o=r(6812),i=Function.prototype,a=n&&Object.getOwnPropertyDescriptor,c=o(i,"name"),u=c&&"something"===function(){}.name,s=c&&(!n||n&&a(i,"name").configurable);t.exports={EXISTS:c,PROPER:u,CONFIGURABLE:s}},8844:(t,e,r)=>{var n=r(7215),o=Function.prototype,i=o.call,a=n&&o.bind.bind(i,i);t.exports=n?a:function(t){return function(){return i.apply(t,arguments)}}},6058:(t,e,r)=>{var n=r(9037),o=r(9985);t.exports=function(t,e){return arguments.length<2?(r=n[t],o(r)?r:void 0):n[t]&&n[t][e];var r}},4849:(t,e,r)=>{var n=r(509),o=r(981);t.exports=function(t,e){var r=t[e];return o(r)?void 0:n(r)}},9037:function(t,e,r){var n=function(t){return t&&t.Math===Math&&t};t.exports=n("object"==typeof globalThis&&globalThis)||n("object"==typeof window&&window)||n("object"==typeof self&&self)||n("object"==typeof r.g&&r.g)||n("object"==typeof this&&this)||function(){return this}()||Function("return this")()},6812:(t,e,r)=>{var n=r(8844),o=r(690),i=n({}.hasOwnProperty);t.exports=Object.hasOwn||function(t,e){return i(o(t),e)}},7248:t=>{t.exports={}},8506:(t,e,r)=>{var n=r(7697),o=r(3689),i=r(6420);t.exports=!n&&!o((function(){return 7!==Object.defineProperty(i("div"),"a",{get:function(){return 7}}).a}))},4413:(t,e,r)=>{var n=r(8844),o=r(3689),i=r(6648),a=Object,c=n("".split);t.exports=o((function(){return!a("z").propertyIsEnumerable(0)}))?function(t){return"String"===i(t)?c(t,""):a(t)}:a},6738:(t,e,r)=>{var n=r(8844),o=r(9985),i=r(4091),a=n(Function.toString);o(i.inspectSource)||(i.inspectSource=function(t){return a(t)}),t.exports=i.inspectSource},618:(t,e,r)=>{var n,o,i,a=r(9834),c=r(9037),u=r(8999),s=r(5773),l=r(6812),f=r(4091),p=r(2713),d=r(7248),v="Object already initialized",m=c.TypeError,g=c.WeakMap;if(a||f.state){var y=f.state||(f.state=new g);y.get=y.get,y.has=y.has,y.set=y.set,n=function(t,e){if(y.has(t))throw new m(v);return e.facade=t,y.set(t,e),e},o=function(t){return y.get(t)||{}},i=function(t){return y.has(t)}}else{var b=p("state");d[b]=!0,n=function(t,e){if(l(t,b))throw new m(v);return e.facade=t,s(t,b,e),e},o=function(t){return l(t,b)?t[b]:{}},i=function(t){return l(t,b)}}t.exports={set:n,get:o,has:i,enforce:function(t){return i(t)?o(t):n(t,{})},getterFor:function(t){return function(e){var r;if(!u(e)||(r=o(e)).type!==t)throw new m("Incompatible receiver, "+t+" required");return r}}}},9985:(t,e,r)=>{var n=r(2659),o=n.all;t.exports=n.IS_HTMLDDA?function(t){return"function"==typeof t||t===o}:function(t){return"function"==typeof t}},5266:(t,e,r)=>{var n=r(3689),o=r(9985),i=/#|\.prototype\./,a=function(t,e){var r=u[c(t)];return r===l||r!==s&&(o(e)?n(e):!!e)},c=a.normalize=function(t){return String(t).replace(i,".").toLowerCase()},u=a.data={},s=a.NATIVE="N",l=a.POLYFILL="P";t.exports=a},981:t=>{t.exports=function(t){return null==t}},8999:(t,e,r)=>{var n=r(9985),o=r(2659),i=o.all;t.exports=o.IS_HTMLDDA?function(t){return"object"==typeof t?null!==t:n(t)||t===i}:function(t){return"object"==typeof t?null!==t:n(t)}},3931:t=>{t.exports=!1},734:(t,e,r)=>{var n=r(6058),o=r(9985),i=r(3622),a=r(9525),c=Object;t.exports=a?function(t){return"symbol"==typeof t}:function(t){var e=n("Symbol");return o(e)&&i(e.prototype,c(t))}},6310:(t,e,r)=>{var n=r(3126);t.exports=function(t){return n(t.length)}},8702:(t,e,r)=>{var n=r(8844),o=r(3689),i=r(9985),a=r(6812),c=r(7697),u=r(1236).CONFIGURABLE,s=r(6738),l=r(618),f=l.enforce,p=l.get,d=String,v=Object.defineProperty,m=n("".slice),g=n("".replace),y=n([].join),b=c&&!o((function(){return 8!==v((function(){}),"length",{value:8}).length})),h=String(String).split("String"),x=t.exports=function(t,e,r){"Symbol("===m(d(e),0,7)&&(e="["+g(d(e),/^Symbol\(([^)]*)\)/,"$1")+"]"),r&&r.getter&&(e="get "+e),r&&r.setter&&(e="set "+e),(!a(t,"name")||u&&t.name!==e)&&(c?v(t,"name",{value:e,configurable:!0}):t.name=e),b&&r&&a(r,"arity")&&t.length!==r.arity&&v(t,"length",{value:r.arity});try{r&&a(r,"constructor")&&r.constructor?c&&v(t,"prototype",{writable:!1}):t.prototype&&(t.prototype=void 0)}catch(t){}var n=f(t);return a(n,"source")||(n.source=y(h,"string"==typeof e?e:"")),t};Function.prototype.toString=x((function(){return i(this)&&p(this).source||s(this)}),"toString")},8828:t=>{var e=Math.ceil,r=Math.floor;t.exports=Math.trunc||function(t){var n=+t;return(n>0?r:e)(n)}},2560:(t,e,r)=>{var n=r(7697),o=r(8506),i=r(5648),a=r(5027),c=r(8360),u=TypeError,s=Object.defineProperty,l=Object.getOwnPropertyDescriptor,f="enumerable",p="configurable",d="writable";e.f=n?i?function(t,e,r){if(a(t),e=c(e),a(r),"function"==typeof t&&"prototype"===e&&"value"in r&&d in r&&!r[d]){var n=l(t,e);n&&n[d]&&(t[e]=r.value,r={configurable:p in r?r[p]:n[p],enumerable:f in r?r[f]:n[f],writable:!1})}return s(t,e,r)}:s:function(t,e,r){if(a(t),e=c(e),a(r),o)try{return s(t,e,r)}catch(t){}if("get"in r||"set"in r)throw new u("Accessors not supported");return"value"in r&&(t[e]=r.value),t}},2474:(t,e,r)=>{var n=r(7697),o=r(2615),i=r(9556),a=r(5684),c=r(5290),u=r(8360),s=r(6812),l=r(8506),f=Object.getOwnPropertyDescriptor;e.f=n?f:function(t,e){if(t=c(t),e=u(e),l)try{return f(t,e)}catch(t){}if(s(t,e))return a(!o(i.f,t,e),t[e])}},2741:(t,e,r)=>{var n=r(4948),o=r(2739).concat("length","prototype");e.f=Object.getOwnPropertyNames||function(t){return n(t,o)}},7518:(t,e)=>{e.f=Object.getOwnPropertySymbols},3622:(t,e,r)=>{var n=r(8844);t.exports=n({}.isPrototypeOf)},4948:(t,e,r)=>{var n=r(8844),o=r(6812),i=r(5290),a=r(4328).indexOf,c=r(7248),u=n([].push);t.exports=function(t,e){var r,n=i(t),s=0,l=[];for(r in n)!o(c,r)&&o(n,r)&&u(l,r);for(;e.length>s;)o(n,r=e[s++])&&(~a(l,r)||u(l,r));return l}},9556:(t,e)=>{var r={}.propertyIsEnumerable,n=Object.getOwnPropertyDescriptor,o=n&&!r.call({1:2},1);e.f=o?function(t){var e=n(this,t);return!!e&&e.enumerable}:r},5899:(t,e,r)=>{var n=r(2615),o=r(9985),i=r(8999),a=TypeError;t.exports=function(t,e){var r,c;if("string"===e&&o(r=t.toString)&&!i(c=n(r,t)))return c;if(o(r=t.valueOf)&&!i(c=n(r,t)))return c;if("string"!==e&&o(r=t.toString)&&!i(c=n(r,t)))return c;throw new a("Can't convert object to primitive value")}},9152:(t,e,r)=>{var n=r(6058),o=r(8844),i=r(2741),a=r(7518),c=r(5027),u=o([].concat);t.exports=n("Reflect","ownKeys")||function(t){var e=i.f(c(t)),r=a.f;return r?u(e,r(t)):e}},4684:(t,e,r)=>{var n=r(981),o=TypeError;t.exports=function(t){if(n(t))throw new o("Can't call method on "+t);return t}},2713:(t,e,r)=>{var n=r(3430),o=r(4630),i=n("keys");t.exports=function(t){return i[t]||(i[t]=o(t))}},4091:(t,e,r)=>{var n=r(9037),o=r(5014),i="__core-js_shared__",a=n[i]||o(i,{});t.exports=a},3430:(t,e,r)=>{var n=r(3931),o=r(4091);(t.exports=function(t,e){return o[t]||(o[t]=void 0!==e?e:{})})("versions",[]).push({version:"3.33.3",mode:n?"pure":"global",copyright:"© 2014-2023 Denis Pushkarev (zloirock.ru)",license:"https://github.com/zloirock/core-js/blob/v3.33.3/LICENSE",source:"https://github.com/zloirock/core-js"})},146:(t,e,r)=>{var n=r(3615),o=r(3689),i=r(9037).String;t.exports=!!Object.getOwnPropertySymbols&&!o((function(){var t=Symbol("symbol detection");return!i(t)||!(Object(t)instanceof Symbol)||!Symbol.sham&&n&&n<41}))},7578:(t,e,r)=>{var n=r(8700),o=Math.max,i=Math.min;t.exports=function(t,e){var r=n(t);return r<0?o(r+e,0):i(r,e)}},5290:(t,e,r)=>{var n=r(4413),o=r(4684);t.exports=function(t){return n(o(t))}},8700:(t,e,r)=>{var n=r(8828);t.exports=function(t){var e=+t;return e!=e||0===e?0:n(e)}},3126:(t,e,r)=>{var n=r(8700),o=Math.min;t.exports=function(t){return t>0?o(n(t),9007199254740991):0}},690:(t,e,r)=>{var n=r(4684),o=Object;t.exports=function(t){return o(n(t))}},8732:(t,e,r)=>{var n=r(2615),o=r(8999),i=r(734),a=r(4849),c=r(5899),u=r(4201),s=TypeError,l=u("toPrimitive");t.exports=function(t,e){if(!o(t)||i(t))return t;var r,u=a(t,l);if(u){if(void 0===e&&(e="default"),r=n(u,t,e),!o(r)||i(r))return r;throw new s("Can't convert object to primitive value")}return void 0===e&&(e="number"),c(t,e)}},8360:(t,e,r)=>{var n=r(8732),o=r(734);t.exports=function(t){var e=n(t,"string");return o(e)?e:e+""}},3691:t=>{var e=String;t.exports=function(t){try{return e(t)}catch(t){return"Object"}}},4630:(t,e,r)=>{var n=r(8844),o=0,i=Math.random(),a=n(1..toString);t.exports=function(t){return"Symbol("+(void 0===t?"":t)+")_"+a(++o+i,36)}},9525:(t,e,r)=>{var n=r(146);t.exports=n&&!Symbol.sham&&"symbol"==typeof Symbol.iterator},5648:(t,e,r)=>{var n=r(7697),o=r(3689);t.exports=n&&o((function(){return 42!==Object.defineProperty((function(){}),"prototype",{value:42,writable:!1}).prototype}))},9834:(t,e,r)=>{var n=r(9037),o=r(9985),i=n.WeakMap;t.exports=o(i)&&/native code/.test(String(i))},4201:(t,e,r)=>{var n=r(9037),o=r(3430),i=r(6812),a=r(4630),c=r(146),u=r(9525),s=n.Symbol,l=o("wks"),f=u?s.for||s:s&&s.withoutSetter||a;t.exports=function(t){return i(l,t)||(l[t]=c&&i(s,t)?s[t]:f("Symbol."+t)),l[t]}},5398:(t,e,r)=>{var n=r(9989),o=r(9037);n({global:!0,forced:o.globalThis!==o},{globalThis:o})},4149:(t,e,r)=>{var n=r(3803);t.exports=n}},e={};function r(n){var o=e[n];if(void 0!==o)return o.exports;var i=e[n]={exports:{}};return t[n].call(i.exports,i,i.exports,r),i.exports}r.m=t,r.u=t=>"OptiVorbis worker.js",r.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(t){if("object"==typeof window)return window}}(),r.o=(t,e)=>Object.prototype.hasOwnProperty.call(t,e),(()=>{var t;r.g.importScripts&&(t=r.g.location+"");var e=r.g.document;if(!t&&e&&(e.currentScript&&(t=e.currentScript.src),!t)){var n=e.getElementsByTagName("script");if(n.length)for(var o=n.length-1;o>-1&&!t;)t=n[o--].src}if(!t)throw new Error("Automatic publicPath is not supported in this browser");t=t.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),r.p=t})(),r.b=document.baseURI||self.location.href,(()=>{r(4149);const t=["bg-sky-300","dark:bg-sky-600"],e=document.getElementById("file-input"),n=document.getElementById("file-input-button"),o=document.getElementById("file-input-label");function i(t){let e=!1;if(t)for(let r=0;r<t.items.length&&!e;r+=1){const n=t.items[r],o=n.getAsFile();n.type.match(/^(?:audio|application|video)\/ogg/)&&(e=null==o||o)}return e}function a(e){const{dataTransfer:r}=e;i(r)&&(r.dropEffect="copy",o.classList.add(...t),e.preventDefault())}function c(){o.classList.remove(...t)}function u(r){const{dataTransfer:n}=r,a=i(n);if(a instanceof File){const n=new DataTransfer;n.items.add(a),e.files=n.files,e.dispatchEvent(new Event("input")),o.classList.remove(...t),r.preventDefault()}}const s=document.getElementById("file-input-section"),l=document.getElementById("output-file-section"),f=document.getElementById("output-file-preview"),p=document.getElementById("output-file-back"),d=document.getElementById("output-file-download"),v=document.getElementById("file-error-modal"),m=document.getElementById("file-error-modal-message");document.getElementById("file-error-modal-close").addEventListener("click",(()=>{v.classList.add("hidden")}));const g=document.getElementById("optivorbis-logo"),y=document.getElementById("file-input-section"),b=document.getElementById("file-input"),h=document.getElementById("file-process-text"),x=document.getElementById("output-file-preview"),w=document.getElementById("output-file-section");function E(){g.classList.remove("animate-pulse"),y.classList.remove("hidden"),h.classList.add("hidden")}globalThis.outputFileName="optimized_file.ogg";const O=new Worker(new URL(r.p+r.u(155),r.b),{name:"OptiVorbis worker"});window.addEventListener("DOMContentLoaded",(()=>{(function(t){b.addEventListener("input",(()=>{if(!b.files||0===b.files.length)return;const e=b.files[0];globalThis.outputFileName=e.name,g.classList.add("animate-pulse"),y.classList.add("hidden"),h.classList.remove("hidden"),t.postMessage(e),t.onerror=E,t.onmessageerror=E,t.onmessage=t=>{if(E(),"string"!=typeof t.data){const e=x.src;e&&(x.pause(),URL.revokeObjectURL(e)),x.src=URL.createObjectURL(t.data),y.classList.add("hidden"),w.classList.remove("hidden")}else e=t.data,m.innerText=e,v.classList.remove("hidden");var e},b.files=(new DataTransfer).files}))})(O),function(){const t=[o,n];for(let e=0;e<t.length;e+=1){const r=t[e];r.addEventListener("dragenter",a),r.addEventListener("dragover",a),r.addEventListener("dragleave",c),r.addEventListener("drop",u)}n.addEventListener("click",(()=>e.click()))}(),p.addEventListener("click",(()=>{const t=f.src;t&&(f.pause(),URL.revokeObjectURL(t)),s.classList.remove("hidden"),l.classList.add("hidden")})),d.addEventListener("click",(()=>{const t=document.createElement("a");t.href=f.src,t.download=globalThis.outputFileName,t.click()}))}))})()})();