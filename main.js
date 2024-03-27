(()=>{"use strict";var t={9510:(t,e,r)=>{r(5081),t.exports=r(4475)},9306:(t,e,r)=>{var n=r(4901),o=r(6823),i=TypeError;t.exports=function(t){if(n(t))return t;throw new i(o(t)+" is not a function")}},8551:(t,e,r)=>{var n=r(34),o=String,i=TypeError;t.exports=function(t){if(n(t))return t;throw new i(o(t)+" is not an object")}},9617:(t,e,r)=>{var n=r(5397),o=r(5610),i=r(6198),a=function(t){return function(e,r,a){var c=n(e),u=i(c);if(0===u)return!t&&-1;var s,l=o(a,u);if(t&&r!=r){for(;u>l;)if((s=c[l++])!=s)return!0}else for(;u>l;l++)if((t||l in c)&&c[l]===r)return t||l||0;return!t&&-1}};t.exports={includes:a(!0),indexOf:a(!1)}},4576:(t,e,r)=>{var n=r(9504),o=n({}.toString),i=n("".slice);t.exports=function(t){return i(o(t),8,-1)}},7740:(t,e,r)=>{var n=r(9297),o=r(5031),i=r(7347),a=r(4913);t.exports=function(t,e,r){for(var c=o(e),u=a.f,s=i.f,l=0;l<c.length;l++){var f=c[l];n(t,f)||r&&n(r,f)||u(t,f,s(e,f))}}},6699:(t,e,r)=>{var n=r(3724),o=r(4913),i=r(6980);t.exports=n?function(t,e,r){return o.f(t,e,i(1,r))}:function(t,e,r){return t[e]=r,t}},6980:t=>{t.exports=function(t,e){return{enumerable:!(1&t),configurable:!(2&t),writable:!(4&t),value:e}}},6840:(t,e,r)=>{var n=r(4901),o=r(4913),i=r(283),a=r(9433);t.exports=function(t,e,r,c){c||(c={});var u=c.enumerable,s=void 0!==c.name?c.name:e;if(n(r)&&i(r,s,c),c.global)u?t[e]=r:a(e,r);else{try{c.unsafe?t[e]&&(u=!0):delete t[e]}catch(t){}u?t[e]=r:o.f(t,e,{value:r,enumerable:!1,configurable:!c.nonConfigurable,writable:!c.nonWritable})}return t}},9433:(t,e,r)=>{var n=r(4475),o=Object.defineProperty;t.exports=function(t,e){try{o(n,t,{value:e,configurable:!0,writable:!0})}catch(r){n[t]=e}return e}},3724:(t,e,r)=>{var n=r(9039);t.exports=!n((function(){return 7!==Object.defineProperty({},1,{get:function(){return 7}})[1]}))},4055:(t,e,r)=>{var n=r(4475),o=r(34),i=n.document,a=o(i)&&o(i.createElement);t.exports=function(t){return a?i.createElement(t):{}}},9392:t=>{t.exports="undefined"!=typeof navigator&&String(navigator.userAgent)||""},7388:(t,e,r)=>{var n,o,i=r(4475),a=r(9392),c=i.process,u=i.Deno,s=c&&c.versions||u&&u.version,l=s&&s.v8;l&&(o=(n=l.split("."))[0]>0&&n[0]<4?1:+(n[0]+n[1])),!o&&a&&(!(n=a.match(/Edge\/(\d+)/))||n[1]>=74)&&(n=a.match(/Chrome\/(\d+)/))&&(o=+n[1]),t.exports=o},8727:t=>{t.exports=["constructor","hasOwnProperty","isPrototypeOf","propertyIsEnumerable","toLocaleString","toString","valueOf"]},6518:(t,e,r)=>{var n=r(4475),o=r(7347).f,i=r(6699),a=r(6840),c=r(9433),u=r(7740),s=r(2796);t.exports=function(t,e){var r,l,f,p,d,v=t.target,m=t.global,g=t.stat;if(r=m?n:g?n[v]||c(v,{}):n[v]&&n[v].prototype)for(l in e){if(p=e[l],f=t.dontCallGetSet?(d=o(r,l))&&d.value:r[l],!s(m?l:v+(g?".":"#")+l,t.forced)&&void 0!==f){if(typeof p==typeof f)continue;u(p,f)}(t.sham||f&&f.sham)&&i(p,"sham",!0),a(r,l,p,t)}}},9039:t=>{t.exports=function(t){try{return!!t()}catch(t){return!0}}},616:(t,e,r)=>{var n=r(9039);t.exports=!n((function(){var t=function(){}.bind();return"function"!=typeof t||t.hasOwnProperty("prototype")}))},9565:(t,e,r)=>{var n=r(616),o=Function.prototype.call;t.exports=n?o.bind(o):function(){return o.apply(o,arguments)}},350:(t,e,r)=>{var n=r(3724),o=r(9297),i=Function.prototype,a=n&&Object.getOwnPropertyDescriptor,c=o(i,"name"),u=c&&"something"===function(){}.name,s=c&&(!n||n&&a(i,"name").configurable);t.exports={EXISTS:c,PROPER:u,CONFIGURABLE:s}},9504:(t,e,r)=>{var n=r(616),o=Function.prototype,i=o.call,a=n&&o.bind.bind(i,i);t.exports=n?a:function(t){return function(){return i.apply(t,arguments)}}},7751:(t,e,r)=>{var n=r(4475),o=r(4901);t.exports=function(t,e){return arguments.length<2?(r=n[t],o(r)?r:void 0):n[t]&&n[t][e];var r}},5966:(t,e,r)=>{var n=r(9306),o=r(4117);t.exports=function(t,e){var r=t[e];return o(r)?void 0:n(r)}},4475:function(t,e,r){var n=function(t){return t&&t.Math===Math&&t};t.exports=n("object"==typeof globalThis&&globalThis)||n("object"==typeof window&&window)||n("object"==typeof self&&self)||n("object"==typeof r.g&&r.g)||n("object"==typeof this&&this)||function(){return this}()||Function("return this")()},9297:(t,e,r)=>{var n=r(9504),o=r(8981),i=n({}.hasOwnProperty);t.exports=Object.hasOwn||function(t,e){return i(o(t),e)}},421:t=>{t.exports={}},5917:(t,e,r)=>{var n=r(3724),o=r(9039),i=r(4055);t.exports=!n&&!o((function(){return 7!==Object.defineProperty(i("div"),"a",{get:function(){return 7}}).a}))},7055:(t,e,r)=>{var n=r(9504),o=r(9039),i=r(4576),a=Object,c=n("".split);t.exports=o((function(){return!a("z").propertyIsEnumerable(0)}))?function(t){return"String"===i(t)?c(t,""):a(t)}:a},3706:(t,e,r)=>{var n=r(9504),o=r(4901),i=r(7629),a=n(Function.toString);o(i.inspectSource)||(i.inspectSource=function(t){return a(t)}),t.exports=i.inspectSource},1181:(t,e,r)=>{var n,o,i,a=r(8622),c=r(4475),u=r(34),s=r(6699),l=r(9297),f=r(7629),p=r(6119),d=r(421),v="Object already initialized",m=c.TypeError,g=c.WeakMap;if(a||f.state){var y=f.state||(f.state=new g);y.get=y.get,y.has=y.has,y.set=y.set,n=function(t,e){if(y.has(t))throw new m(v);return e.facade=t,y.set(t,e),e},o=function(t){return y.get(t)||{}},i=function(t){return y.has(t)}}else{var b=p("state");d[b]=!0,n=function(t,e){if(l(t,b))throw new m(v);return e.facade=t,s(t,b,e),e},o=function(t){return l(t,b)?t[b]:{}},i=function(t){return l(t,b)}}t.exports={set:n,get:o,has:i,enforce:function(t){return i(t)?o(t):n(t,{})},getterFor:function(t){return function(e){var r;if(!u(e)||(r=o(e)).type!==t)throw new m("Incompatible receiver, "+t+" required");return r}}}},4901:t=>{var e="object"==typeof document&&document.all;t.exports=void 0===e&&void 0!==e?function(t){return"function"==typeof t||t===e}:function(t){return"function"==typeof t}},2796:(t,e,r)=>{var n=r(9039),o=r(4901),i=/#|\.prototype\./,a=function(t,e){var r=u[c(t)];return r===l||r!==s&&(o(e)?n(e):!!e)},c=a.normalize=function(t){return String(t).replace(i,".").toLowerCase()},u=a.data={},s=a.NATIVE="N",l=a.POLYFILL="P";t.exports=a},4117:t=>{t.exports=function(t){return null==t}},34:(t,e,r)=>{var n=r(4901);t.exports=function(t){return"object"==typeof t?null!==t:n(t)}},6395:t=>{t.exports=!1},757:(t,e,r)=>{var n=r(7751),o=r(4901),i=r(1625),a=r(7040),c=Object;t.exports=a?function(t){return"symbol"==typeof t}:function(t){var e=n("Symbol");return o(e)&&i(e.prototype,c(t))}},6198:(t,e,r)=>{var n=r(8014);t.exports=function(t){return n(t.length)}},283:(t,e,r)=>{var n=r(9504),o=r(9039),i=r(4901),a=r(9297),c=r(3724),u=r(350).CONFIGURABLE,s=r(3706),l=r(1181),f=l.enforce,p=l.get,d=String,v=Object.defineProperty,m=n("".slice),g=n("".replace),y=n([].join),b=c&&!o((function(){return 8!==v((function(){}),"length",{value:8}).length})),h=String(String).split("String"),x=t.exports=function(t,e,r){"Symbol("===m(d(e),0,7)&&(e="["+g(d(e),/^Symbol\(([^)]*)\).*$/,"$1")+"]"),r&&r.getter&&(e="get "+e),r&&r.setter&&(e="set "+e),(!a(t,"name")||u&&t.name!==e)&&(c?v(t,"name",{value:e,configurable:!0}):t.name=e),b&&r&&a(r,"arity")&&t.length!==r.arity&&v(t,"length",{value:r.arity});try{r&&a(r,"constructor")&&r.constructor?c&&v(t,"prototype",{writable:!1}):t.prototype&&(t.prototype=void 0)}catch(t){}var n=f(t);return a(n,"source")||(n.source=y(h,"string"==typeof e?e:"")),t};Function.prototype.toString=x((function(){return i(this)&&p(this).source||s(this)}),"toString")},741:t=>{var e=Math.ceil,r=Math.floor;t.exports=Math.trunc||function(t){var n=+t;return(n>0?r:e)(n)}},4913:(t,e,r)=>{var n=r(3724),o=r(5917),i=r(8686),a=r(8551),c=r(6969),u=TypeError,s=Object.defineProperty,l=Object.getOwnPropertyDescriptor,f="enumerable",p="configurable",d="writable";e.f=n?i?function(t,e,r){if(a(t),e=c(e),a(r),"function"==typeof t&&"prototype"===e&&"value"in r&&d in r&&!r[d]){var n=l(t,e);n&&n[d]&&(t[e]=r.value,r={configurable:p in r?r[p]:n[p],enumerable:f in r?r[f]:n[f],writable:!1})}return s(t,e,r)}:s:function(t,e,r){if(a(t),e=c(e),a(r),o)try{return s(t,e,r)}catch(t){}if("get"in r||"set"in r)throw new u("Accessors not supported");return"value"in r&&(t[e]=r.value),t}},7347:(t,e,r)=>{var n=r(3724),o=r(9565),i=r(8773),a=r(6980),c=r(5397),u=r(6969),s=r(9297),l=r(5917),f=Object.getOwnPropertyDescriptor;e.f=n?f:function(t,e){if(t=c(t),e=u(e),l)try{return f(t,e)}catch(t){}if(s(t,e))return a(!o(i.f,t,e),t[e])}},8480:(t,e,r)=>{var n=r(1828),o=r(8727).concat("length","prototype");e.f=Object.getOwnPropertyNames||function(t){return n(t,o)}},3717:(t,e)=>{e.f=Object.getOwnPropertySymbols},1625:(t,e,r)=>{var n=r(9504);t.exports=n({}.isPrototypeOf)},1828:(t,e,r)=>{var n=r(9504),o=r(9297),i=r(5397),a=r(9617).indexOf,c=r(421),u=n([].push);t.exports=function(t,e){var r,n=i(t),s=0,l=[];for(r in n)!o(c,r)&&o(n,r)&&u(l,r);for(;e.length>s;)o(n,r=e[s++])&&(~a(l,r)||u(l,r));return l}},8773:(t,e)=>{var r={}.propertyIsEnumerable,n=Object.getOwnPropertyDescriptor,o=n&&!r.call({1:2},1);e.f=o?function(t){var e=n(this,t);return!!e&&e.enumerable}:r},4270:(t,e,r)=>{var n=r(9565),o=r(4901),i=r(34),a=TypeError;t.exports=function(t,e){var r,c;if("string"===e&&o(r=t.toString)&&!i(c=n(r,t)))return c;if(o(r=t.valueOf)&&!i(c=n(r,t)))return c;if("string"!==e&&o(r=t.toString)&&!i(c=n(r,t)))return c;throw new a("Can't convert object to primitive value")}},5031:(t,e,r)=>{var n=r(7751),o=r(9504),i=r(8480),a=r(3717),c=r(8551),u=o([].concat);t.exports=n("Reflect","ownKeys")||function(t){var e=i.f(c(t)),r=a.f;return r?u(e,r(t)):e}},7750:(t,e,r)=>{var n=r(4117),o=TypeError;t.exports=function(t){if(n(t))throw new o("Can't call method on "+t);return t}},6119:(t,e,r)=>{var n=r(5745),o=r(3392),i=n("keys");t.exports=function(t){return i[t]||(i[t]=o(t))}},7629:(t,e,r)=>{var n=r(6395),o=r(4475),i=r(9433),a="__core-js_shared__",c=t.exports=o[a]||i(a,{});(c.versions||(c.versions=[])).push({version:"3.36.1",mode:n?"pure":"global",copyright:"© 2014-2024 Denis Pushkarev (zloirock.ru)",license:"https://github.com/zloirock/core-js/blob/v3.36.1/LICENSE",source:"https://github.com/zloirock/core-js"})},5745:(t,e,r)=>{var n=r(7629);t.exports=function(t,e){return n[t]||(n[t]=e||{})}},4495:(t,e,r)=>{var n=r(7388),o=r(9039),i=r(4475).String;t.exports=!!Object.getOwnPropertySymbols&&!o((function(){var t=Symbol("symbol detection");return!i(t)||!(Object(t)instanceof Symbol)||!Symbol.sham&&n&&n<41}))},5610:(t,e,r)=>{var n=r(1291),o=Math.max,i=Math.min;t.exports=function(t,e){var r=n(t);return r<0?o(r+e,0):i(r,e)}},5397:(t,e,r)=>{var n=r(7055),o=r(7750);t.exports=function(t){return n(o(t))}},1291:(t,e,r)=>{var n=r(741);t.exports=function(t){var e=+t;return e!=e||0===e?0:n(e)}},8014:(t,e,r)=>{var n=r(1291),o=Math.min;t.exports=function(t){var e=n(t);return e>0?o(e,9007199254740991):0}},8981:(t,e,r)=>{var n=r(7750),o=Object;t.exports=function(t){return o(n(t))}},2777:(t,e,r)=>{var n=r(9565),o=r(34),i=r(757),a=r(5966),c=r(4270),u=r(8227),s=TypeError,l=u("toPrimitive");t.exports=function(t,e){if(!o(t)||i(t))return t;var r,u=a(t,l);if(u){if(void 0===e&&(e="default"),r=n(u,t,e),!o(r)||i(r))return r;throw new s("Can't convert object to primitive value")}return void 0===e&&(e="number"),c(t,e)}},6969:(t,e,r)=>{var n=r(2777),o=r(757);t.exports=function(t){var e=n(t,"string");return o(e)?e:e+""}},6823:t=>{var e=String;t.exports=function(t){try{return e(t)}catch(t){return"Object"}}},3392:(t,e,r)=>{var n=r(9504),o=0,i=Math.random(),a=n(1..toString);t.exports=function(t){return"Symbol("+(void 0===t?"":t)+")_"+a(++o+i,36)}},7040:(t,e,r)=>{var n=r(4495);t.exports=n&&!Symbol.sham&&"symbol"==typeof Symbol.iterator},8686:(t,e,r)=>{var n=r(3724),o=r(9039);t.exports=n&&o((function(){return 42!==Object.defineProperty((function(){}),"prototype",{value:42,writable:!1}).prototype}))},8622:(t,e,r)=>{var n=r(4475),o=r(4901),i=n.WeakMap;t.exports=o(i)&&/native code/.test(String(i))},8227:(t,e,r)=>{var n=r(4475),o=r(5745),i=r(9297),a=r(3392),c=r(4495),u=r(7040),s=n.Symbol,l=o("wks"),f=u?s.for||s:s&&s.withoutSetter||a;t.exports=function(t){return i(l,t)||(l[t]=c&&i(s,t)?s[t]:f("Symbol."+t)),l[t]}},5081:(t,e,r)=>{var n=r(6518),o=r(4475);n({global:!0,forced:o.globalThis!==o},{globalThis:o})},657:(t,e,r)=>{var n=r(9510);t.exports=n}},e={};function r(n){var o=e[n];if(void 0!==o)return o.exports;var i=e[n]={exports:{}};return t[n].call(i.exports,i,i.exports,r),i.exports}r.m=t,r.u=t=>"OptiVorbis worker.js",r.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(t){if("object"==typeof window)return window}}(),r.o=(t,e)=>Object.prototype.hasOwnProperty.call(t,e),(()=>{var t;r.g.importScripts&&(t=r.g.location+"");var e=r.g.document;if(!t&&e&&(e.currentScript&&(t=e.currentScript.src),!t)){var n=e.getElementsByTagName("script");if(n.length)for(var o=n.length-1;o>-1&&(!t||!/^http(s?):/.test(t));)t=n[o--].src}if(!t)throw new Error("Automatic publicPath is not supported in this browser");t=t.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),r.p=t})(),r.b=document.baseURI||self.location.href,(()=>{r(657);const t=["bg-sky-300","dark:bg-sky-600"],e=document.getElementById("file-input"),n=document.getElementById("file-input-button"),o=document.getElementById("file-input-label");function i(t){let e=!1;if(t)for(let r=0;r<t.items.length&&!e;r+=1){const n=t.items[r],o=n.getAsFile();n.type.match(/^(?:audio|application|video)\/ogg/)&&(e=null==o||o)}return e}function a(e){const{dataTransfer:r}=e;i(r)&&(r.dropEffect="copy",o.classList.add(...t),e.preventDefault())}function c(){o.classList.remove(...t)}function u(r){const{dataTransfer:n}=r,a=i(n);if(a instanceof File){const n=new DataTransfer;n.items.add(a),e.files=n.files,e.dispatchEvent(new Event("input")),o.classList.remove(...t),r.preventDefault()}}const s=document.getElementById("file-input-section"),l=document.getElementById("output-file-section"),f=document.getElementById("output-file-preview"),p=document.getElementById("output-file-back"),d=document.getElementById("output-file-download"),v=document.getElementById("file-error-modal"),m=document.getElementById("file-error-modal-message");document.getElementById("file-error-modal-close").addEventListener("click",(()=>{v.classList.add("hidden")}));const g=document.getElementById("optivorbis-logo"),y=document.getElementById("file-input-section"),b=document.getElementById("file-input"),h=document.getElementById("file-process-text"),x=document.getElementById("output-file-preview"),w=document.getElementById("output-file-section");function E(){g.classList.remove("animate-pulse"),y.classList.remove("hidden"),h.classList.add("hidden")}globalThis.outputFileName="optimized_file.ogg";const O=new Worker(new URL(r.p+r.u(812),r.b),{name:"OptiVorbis worker"});window.addEventListener("DOMContentLoaded",(()=>{(function(t){b.addEventListener("input",(()=>{if(!b.files||0===b.files.length)return;const e=b.files[0];globalThis.outputFileName=e.name,g.classList.add("animate-pulse"),y.classList.add("hidden"),h.classList.remove("hidden"),t.postMessage(e),t.onerror=E,t.onmessageerror=E,t.onmessage=t=>{if(E(),"string"!=typeof t.data){const e=x.src;e&&(x.pause(),URL.revokeObjectURL(e)),x.src=URL.createObjectURL(t.data),y.classList.add("hidden"),w.classList.remove("hidden")}else e=t.data,m.innerText=e,v.classList.remove("hidden");var e},b.files=(new DataTransfer).files}))})(O),function(){const t=[o,n];for(let e=0;e<t.length;e+=1){const r=t[e];r.addEventListener("dragenter",a),r.addEventListener("dragover",a),r.addEventListener("dragleave",c),r.addEventListener("drop",u)}n.addEventListener("click",(()=>e.click()))}(),p.addEventListener("click",(()=>{const t=f.src;t&&(f.pause(),URL.revokeObjectURL(t)),s.classList.remove("hidden"),l.classList.add("hidden")})),d.addEventListener("click",(()=>{const t=document.createElement("a");t.href=f.src,t.download=globalThis.outputFileName,t.click()}))}))})()})();