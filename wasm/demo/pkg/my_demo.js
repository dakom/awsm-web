
const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : webkitAudioContext);
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachegetFloat64Memory0;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_32(arg0, arg1, arg2) {
    try {
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcac05c548db2d908(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_35(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6d35332ea1bf4c6a(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_38(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc5ca5b9daac7a878(arg0, arg1);
}

function __wbg_adapter_41(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he9844c606aeffa26(arg0, arg1);
}

function __wbg_adapter_44(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf26f0f4fccb085d9(arg0, arg1, arg2);
}

function __wbg_adapter_47(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__haa64308ee4a357c0(arg0, arg1, addHeapObject(arg2));
}

/**
*/
export function run() {
    wasm.run();
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    };
}

let cachegetFloat32Memory0 = null;
function getFloat32Memory0() {
    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory0;
}

function getArrayF32FromWasm0(ptr, len) {
    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);
}
function __wbg_adapter_563(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h376e10714cc85c90(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {

        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_is_null = function(arg0) {
        var ret = getObject(arg0) === null;
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_new_68adb0d58759a4ed = function() {
        var ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_2e79e744454afade = function(arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        var ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_instanceof_WebGl2RenderingContext_97ea0fcd122c6bea = function(arg0) {
        var ret = getObject(arg0) instanceof WebGL2RenderingContext;
        return ret;
    };
    imports.wbg.__wbg_canvas_9d18bcba889ec398 = function(arg0) {
        var ret = getObject(arg0).canvas;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_drawingBufferWidth_a9356943b813b154 = function(arg0) {
        var ret = getObject(arg0).drawingBufferWidth;
        return ret;
    };
    imports.wbg.__wbg_drawingBufferHeight_1fb3e7ee86e13056 = function(arg0) {
        var ret = getObject(arg0).drawingBufferHeight;
        return ret;
    };
    imports.wbg.__wbg_bindBufferBase_6ff29514b22d37bd = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bindBufferBase(arg1 >>> 0, arg2 >>> 0, getObject(arg3));
    };
    imports.wbg.__wbg_bindVertexArray_59be0e2201d7dbdd = function(arg0, arg1) {
        getObject(arg0).bindVertexArray(getObject(arg1));
    };
    imports.wbg.__wbg_bufferData_50d5fe41aec684e4 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
    };
    imports.wbg.__wbg_bufferData_61df55c27467e54c = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).bufferData(arg1 >>> 0, getArrayU8FromWasm0(arg2, arg3), arg4 >>> 0);
    };
    imports.wbg.__wbg_bufferSubData_e37facb920e00804 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).bufferSubData(arg1 >>> 0, arg2, getObject(arg3), arg4 >>> 0, arg5 >>> 0);
    };
    imports.wbg.__wbg_createVertexArray_cc565c538c3ff35e = function(arg0) {
        var ret = getObject(arg0).createVertexArray();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_drawArraysInstanced_13db72d779b93602 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawArraysInstanced(arg1 >>> 0, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_drawBuffers_d1bddd98e11d3e24 = function(arg0, arg1) {
        getObject(arg0).drawBuffers(getObject(arg1));
    };
    imports.wbg.__wbg_getActiveUniformBlockName_880070341e639bd3 = function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg1).getActiveUniformBlockName(getObject(arg2), arg3 >>> 0);
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getActiveUniformBlockParameter_c54fdcd7cff61d65 = handleError(function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getActiveUniformBlockParameter(getObject(arg1), arg2 >>> 0, arg3 >>> 0);
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_getActiveUniforms_01c71a6480d5f845 = function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getActiveUniforms(getObject(arg1), getObject(arg2), arg3 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getUniformBlockIndex_cf9cce994b01a40a = function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getUniformBlockIndex(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return ret;
    };
    imports.wbg.__wbg_readPixels_3c36115cd1d989ac = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
        getObject(arg0).readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7 === 0 ? undefined : getArrayU8FromWasm0(arg7, arg8));
    });
    imports.wbg.__wbg_texImage2D_af68d96615827c77 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
    });
    imports.wbg.__wbg_texImage2D_fd8de2fd3c4b5ad9 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    });
    imports.wbg.__wbg_texImage2D_2d29544b24c537db = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    });
    imports.wbg.__wbg_texImage2D_92f34fa419290ed7 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    });
    imports.wbg.__wbg_texImage2D_a6f2757b7cbb674e = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    });
    imports.wbg.__wbg_texImage2D_178a900e383ef0f6 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    });
    imports.wbg.__wbg_texImage3D_ec4a4f864e3fe7b9 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
        getObject(arg0).texImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8 >>> 0, arg9 >>> 0, getObject(arg10));
    });
    imports.wbg.__wbg_uniform1fv_d9abea436c26a6ec = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform1fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform1ui_e26568591622d0e8 = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1ui(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_uniform2fv_2df595964f6fa4df = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform3fv_669e205afcea468f = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform3fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform4fv_eccb883c3863674e = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform4fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniformBlockBinding_23e4ab8e329dbd44 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniformBlockBinding(getObject(arg1), arg2 >>> 0, arg3 >>> 0);
    };
    imports.wbg.__wbg_uniformMatrix2fv_99cb2e04610a787a = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix2fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix3fv_fcad2e5e4920369e = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix3fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix4fv_3948f944a005bdd0 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_vertexAttribDivisor_287d4982f5c651cf = function(arg0, arg1, arg2) {
        getObject(arg0).vertexAttribDivisor(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_activeTexture_8c0f299bfec3fb46 = function(arg0, arg1) {
        getObject(arg0).activeTexture(arg1 >>> 0);
    };
    imports.wbg.__wbg_attachShader_5eff139e1b567fc5 = function(arg0, arg1, arg2) {
        getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
    };
    imports.wbg.__wbg_bindAttribLocation_c64d9b3ccd0ee4fe = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).bindAttribLocation(getObject(arg1), arg2 >>> 0, getStringFromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_bindBuffer_4d9ee91ad842ba92 = function(arg0, arg1, arg2) {
        getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_bindFramebuffer_909497f01b22ae78 = function(arg0, arg1, arg2) {
        getObject(arg0).bindFramebuffer(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_bindRenderbuffer_5709350385f688fe = function(arg0, arg1, arg2) {
        getObject(arg0).bindRenderbuffer(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_bindTexture_28d8d5d1a523ac28 = function(arg0, arg1, arg2) {
        getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_blendFunc_17a5b103e8a5ae98 = function(arg0, arg1, arg2) {
        getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_checkFramebufferStatus_3538e03ee8ff5d51 = function(arg0, arg1) {
        var ret = getObject(arg0).checkFramebufferStatus(arg1 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_clear_3fcaf4948a30f1fa = function(arg0, arg1) {
        getObject(arg0).clear(arg1 >>> 0);
    };
    imports.wbg.__wbg_clearColor_6b9aa12de1c75178 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_compileShader_54ff8de585d7b3ee = function(arg0, arg1) {
        getObject(arg0).compileShader(getObject(arg1));
    };
    imports.wbg.__wbg_createBuffer_ff6fae04a8379012 = function(arg0) {
        var ret = getObject(arg0).createBuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createFramebuffer_cff051073cc4f9de = function(arg0) {
        var ret = getObject(arg0).createFramebuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createProgram_4f13aad8942a0f0d = function(arg0) {
        var ret = getObject(arg0).createProgram();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createRenderbuffer_e0e686c2c64ac051 = function(arg0) {
        var ret = getObject(arg0).createRenderbuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createShader_bc96d3d7082bf183 = function(arg0, arg1) {
        var ret = getObject(arg0).createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createTexture_c87d4dc81bfac194 = function(arg0) {
        var ret = getObject(arg0).createTexture();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_deleteProgram_04bdadacd621fd42 = function(arg0, arg1) {
        getObject(arg0).deleteProgram(getObject(arg1));
    };
    imports.wbg.__wbg_deleteShader_b109483996812918 = function(arg0, arg1) {
        getObject(arg0).deleteShader(getObject(arg1));
    };
    imports.wbg.__wbg_depthMask_49cd0944b9f981a2 = function(arg0, arg1) {
        getObject(arg0).depthMask(arg1 !== 0);
    };
    imports.wbg.__wbg_detachShader_eb4a679ebe47554f = function(arg0, arg1, arg2) {
        getObject(arg0).detachShader(getObject(arg1), getObject(arg2));
    };
    imports.wbg.__wbg_drawArrays_0f3e5369dd3cb8d5 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__wbg_drawElements_f46757e11dccffd3 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
    };
    imports.wbg.__wbg_enable_b726776a768bd24d = function(arg0, arg1) {
        getObject(arg0).enable(arg1 >>> 0);
    };
    imports.wbg.__wbg_enableVertexAttribArray_ccbac340a301dac1 = function(arg0, arg1) {
        getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_framebufferRenderbuffer_b253d56019536236 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4));
    };
    imports.wbg.__wbg_framebufferTexture2D_2bdf4f4f57064288 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4), arg5);
    };
    imports.wbg.__wbg_generateMipmap_dcae0c8987f25efc = function(arg0, arg1) {
        getObject(arg0).generateMipmap(arg1 >>> 0);
    };
    imports.wbg.__wbg_getActiveAttrib_75d10f5fc4921a92 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getActiveAttrib(getObject(arg1), arg2 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_getActiveUniform_2940e4f4460dc5c8 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getActiveUniform(getObject(arg1), arg2 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_getAttribLocation_d4033a8b97a4ad6b = function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getAttribLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return ret;
    };
    imports.wbg.__wbg_getParameter_acf39109736e1337 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).getParameter(arg1 >>> 0);
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_getProgramInfoLog_990ced340fabdcbb = function(arg0, arg1, arg2) {
        var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getProgramParameter_e44f5daeacd98160 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getShaderInfoLog_fd5c172fb8c03b96 = function(arg0, arg1, arg2) {
        var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getShaderParameter_7071856a1f55aa97 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getUniformLocation_4b3c35f82457bdf3 = function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_linkProgram_dc68765f87076be7 = function(arg0, arg1) {
        getObject(arg0).linkProgram(getObject(arg1));
    };
    imports.wbg.__wbg_pixelStorei_8faf6f339541eae6 = function(arg0, arg1, arg2) {
        getObject(arg0).pixelStorei(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_renderbufferStorage_fea6f3dcc99c9d47 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
    };
    imports.wbg.__wbg_shaderSource_dc7ae56eec603808 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_texParameteri_c1771e59529aea60 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_uniform1f_2a011c1edaa3c88b = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1f(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_uniform1i_e99a16be2875dc87 = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1i(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_uniform2f_feef43477f513bae = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
    };
    imports.wbg.__wbg_uniform3f_9af5e3c63db096c0 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniform3f(getObject(arg1), arg2, arg3, arg4);
    };
    imports.wbg.__wbg_uniform4f_45e68429de9d6360 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
    };
    imports.wbg.__wbg_useProgram_f3c9bc74cb69780f = function(arg0, arg1) {
        getObject(arg0).useProgram(getObject(arg1));
    };
    imports.wbg.__wbg_vertexAttribPointer_12cd254a2e5c6988 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    };
    imports.wbg.__wbg_viewport_0b056ac66214d59d = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).viewport(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_instanceof_Window_e8f84259147dce74 = function(arg0) {
        var ret = getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_document_d3b6d86af1c5d199 = function(arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_location_4c98b1eeb5ceccc0 = function(arg0) {
        var ret = getObject(arg0).location;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_innerWidth_2a084ee2fb8c0457 = handleError(function(arg0) {
        var ret = getObject(arg0).innerWidth;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_innerHeight_4676de3f9d6f79be = handleError(function(arg0) {
        var ret = getObject(arg0).innerHeight;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_alert_b8cc97eba2487980 = handleError(function(arg0, arg1, arg2) {
        getObject(arg0).alert(getStringFromWasm0(arg1, arg2));
    });
    imports.wbg.__wbg_cancelAnimationFrame_396f71da29fb2b46 = handleError(function(arg0, arg1) {
        getObject(arg0).cancelAnimationFrame(arg1);
    });
    imports.wbg.__wbg_requestAnimationFrame_e5d576010b9bc3a3 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        return ret;
    });
    imports.wbg.__wbg_fetch_39dfad16fe04318d = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).fetch(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_967775b24c689b32 = function(arg0) {
        var ret = getObject(arg0) instanceof CanvasRenderingContext2D;
        return ret;
    };
    imports.wbg.__wbg_setfillStyle_c05ba2508c693321 = function(arg0, arg1) {
        getObject(arg0).fillStyle = getObject(arg1);
    };
    imports.wbg.__wbg_fillRect_57b5c7207b51d2b9 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_new_9a8e1cb83c7d49c5 = handleError(function(arg0, arg1) {
        var ret = new Event(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_src_a31acd9187eac9d0 = function(arg0, arg1) {
        var ret = getObject(arg1).src;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setsrc_e1f6baacb3a92a01 = function(arg0, arg1, arg2) {
        getObject(arg0).src = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setcrossOrigin_6019a824b8cd56ce = function(arg0, arg1, arg2) {
        getObject(arg0).crossOrigin = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_naturalWidth_050289a437a51301 = function(arg0) {
        var ret = getObject(arg0).naturalWidth;
        return ret;
    };
    imports.wbg.__wbg_naturalHeight_44769c9ca53dbee0 = function(arg0) {
        var ret = getObject(arg0).naturalHeight;
        return ret;
    };
    imports.wbg.__wbg_new_ee3c59abbd5bd329 = handleError(function() {
        var ret = new Image();
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_settextContent_d2083d38d155212a = function(arg0, arg1, arg2) {
        getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_appendChild_8658f795c44d1316 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_removeChild_be8cb6ec13799e04 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).removeChild(getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_setProperty_4a05a7c81066031f = handleError(function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    });
    imports.wbg.__wbg_origin_1acc71a728d92be7 = handleError(function(arg0, arg1) {
        var ret = getObject(arg1).origin;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    });
    imports.wbg.__wbg_pathname_e5d7dbebdeeddf83 = handleError(function(arg0, arg1) {
        var ret = getObject(arg1).pathname;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    });
    imports.wbg.__wbg_search_9444c9b4f9d75474 = handleError(function(arg0, arg1) {
        var ret = getObject(arg1).search;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    });
    imports.wbg.__wbg_ok_e3e9190b50b9cdb6 = function(arg0) {
        var ret = getObject(arg0).ok;
        return ret;
    };
    imports.wbg.__wbg_arrayBuffer_b6591b9332fe79c7 = handleError(function(arg0) {
        var ret = getObject(arg0).arrayBuffer();
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_json_6b2a36109345a9ee = handleError(function(arg0) {
        var ret = getObject(arg0).json();
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_text_e038bae70fd539db = handleError(function(arg0) {
        var ret = getObject(arg0).text();
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_drawBuffersWEBGL_900926a3ce0fdeaf = function(arg0, arg1) {
        getObject(arg0).drawBuffersWEBGL(getObject(arg1));
    };
    imports.wbg.__wbg_instanceof_MouseEvent_8bb316f1b7a90822 = function(arg0) {
        var ret = getObject(arg0) instanceof MouseEvent;
        return ret;
    };
    imports.wbg.__wbg_clientX_35a1d3ce225575e1 = function(arg0) {
        var ret = getObject(arg0).clientX;
        return ret;
    };
    imports.wbg.__wbg_clientY_b7871a972b1e0e0b = function(arg0) {
        var ret = getObject(arg0).clientY;
        return ret;
    };
    imports.wbg.__wbg_movementX_653e3c4e9b4895d6 = function(arg0) {
        var ret = getObject(arg0).movementX;
        return ret;
    };
    imports.wbg.__wbg_movementY_398488c8f6c40187 = function(arg0) {
        var ret = getObject(arg0).movementY;
        return ret;
    };
    imports.wbg.__wbg_bindVertexArrayOES_c00fef873be0e2f3 = function(arg0, arg1) {
        getObject(arg0).bindVertexArrayOES(getObject(arg1));
    };
    imports.wbg.__wbg_createVertexArrayOES_267fca9d282a48fe = function(arg0) {
        var ret = getObject(arg0).createVertexArrayOES();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_origin_647f52cc075efef7 = function(arg0, arg1) {
        var ret = getObject(arg1).origin;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_new_c7dacb15fa970e7c = handleError(function(arg0, arg1) {
        var ret = new URL(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_createObjectURL_56f69e60ba5ed5a6 = handleError(function(arg0, arg1) {
        var ret = URL.createObjectURL(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    });
    imports.wbg.__wbg_setbuffer_a11fc5f455527b2e = function(arg0, arg1) {
        getObject(arg0).buffer = getObject(arg1);
    };
    imports.wbg.__wbg_setloop_692aa5fa77d1435e = function(arg0, arg1) {
        getObject(arg0).loop = arg1 !== 0;
    };
    imports.wbg.__wbg_setonended_e453c8052ef88cfe = function(arg0, arg1) {
        getObject(arg0).onended = getObject(arg1);
    };
    imports.wbg.__wbg_start_90936df7ba31920f = handleError(function(arg0) {
        getObject(arg0).start();
    });
    imports.wbg.__wbg_stop_de4249c96d90330b = handleError(function(arg0) {
        getObject(arg0).stop();
    });
    imports.wbg.__wbg_destination_4a04ba9566943864 = function(arg0) {
        var ret = getObject(arg0).destination;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_583bf1d2aa1a9ec5 = handleError(function() {
        var ret = new lAudioContext();
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_createBufferSource_1c77eeba474fe9a5 = handleError(function(arg0) {
        var ret = getObject(arg0).createBufferSource();
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_decodeAudioData_97780e8562fbbd62 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).decodeAudioData(getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_sethref_f87ddf6ac0b0a380 = function(arg0, arg1, arg2) {
        getObject(arg0).href = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_newwithstr_3a742362b742e6ac = handleError(function(arg0, arg1) {
        var ret = new Request(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_body_61c142aa6eae691f = function(arg0) {
        var ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_pointerLockElement_239774753ce7e7d2 = function(arg0) {
        var ret = getObject(arg0).pointerLockElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_d00b8e24838e36e1 = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_instanceof_WebGlRenderingContext_b25acea07fa8a767 = function(arg0) {
        var ret = getObject(arg0) instanceof WebGLRenderingContext;
        return ret;
    };
    imports.wbg.__wbg_canvas_2537ca49cd201230 = function(arg0) {
        var ret = getObject(arg0).canvas;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_drawingBufferWidth_5a39a19f3184adf0 = function(arg0) {
        var ret = getObject(arg0).drawingBufferWidth;
        return ret;
    };
    imports.wbg.__wbg_drawingBufferHeight_34a99cdbbd30e24c = function(arg0) {
        var ret = getObject(arg0).drawingBufferHeight;
        return ret;
    };
    imports.wbg.__wbg_bufferData_bbb4fad8241073de = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
    };
    imports.wbg.__wbg_bufferData_9f31e49b55a28f79 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).bufferData(arg1 >>> 0, getArrayU8FromWasm0(arg2, arg3), arg4 >>> 0);
    };
    imports.wbg.__wbg_readPixels_7fd16b17126c5084 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
        getObject(arg0).readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7 === 0 ? undefined : getArrayU8FromWasm0(arg7, arg8));
    });
    imports.wbg.__wbg_texImage2D_fe7acef5c07410f2 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
    });
    imports.wbg.__wbg_texImage2D_094d41b8b638dc26 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    });
    imports.wbg.__wbg_texImage2D_0a899ac6c99d5ea3 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    });
    imports.wbg.__wbg_texImage2D_5f998b6abca94e7a = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    });
    imports.wbg.__wbg_texImage2D_a6710d56d9b5167b = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    });
    imports.wbg.__wbg_texImage2D_efc575c3e4a9d7df = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    });
    imports.wbg.__wbg_uniform1fv_37e83cc4d155992b = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform1fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform2fv_9d808b33d06dfce3 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform3fv_7316675cf6c56f5f = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform3fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniform4fv_103f58acbd7fb97a = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform4fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_uniformMatrix2fv_8b8529949e09f4cd = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix2fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix3fv_3098ede353246b9d = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix3fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_uniformMatrix4fv_587c4710f3d658e8 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_activeTexture_85a6cc041d2d1975 = function(arg0, arg1) {
        getObject(arg0).activeTexture(arg1 >>> 0);
    };
    imports.wbg.__wbg_attachShader_d87c96f460f4eb6e = function(arg0, arg1, arg2) {
        getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
    };
    imports.wbg.__wbg_bindAttribLocation_f9118b9afe6ff39d = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).bindAttribLocation(getObject(arg1), arg2 >>> 0, getStringFromWasm0(arg3, arg4));
    };
    imports.wbg.__wbg_bindBuffer_449cd5290cdcf8fc = function(arg0, arg1, arg2) {
        getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_bindFramebuffer_812afdb157a89267 = function(arg0, arg1, arg2) {
        getObject(arg0).bindFramebuffer(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_bindRenderbuffer_4cc12911711a1ccf = function(arg0, arg1, arg2) {
        getObject(arg0).bindRenderbuffer(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_bindTexture_5cb608a16add8d39 = function(arg0, arg1, arg2) {
        getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__wbg_blendFunc_29bd12994d15280b = function(arg0, arg1, arg2) {
        getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_checkFramebufferStatus_8157214f498bc828 = function(arg0, arg1) {
        var ret = getObject(arg0).checkFramebufferStatus(arg1 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_clear_cef7e1e0e1ba5465 = function(arg0, arg1) {
        getObject(arg0).clear(arg1 >>> 0);
    };
    imports.wbg.__wbg_clearColor_1010eed213a6cae8 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_compileShader_5d4e462508b1515e = function(arg0, arg1) {
        getObject(arg0).compileShader(getObject(arg1));
    };
    imports.wbg.__wbg_createBuffer_e8cf486cca25f5ed = function(arg0) {
        var ret = getObject(arg0).createBuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createFramebuffer_9cbd5d5ff3ddd15c = function(arg0) {
        var ret = getObject(arg0).createFramebuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createProgram_74d233ba41538562 = function(arg0) {
        var ret = getObject(arg0).createProgram();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createRenderbuffer_1859138144d48d84 = function(arg0) {
        var ret = getObject(arg0).createRenderbuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createShader_c35e740afca0efee = function(arg0, arg1) {
        var ret = getObject(arg0).createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createTexture_87a93aac65d1db98 = function(arg0) {
        var ret = getObject(arg0).createTexture();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_deleteProgram_9ecef1fe19c92477 = function(arg0, arg1) {
        getObject(arg0).deleteProgram(getObject(arg1));
    };
    imports.wbg.__wbg_deleteShader_23ad3b75ec8d45d1 = function(arg0, arg1) {
        getObject(arg0).deleteShader(getObject(arg1));
    };
    imports.wbg.__wbg_depthMask_14a50f78fd697b53 = function(arg0, arg1) {
        getObject(arg0).depthMask(arg1 !== 0);
    };
    imports.wbg.__wbg_detachShader_4d41a4cb6c8cf8fc = function(arg0, arg1, arg2) {
        getObject(arg0).detachShader(getObject(arg1), getObject(arg2));
    };
    imports.wbg.__wbg_drawArrays_1b1b532115644466 = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__wbg_drawElements_aea82c3e1c9ef971 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
    };
    imports.wbg.__wbg_enable_661b8ab579bd9c69 = function(arg0, arg1) {
        getObject(arg0).enable(arg1 >>> 0);
    };
    imports.wbg.__wbg_enableVertexAttribArray_eda4ec3cc346806e = function(arg0, arg1) {
        getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_framebufferRenderbuffer_a8ff35e047905ef3 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4));
    };
    imports.wbg.__wbg_framebufferTexture2D_b1e9d37b9d60afd2 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4), arg5);
    };
    imports.wbg.__wbg_generateMipmap_9d3ce82f14358475 = function(arg0, arg1) {
        getObject(arg0).generateMipmap(arg1 >>> 0);
    };
    imports.wbg.__wbg_getActiveAttrib_bf66147c0d29ca4a = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getActiveAttrib(getObject(arg1), arg2 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_getActiveUniform_1a0f6d129779214d = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getActiveUniform(getObject(arg1), arg2 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_getAttribLocation_2d81461aadc11bf6 = function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getAttribLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return ret;
    };
    imports.wbg.__wbg_getExtension_eb70e347733b74bb = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getExtension(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_getParameter_1f4e89a8b5d5b753 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).getParameter(arg1 >>> 0);
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_getProgramInfoLog_22b088fe758b29aa = function(arg0, arg1, arg2) {
        var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getProgramParameter_328434b297539fba = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getShaderInfoLog_1310dfc6a6714341 = function(arg0, arg1, arg2) {
        var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_getShaderParameter_c0c9b057b37ad55c = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getUniformLocation_356ed70052165dab = function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_linkProgram_413f1735416682a4 = function(arg0, arg1) {
        getObject(arg0).linkProgram(getObject(arg1));
    };
    imports.wbg.__wbg_pixelStorei_e5deb861cb5a37d5 = function(arg0, arg1, arg2) {
        getObject(arg0).pixelStorei(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_renderbufferStorage_a7099f7533a4fd8b = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
    };
    imports.wbg.__wbg_shaderSource_e95e88c01a88e78d = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_texParameteri_8bb94654a8896fcf = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    };
    imports.wbg.__wbg_uniform1f_8702b2439184a87a = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1f(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_uniform1i_a190be18b9337370 = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1i(getObject(arg1), arg2);
    };
    imports.wbg.__wbg_uniform2f_53184c3924a40f2b = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
    };
    imports.wbg.__wbg_uniform3f_a72390174c6194e2 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniform3f(getObject(arg1), arg2, arg3, arg4);
    };
    imports.wbg.__wbg_uniform4f_56ba9fda0aa89e95 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
    };
    imports.wbg.__wbg_useProgram_0172835766dd7682 = function(arg0, arg1) {
        getObject(arg0).useProgram(getObject(arg1));
    };
    imports.wbg.__wbg_vertexAttribPointer_46de816173486906 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    };
    imports.wbg.__wbg_viewport_58b0f8fe573107b8 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).viewport(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_setclassName_5c90ce56028589a5 = function(arg0, arg1, arg2) {
        getObject(arg0).className = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_clientWidth_9d49a840a7d34231 = function(arg0) {
        var ret = getObject(arg0).clientWidth;
        return ret;
    };
    imports.wbg.__wbg_clientHeight_61a64caaa4444b63 = function(arg0) {
        var ret = getObject(arg0).clientHeight;
        return ret;
    };
    imports.wbg.__wbg_requestPointerLock_ef27dbf2296e9458 = function(arg0) {
        getObject(arg0).requestPointerLock();
    };
    imports.wbg.__wbg_instanceof_HtmlElement_773e85b6bd68ae2d = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLElement;
        return ret;
    };
    imports.wbg.__wbg_style_ae2bb40204a83a34 = function(arg0) {
        var ret = getObject(arg0).style;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setonload_98f877645be108d6 = function(arg0, arg1) {
        getObject(arg0).onload = getObject(arg1);
    };
    imports.wbg.__wbg_setonerror_39830bb43447924a = function(arg0, arg1) {
        getObject(arg0).onerror = getObject(arg1);
    };
    imports.wbg.__wbg_signal_75d48461b733f3a1 = function(arg0) {
        var ret = getObject(arg0).signal;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_ab30a50b386fe26d = handleError(function() {
        var ret = new AbortController();
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_abort_45f593994718ecfd = function(arg0) {
        getObject(arg0).abort();
    };
    imports.wbg.__wbg_drawArraysInstancedANGLE_b716c4f3aeed9173 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawArraysInstancedANGLE(arg1 >>> 0, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_vertexAttribDivisorANGLE_d02bf8a0619fe251 = function(arg0, arg1, arg2) {
        getObject(arg0).vertexAttribDivisorANGLE(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_connect_11991f366578ef1d = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).connect(getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_newwithbuffersourcesequenceandoptions_3f7810a892cf2bd7 = handleError(function(arg0, arg1) {
        var ret = new Blob(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_addEventListener_116c561435e7160d = handleError(function(arg0, arg1, arg2, arg3) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    });
    imports.wbg.__wbg_addEventListener_27eb43df29303d67 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
    });
    imports.wbg.__wbg_removeEventListener_d9ceb7fdf4ca5166 = handleError(function(arg0, arg1, arg2, arg3) {
        getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    });
    imports.wbg.__wbg_instanceof_HtmlCanvasElement_d2d7786f00856e0a = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLCanvasElement;
        return ret;
    };
    imports.wbg.__wbg_setwidth_8d33dd91eeeee87d = function(arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    };
    imports.wbg.__wbg_setheight_757ff0f25240fd75 = function(arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    };
    imports.wbg.__wbg_getContext_59043a63a2f9266b = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_getContext_dc042961dbf1dae9 = handleError(function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2), getObject(arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    });
    imports.wbg.__wbg_size_a82aac52b686ef1b = function(arg0) {
        var ret = getObject(arg0).size;
        return ret;
    };
    imports.wbg.__wbg_type_700e1577b1de876a = function(arg0) {
        var ret = getObject(arg0).type;
        return ret;
    };
    imports.wbg.__wbg_name_2965898684ac94ce = function(arg0, arg1) {
        var ret = getObject(arg1).name;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        var ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_next_ff567d625ac44e49 = function(arg0) {
        var ret = getObject(arg0).next;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_610093e8f95067a4 = handleError(function(arg0) {
        var ret = getObject(arg0).next();
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_done_deb5f896b3ea14eb = function(arg0) {
        var ret = getObject(arg0).done;
        return ret;
    };
    imports.wbg.__wbg_value_5b6409ce10298b82 = function(arg0) {
        var ret = getObject(arg0).value;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_iterator_fe2907a0b53cd987 = function() {
        var ret = Symbol.iterator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_2e96a823c1c5a5bd = handleError(function(arg0, arg1) {
        var ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_call_e9f0ce4da840ab94 = handleError(function(arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_from_c205b0d4f5dc1441 = function(arg0) {
        var ret = Array.from(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_isArray_184f51bb3cd68808 = function(arg0) {
        var ret = Array.isArray(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_of_5654e5e60920a8a6 = function(arg0) {
        var ret = Array.of(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_ArrayBuffer_13deac6f163ebe71 = function(arg0) {
        var ret = getObject(arg0) instanceof ArrayBuffer;
        return ret;
    };
    imports.wbg.__wbg_values_bfec0c6fe421158f = function(arg0) {
        var ret = getObject(arg0).values();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_4896ab6bba55e0d9 = function(arg0, arg1) {
        var ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newnoargs_e2fdfe2af14a2323 = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_0dad7db75ec90ae7 = handleError(function(arg0, arg1, arg2) {
        var ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_isSafeInteger_0eed5f68c7ef008f = function(arg0) {
        var ret = Number.isSafeInteger(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_is_a2bc492e20d950cf = function(arg0, arg1) {
        var ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_new_8172f4fed77fdb7c = function() {
        var ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_7039bf8b99f049e1 = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_563(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            var ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_resolve_4df26938859b92e3 = function(arg0) {
        var ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_ffb6e71f7a6735ad = function(arg0, arg1) {
        var ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_021fcdc7f0350b58 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_179e8c2a5a4c73a3 = handleError(function() {
        var ret = self.self;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_window_492cfe63a6e41dfa = handleError(function() {
        var ret = window.window;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_globalThis_8ebfea75c2dd63ee = handleError(function() {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_global_62ea2619f58bf94d = handleError(function() {
        var ret = global.global;
        return addHeapObject(ret);
    });
    imports.wbg.__wbg_buffer_88f603259d7a7b82 = function(arg0) {
        var ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_2e98733d73dac355 = function(arg0) {
        var ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_new_85d8a1fc4384acef = function(arg0) {
        var ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_478951586c457484 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_5ba4b4465eeaa8d3 = function(arg0, arg1, arg2) {
        var ret = new Uint32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_db5ae49df92eb609 = function(arg0) {
        var ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_new_2d3653c61bf7d9f5 = function(arg0) {
        var ret = new Uint32Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_719fe1c0bde5fdd7 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_66305c055ad2f047 = function(arg0, arg1, arg2) {
        var ret = new Float32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_5ed9637f0c91cf31 = function(arg0) {
        var ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_new_97dfb1e289e6c216 = function(arg0) {
        var ret = new Float32Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_02fc6472d777f843 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_instanceof_Uint8Array_f334fbbaf83a3430 = function(arg0) {
        var ret = getObject(arg0) instanceof Uint8Array;
        return ret;
    };
    imports.wbg.__wbg_byteLength_eaa4a2fa4e78c5ae = function(arg0) {
        var ret = getObject(arg0).byteLength;
        return ret;
    };
    imports.wbg.__wbg_set_afe54b1eeb1aa77c = handleError(function(arg0, arg1, arg2) {
        var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    });
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        var ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_rethrow = function(arg0) {
        throw takeObject(arg0);
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1257 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 412, __wbg_adapter_35);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper980 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 356, __wbg_adapter_47);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper978 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 356, __wbg_adapter_41);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper570 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 183, __wbg_adapter_32);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper573 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 183, __wbg_adapter_38);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper571 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 183, __wbg_adapter_44);
        return addHeapObject(ret);
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

