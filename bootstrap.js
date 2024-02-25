/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".bootstrap.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/virtual_heart_bg.wasm": function() {
/******/ 			return {
/******/ 				"./virtual_heart_bg.js": {
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_cee7a886d55e7df5": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_instanceof_Window_cee7a886d55e7df5"](p0i32);
/******/ 					},
/******/ 					"__wbg_document_eb7fd66bde3ee213": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_document_eb7fd66bde3ee213"](p0i32);
/******/ 					},
/******/ 					"__wbg_body_874ccb42daaab363": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_body_874ccb42daaab363"](p0i32);
/******/ 					},
/******/ 					"__wbg_createElement_03cf347ddad1c8c0": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_createElement_03cf347ddad1c8c0"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_setAttribute_f7ffa687ef977957": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_setAttribute_f7ffa687ef977957"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_remove_f7de42c5f9035d0e": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_remove_f7de42c5f9035d0e"](p0i32);
/******/ 					},
/******/ 					"__wbg_append_fd99b0b80132b946": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_append_fd99b0b80132b946"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlElement_99861aeb7af981c2": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_instanceof_HtmlElement_99861aeb7af981c2"](p0i32);
/******/ 					},
/******/ 					"__wbg_offsetWidth_b5af4d8ba15fa071": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_offsetWidth_b5af4d8ba15fa071"](p0i32);
/******/ 					},
/******/ 					"__wbg_offsetHeight_6d87eac18f152bf4": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_offsetHeight_6d87eac18f152bf4"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_CanvasRenderingContext2d_4ec30ddd3f29f8f9": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_instanceof_CanvasRenderingContext2d_4ec30ddd3f29f8f9"](p0i32);
/******/ 					},
/******/ 					"__wbg_setstrokeStyle_4e23a7de64dd200e": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_setstrokeStyle_4e23a7de64dd200e"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setfillStyle_59f426135f52910f": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_setfillStyle_59f426135f52910f"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setlineWidth_2a86551e67f86eac": function(p0i32,p1f64) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_setlineWidth_2a86551e67f86eac"](p0i32,p1f64);
/******/ 					},
/******/ 					"__wbg_setfont_16d6e31e06a420a5": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_setfont_16d6e31e06a420a5"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_settextAlign_01ae8ab6d251b620": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_settextAlign_01ae8ab6d251b620"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_settextBaseline_c3266d3bd4a6695c": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_settextBaseline_c3266d3bd4a6695c"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_beginPath_88b78ef70b03baad": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_beginPath_88b78ef70b03baad"](p0i32);
/******/ 					},
/******/ 					"__wbg_fill_f13327c32ef64096": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_fill_f13327c32ef64096"](p0i32);
/******/ 					},
/******/ 					"__wbg_stroke_6c4d92cf5b29b95b": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_stroke_6c4d92cf5b29b95b"](p0i32);
/******/ 					},
/******/ 					"__wbg_arc_92cdcade9f4dc6a6": function(p0i32,p1f64,p2f64,p3f64,p4f64,p5f64) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_arc_92cdcade9f4dc6a6"](p0i32,p1f64,p2f64,p3f64,p4f64,p5f64);
/******/ 					},
/******/ 					"__wbg_lineTo_b4c86762deeba077": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_lineTo_b4c86762deeba077"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__wbg_moveTo_0bec47de5fc1f1dc": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_moveTo_0bec47de5fc1f1dc"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__wbg_fillRect_4dd28e628381d240": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_fillRect_4dd28e628381d240"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__wbg_restore_c066f36f0f2e06af": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_restore_c066f36f0f2e06af"](p0i32);
/******/ 					},
/******/ 					"__wbg_save_50ad60f57307aa8a": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_save_50ad60f57307aa8a"](p0i32);
/******/ 					},
/******/ 					"__wbg_fillText_07e5da9e41652f20": function(p0i32,p1i32,p2i32,p3f64,p4f64) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_fillText_07e5da9e41652f20"](p0i32,p1i32,p2i32,p3f64,p4f64);
/******/ 					},
/******/ 					"__wbg_rotate_a8734666fd1f1d1c": function(p0i32,p1f64) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_rotate_a8734666fd1f1d1c"](p0i32,p1f64);
/******/ 					},
/******/ 					"__wbg_translate_0602972ba441a2c9": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_translate_0602972ba441a2c9"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__wbg_width_aa1ac55fb41db6ae": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_width_aa1ac55fb41db6ae"](p0i32);
/******/ 					},
/******/ 					"__wbg_height_bea901cd16645fb7": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_height_bea901cd16645fb7"](p0i32);
/******/ 					},
/******/ 					"__wbg_getContext_dfc91ab0837db1d1": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_getContext_dfc91ab0837db1d1"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_settextContent_1493ae8928df81aa": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_settextContent_1493ae8928df81aa"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_cfecb3965268594c": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_newnoargs_cfecb3965268594c"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_3f093dd26d5569f8": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_call_3f093dd26d5569f8"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_self_05040bd9523805b9": function() {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_self_05040bd9523805b9"]();
/******/ 					},
/******/ 					"__wbg_window_adc720039f2cb14f": function() {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_window_adc720039f2cb14f"]();
/******/ 					},
/******/ 					"__wbg_globalThis_622105db80c1457d": function() {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_globalThis_622105db80c1457d"]();
/******/ 					},
/******/ 					"__wbg_global_f56b013ed9bcf359": function() {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_global_f56b013ed9bcf359"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_string_get": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbindgen_string_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_abs_ca27dcbf357a4d2c": function(p0f64) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_abs_ca27dcbf357a4d2c"](p0f64);
/******/ 					},
/******/ 					"__wbg_stringify_865daa6fb8c83d5a": function(p0i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbg_stringify_865daa6fb8c83d5a"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/virtual_heart_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/virtual_heart_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/virtual_heart_bg.wasm":"edc4c2709b653b8c8ca5"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// A dependency graph that contains any wasm must all be imported\n// asynchronously. This `bootstrap.js` file does the single async import, so\n// that no one else needs to worry about it again.\n__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./index.js */ \"./index.js\"))\n  .catch(e => console.error(\"Error importing `index.js`:\", e));\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });