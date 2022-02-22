
const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : (typeof webkitAudioContext !== 'undefined' ? webkitAudioContext : undefined));
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

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
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

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
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

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

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
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachegetFloat64Memory0;
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

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}
function __wbg_adapter_32(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6e2bf2d78cf2d0ab(arg0, arg1, arg2);
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_35(arg0, arg1, arg2) {
    try {
        _assertNum(arg0);
        _assertNum(arg1);
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4bf180312e61a4bf(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_38(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h449b92d7b0b66678(arg0, arg1);
}

function __wbg_adapter_41(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he18adc722b0cce96(arg0, arg1, addHeapObject(arg2));
}

/**
*/
export function run() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.run(retptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
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
function __wbg_adapter_605(arg0, arg1, arg2, arg3) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm.wasm_bindgen__convert__closures__invoke2_mut__haeb44ed119806fe2(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
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
        input = new URL('my_demo_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        var ret = typeof(val) === 'object' && val !== null;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_error_09919627ac0992f5 = function() { return logError(function (arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    }, arguments) };
    imports.wbg.__wbg_new_693216e109162396 = function() { return logError(function () {
        var ret = new Error();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function() { return logError(function (arg0, arg1) {
        var ret = getObject(arg1).stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'number' ? obj : undefined;
        if (!isLikeNone(ret)) {
            _assertNum(ret);
        }
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        var ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        _assertNum(ret);
        return ret;
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        var ret = getObject(arg0) === getObject(arg1);
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_is_null = function(arg0) {
        var ret = getObject(arg0) === null;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_get_f7833d6ec572e462 = function() { return logError(function (arg0, arg1) {
        var ret = getObject(arg0)[takeObject(arg1)];
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_fbb49ad265f9dee8 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    }, arguments) };
    imports.wbg.__wbg_instanceof_WebGl2RenderingContext_df519ebc1fd4a55f = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof WebGL2RenderingContext;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_canvas_1396c967596541f8 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).canvas;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_drawingBufferWidth_71d8fc09f0e9f69f = function() { return logError(function (arg0) {
        var ret = getObject(arg0).drawingBufferWidth;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_drawingBufferHeight_6eb26c267e6b86e7 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).drawingBufferHeight;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_bindBufferBase_dc3a8efd711877b2 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).bindBufferBase(arg1 >>> 0, arg2 >>> 0, getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_bindVertexArray_8020efc46272d6b1 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).bindVertexArray(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_bufferData_17b90d9499ee7889 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_bufferData_2b2006d269bd7f65 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).bufferData(arg1 >>> 0, getArrayU8FromWasm0(arg2, arg3), arg4 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_bufferSubData_008a40cae643ab3e = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).bufferSubData(arg1 >>> 0, arg2, getObject(arg3), arg4 >>> 0, arg5 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_createVertexArray_ccfd68f784dda58d = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createVertexArray();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_drawArraysInstanced_9a1c5d4070c3ad43 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawArraysInstanced(arg1 >>> 0, arg2, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_drawBuffers_3e850289094e0ed2 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).drawBuffers(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_getActiveUniforms_3f3d227071aff863 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getActiveUniforms(getObject(arg1), getObject(arg2), arg3 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getUniformBlockIndex_40091d5f34e0ad56 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getUniformBlockIndex(getObject(arg1), getStringFromWasm0(arg2, arg3));
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getUniformIndices_f98214a89ace3c14 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).getUniformIndices(getObject(arg1), getObject(arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_readPixels_888cfd27c9babaee = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
        getObject(arg0).readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7 === 0 ? undefined : getArrayU8FromWasm0(arg7, arg8));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_40c3695cff5564ad = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_8efdc7c9762a31c4 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_ea4f44f738393ea2 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_1c691f3343c78750 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_a70059e5a3cd5248 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_b984b0275ce21109 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    }, arguments) };
    imports.wbg.__wbg_texImage3D_28f78acd64011f60 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
        getObject(arg0).texImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8 >>> 0, arg9 >>> 0, getObject(arg10));
    }, arguments) };
    imports.wbg.__wbg_uniform1fv_d082f50338e583d3 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform1fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_uniform1ui_7860cff51452b921 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).uniform1ui(getObject(arg1), arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_uniform2fv_cc77a4fd0d4ed937 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_uniform3fv_42274ac933a6d6ab = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform3fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_uniform4fv_78c67442a705f45f = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform4fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_uniformBlockBinding_0c9588e660d40948 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).uniformBlockBinding(getObject(arg1), arg2 >>> 0, arg3 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_uniformMatrix2fv_12f7ba152bd6acd9 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix2fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_uniformMatrix3fv_f5495d34084f3876 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix3fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_uniformMatrix4fv_8752c8df4a82f43a = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_vertexAttribDivisor_15b55770388d87bb = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).vertexAttribDivisor(arg1 >>> 0, arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_vertexAttribIPointer_875343d15147d6d2 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).vertexAttribIPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
    }, arguments) };
    imports.wbg.__wbg_activeTexture_e07e910acea70faa = function() { return logError(function (arg0, arg1) {
        getObject(arg0).activeTexture(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_attachShader_2e252ab2fda53d9b = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_bindAttribLocation_5c3fc4d764b702ab = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).bindAttribLocation(getObject(arg1), arg2 >>> 0, getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_bindBuffer_612af2c0d1623df9 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_bindFramebuffer_f79f98a252b25421 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).bindFramebuffer(arg1 >>> 0, getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_bindRenderbuffer_b68abb17f25b0056 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).bindRenderbuffer(arg1 >>> 0, getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_bindTexture_5de299363180ad48 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_blendFunc_a1fda75b5cf06b09 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_checkFramebufferStatus_8945ea81d89abba3 = function() { return logError(function (arg0, arg1) {
        var ret = getObject(arg0).checkFramebufferStatus(arg1 >>> 0);
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clear_4c5eed385310e256 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).clear(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_clearColor_d9d486c5ff20404c = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_compileShader_e224e94272352503 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).compileShader(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_createBuffer_564dc1c3c3f058b7 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createBuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createFramebuffer_ca860b7155b412f2 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createFramebuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createProgram_e9fa1d7669773667 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createProgram();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createRenderbuffer_3d592bfc4a5cfea6 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createRenderbuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createShader_03233922e9b5ebf2 = function() { return logError(function (arg0, arg1) {
        var ret = getObject(arg0).createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createTexture_7ee50a5b223f0511 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createTexture();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_deleteProgram_0d4952ded7ec132a = function() { return logError(function (arg0, arg1) {
        getObject(arg0).deleteProgram(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_depthMask_44ff350c6f8d4d91 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).depthMask(arg1 !== 0);
    }, arguments) };
    imports.wbg.__wbg_detachShader_c7115572e0c5095c = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).detachShader(getObject(arg1), getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_disable_e61fb08d6c7131e4 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).disable(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_drawArrays_aaa2fa80ca85e04c = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
    }, arguments) };
    imports.wbg.__wbg_drawElements_8f3cfd28610fd46e = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
    }, arguments) };
    imports.wbg.__wbg_enable_8e888a63831a3fe5 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).enable(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_enableVertexAttribArray_d1b2636395bdaa7a = function() { return logError(function (arg0, arg1) {
        getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_framebufferRenderbuffer_e19af39663a3b959 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4));
    }, arguments) };
    imports.wbg.__wbg_framebufferTexture2D_ceadbfd128a6e565 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4), arg5);
    }, arguments) };
    imports.wbg.__wbg_generateMipmap_35669af1ecd88073 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).generateMipmap(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_getAttribLocation_7f79c73e983e47cd = function() { return logError(function (arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getAttribLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getParameter_ecc6d50165f87cce = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).getParameter(arg1 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getProgramInfoLog_dbd8d8cedcc8cdcc = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_getProgramParameter_4b9d43902599c2d2 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getShaderInfoLog_5aab05280bd0fe1b = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_getShaderParameter_e5f7e371d4eec000 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getUniformLocation_9541edb0d39d1646 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_linkProgram_116382e2dc17af64 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).linkProgram(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_pixelStorei_ea8cf13cf2f14a47 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).pixelStorei(arg1 >>> 0, arg2);
    }, arguments) };
    imports.wbg.__wbg_renderbufferStorage_f9546132469c19c6 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_shaderSource_0066bb6817bf9e88 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_texParameteri_52fb3e85a6d2c636 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    }, arguments) };
    imports.wbg.__wbg_uniform1f_96a968d4f5cb18de = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).uniform1f(getObject(arg1), arg2);
    }, arguments) };
    imports.wbg.__wbg_uniform1i_a6ce351ee8cef296 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).uniform1i(getObject(arg1), arg2);
    }, arguments) };
    imports.wbg.__wbg_uniform2f_84c79c4f8bb2428e = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
    }, arguments) };
    imports.wbg.__wbg_uniform3f_236679e1bca8660b = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniform3f(getObject(arg1), arg2, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_uniform4f_0ff24ef1f3ab8946 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
    }, arguments) };
    imports.wbg.__wbg_useProgram_de22d1e01c430663 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).useProgram(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_vertexAttribPointer_f2f4a8e4c14289d7 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    }, arguments) };
    imports.wbg.__wbg_viewport_caffbaa3e8b9568b = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).viewport(arg1, arg2, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_instanceof_Window_434ce1849eb4e0fc = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof Window;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_document_5edd43643d1060d9 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_location_11472bb76bf5bbca = function() { return logError(function (arg0) {
        var ret = getObject(arg0).location;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_innerWidth_405786923c1d2641 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).innerWidth;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_innerHeight_25d3be0d129329c3 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).innerHeight;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_alert_27dab5298cb616e6 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).alert(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_cancelAnimationFrame_7c55daff0068fc2b = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).cancelAnimationFrame(arg1);
    }, arguments) };
    imports.wbg.__wbg_requestAnimationFrame_0c71cd3c6779a371 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_fetch_8b74b08010b8d7f8 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).fetch(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setsrc_579baf76725d93d9 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).src = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setcrossOrigin_ff7bb2bd3b455c07 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).crossOrigin = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setcurrentTime_c91519ebd7ac537a = function() { return logError(function (arg0, arg1) {
        getObject(arg0).currentTime = arg1;
    }, arguments) };
    imports.wbg.__wbg_duration_df739e61b0ba220d = function() { return logError(function (arg0) {
        var ret = getObject(arg0).duration;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setautoplay_f8f407ae7e49b024 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).autoplay = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_setloop_bfc89fd2bd98e58e = function() { return logError(function (arg0, arg1) {
        getObject(arg0).loop = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_pause_37a5e2a8ec48cd2b = function() { return handleError(function (arg0) {
        getObject(arg0).pause();
    }, arguments) };
    imports.wbg.__wbg_play_4453bc10fffd28e6 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).play();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithbuffersourcesequenceandoptions_4517520f6253804a = function() { return handleError(function (arg0, arg1) {
        var ret = new Blob(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_4abffc489cdc3ac9 = function() { return handleError(function (arg0, arg1) {
        var ret = new Event(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_a9e789a128c5986f = function() { return handleError(function () {
        var ret = new Audio();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_drawArraysInstancedANGLE_d8e6549aacc0d996 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawArraysInstancedANGLE(arg1 >>> 0, arg2, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_vertexAttribDivisorANGLE_2dc41a79843a435c = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).vertexAttribDivisorANGLE(arg1 >>> 0, arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_addEventListener_6bdba88519fdc1c9 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_addEventListener_55682f77717d7665 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
    }, arguments) };
    imports.wbg.__wbg_removeEventListener_8d16089e686f486a = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_removeEventListener_9cd36e5806463d5d = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), arg4 !== 0);
    }, arguments) };
    imports.wbg.__wbg_new_69df8d622f579abf = function() { return handleError(function (arg0) {
        var ret = new GainNode(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlCanvasElement_a6157e470d06b638 = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof HTMLCanvasElement;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setwidth_362e8db8cbadbe96 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    }, arguments) };
    imports.wbg.__wbg_setheight_28f53831182cc410 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    }, arguments) };
    imports.wbg.__wbg_getContext_bd4e9445094eda84 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getContext_10d5c2a4cc0737c8 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2), getObject(arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_src_69686f5ca82d71b7 = function() { return logError(function (arg0, arg1) {
        var ret = getObject(arg1).src;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_setsrc_b0a1ac4dd261ae2d = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).src = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setcrossOrigin_07e0e4935571a4c5 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).crossOrigin = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_naturalWidth_5a4b3262de67f9a9 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).naturalWidth;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_naturalHeight_c1b21ea61ff05b74 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).naturalHeight;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_da67f111e299956e = function() { return handleError(function () {
        var ret = new Image();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_settextContent_07dfb193b5deabbc = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_appendChild_3fe5090c665d3bb4 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_removeChild_f4a83c9698136bbb = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).removeChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_arrayBuffer_0e2a43f68a8b3e49 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).arrayBuffer();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_json_4ab99130d1a5b3a9 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).json();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_text_aeba5a5bbfef7f15 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).text();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_e8b3a478a1b32d55 = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof CanvasRenderingContext2D;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setfillStyle_680401a47926faa5 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).fillStyle = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_fillRect_7ad775936c1881ce = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_duration_13da0eca8da7e83e = function() { return logError(function (arg0) {
        var ret = getObject(arg0).duration;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setProperty_ebb06e7fa941d6a8 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_origin_1bc82542e055b010 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg1).origin;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_pathname_d0014089875ea691 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg1).pathname;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_search_7e1c9ba7f3985c36 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg1).search;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_bindVertexArrayOES_4364f11e81712180 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).bindVertexArrayOES(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_createVertexArrayOES_54cc0b7c450f4662 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createVertexArrayOES();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_drawBuffersWEBGL_a7706a0daac89708 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).drawBuffersWEBGL(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_body_7538539844356c1c = function() { return logError(function (arg0) {
        var ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_pointerLockElement_774a8f1a871b753b = function() { return logError(function (arg0) {
        var ret = getObject(arg0).pointerLockElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createElement_d017b8d2af99bab9 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setclassName_579da04a76182ffb = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).className = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_clientWidth_4c903f82634f2159 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).clientWidth;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clientHeight_cddbd2cef19a2cb1 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).clientHeight;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_requestPointerLock_8c1b1a4b2317d05b = function() { return logError(function (arg0) {
        getObject(arg0).requestPointerLock();
    }, arguments) };
    imports.wbg.__wbg_instanceof_WebGlRenderingContext_2be4c068bf5f8362 = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof WebGLRenderingContext;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_canvas_d0b58be124e596e3 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).canvas;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_drawingBufferWidth_0bffb3bee322d4ff = function() { return logError(function (arg0) {
        var ret = getObject(arg0).drawingBufferWidth;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_drawingBufferHeight_4ca7b7b942d7e887 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).drawingBufferHeight;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_bufferData_85d635f32a990208 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_bufferData_b6f1f72e6ee3e8c1 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).bufferData(arg1 >>> 0, getArrayU8FromWasm0(arg2, arg3), arg4 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_readPixels_10168509835277d6 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
        getObject(arg0).readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7 === 0 ? undefined : getArrayU8FromWasm0(arg7, arg8));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_16ff123798c82f60 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_a696a10802dbc647 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_eaeef8056d3730ea = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_b46a9b691e69d90b = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_ab99d214117999e9 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    }, arguments) };
    imports.wbg.__wbg_texImage2D_031a02cc3c82b7e1 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
    }, arguments) };
    imports.wbg.__wbg_uniform1fv_e6a2134edff4f2e9 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform1fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_uniform2fv_39b447bb2f7ef74f = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_uniform3fv_a9ee182585ffb135 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform3fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_uniform4fv_481536ab64fdd3a3 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform4fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_uniformMatrix2fv_aaa4e0f7c15bca04 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix2fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_uniformMatrix3fv_73edc84c125080b9 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix3fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_uniformMatrix4fv_f07c6caf5a563616 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_activeTexture_74ed11a5c5d5af90 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).activeTexture(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_attachShader_55dbe770f3ee32ca = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_bindAttribLocation_8a0fac0b00d92ae8 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).bindAttribLocation(getObject(arg1), arg2 >>> 0, getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_bindBuffer_29d52e7bc48650c3 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_bindFramebuffer_bd35ddd23765c7b6 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).bindFramebuffer(arg1 >>> 0, getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_bindRenderbuffer_2d4dbbeabb74952f = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).bindRenderbuffer(arg1 >>> 0, getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_bindTexture_198c816345baca83 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_blendFunc_c8f1e0fb4467f57c = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_checkFramebufferStatus_1161f8158f9a5e03 = function() { return logError(function (arg0, arg1) {
        var ret = getObject(arg0).checkFramebufferStatus(arg1 >>> 0);
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clear_2af1271959ec83d7 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).clear(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_clearColor_51c4f69c743c3252 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_compileShader_3b5f9ef4c67a0777 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).compileShader(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_createBuffer_c40f37e1348bb91f = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createBuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createFramebuffer_410b12a5cc5a8f13 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createFramebuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createProgram_245520da1fb9e47b = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createProgram();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createRenderbuffer_516e5681213c5e91 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createRenderbuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createShader_4d8818a13cb825b3 = function() { return logError(function (arg0, arg1) {
        var ret = getObject(arg0).createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createTexture_f3a6a715d6bada45 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).createTexture();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_deleteProgram_61cc7923289d1bbc = function() { return logError(function (arg0, arg1) {
        getObject(arg0).deleteProgram(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_depthMask_75e08708e9136383 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).depthMask(arg1 !== 0);
    }, arguments) };
    imports.wbg.__wbg_detachShader_32e4b718f0a63080 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).detachShader(getObject(arg1), getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_disable_2b63b75dc6c27537 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).disable(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_drawArrays_22c88d644a33fd59 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
    }, arguments) };
    imports.wbg.__wbg_drawElements_6e26500a25ecf478 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
    }, arguments) };
    imports.wbg.__wbg_enable_8f6dd779ccb8e1de = function() { return logError(function (arg0, arg1) {
        getObject(arg0).enable(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_enableVertexAttribArray_4ed5f91d0718bee1 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_framebufferRenderbuffer_dc299f6ac156bc82 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4));
    }, arguments) };
    imports.wbg.__wbg_framebufferTexture2D_31643260e5b0b294 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4), arg5);
    }, arguments) };
    imports.wbg.__wbg_generateMipmap_04cccfe789890de0 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).generateMipmap(arg1 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_getAttribLocation_da5df7094096113d = function() { return logError(function (arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getAttribLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getExtension_c6ceee3244ee7f20 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).getExtension(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getParameter_cf7a00ba1cbac0d3 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).getParameter(arg1 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getProgramInfoLog_c253042b64e86027 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_getProgramParameter_4f698af0dda0a2d4 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getShaderInfoLog_584794e3bcf1e19b = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_getShaderParameter_64b1ffe576e5fa25 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getUniformLocation_703972f150a46500 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_linkProgram_5fdd57237c761833 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).linkProgram(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_pixelStorei_fab41fe53c557df3 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).pixelStorei(arg1 >>> 0, arg2);
    }, arguments) };
    imports.wbg.__wbg_renderbufferStorage_3f48f93db9d0a1db = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_shaderSource_173ab97288934a60 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_texParameteri_caec5468f2a850c3 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    }, arguments) };
    imports.wbg.__wbg_uniform1f_258478814234cf9c = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).uniform1f(getObject(arg1), arg2);
    }, arguments) };
    imports.wbg.__wbg_uniform1i_a0275676828a22b6 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).uniform1i(getObject(arg1), arg2);
    }, arguments) };
    imports.wbg.__wbg_uniform2f_b0af46ba675f2c0d = function() { return logError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
    }, arguments) };
    imports.wbg.__wbg_uniform3f_65416973a351fbab = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).uniform3f(getObject(arg1), arg2, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_uniform4f_e5d0a91bf98b35ad = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
    }, arguments) };
    imports.wbg.__wbg_useProgram_d5898a40ebe88916 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).useProgram(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_vertexAttribPointer_c994b02cace20298 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    }, arguments) };
    imports.wbg.__wbg_viewport_19577064127daf83 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).viewport(arg1, arg2, arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_destination_97006ee89653d765 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).destination;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_currentTime_45c7c9179b453a2e = function() { return logError(function (arg0) {
        var ret = getObject(arg0).currentTime;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_state_a7f1a46b31f0ae4f = function() { return logError(function (arg0) {
        var ret = getObject(arg0).state;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_7b23bc5a2d082b0d = function() { return handleError(function () {
        var ret = new lAudioContext();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createMediaElementSource_7856afc3b8b3b71f = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).createMediaElementSource(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_suspend_8e4c09519e8f017f = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).suspend();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createBufferSource_16fe7fb2b5f1ae6e = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).createBufferSource();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_decodeAudioData_9606f09cf371539f = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).decodeAudioData(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_resume_d6be3e69971f8e8a = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).resume();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_connect_23205ccf67cb254c = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).connect(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_disconnect_1a99836a042586a0 = function() { return handleError(function (arg0) {
        getObject(arg0).disconnect();
    }, arguments) };
    imports.wbg.__wbg_instanceof_MouseEvent_e20234cd6f6ebeb5 = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof MouseEvent;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clientX_849ccdf456d662ac = function() { return logError(function (arg0) {
        var ret = getObject(arg0).clientX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clientY_1aaff30fe0cd0876 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).clientY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_movementX_954e41adbd12b11f = function() { return logError(function (arg0) {
        var ret = getObject(arg0).movementX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_movementY_f9664367f6924290 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).movementY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_origin_3c7540756a363be8 = function() { return logError(function (arg0, arg1) {
        var ret = getObject(arg1).origin;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_new_4473c9af1cac368b = function() { return handleError(function (arg0, arg1) {
        var ret = new URL(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createObjectURL_c7ec2d7d39afe850 = function() { return handleError(function (arg0, arg1) {
        var ret = URL.createObjectURL(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlElement_d3e8f1c1d6788b24 = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof HTMLElement;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_style_16f5dd9624687c8f = function() { return logError(function (arg0) {
        var ret = getObject(arg0).style;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setonended_585579a7a631e3ec = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onended = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonload_9235de4503eb82c8 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onload = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonerror_939f617c2b40758c = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onerror = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_signal_259ba662a5555524 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).signal;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_7456dc18cc110e9c = function() { return handleError(function () {
        var ret = new AbortController();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_abort_3c64506fb0036132 = function() { return logError(function (arg0) {
        getObject(arg0).abort();
    }, arguments) };
    imports.wbg.__wbg_setbuffer_1e6c597abc9ea97c = function() { return logError(function (arg0, arg1) {
        getObject(arg0).buffer = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setloop_f5c377cc47ce7134 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).loop = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_setonended_430beeec06ee2bd6 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onended = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_start_24109d0cb0662d1f = function() { return handleError(function (arg0) {
        getObject(arg0).start();
    }, arguments) };
    imports.wbg.__wbg_start_41a4d92f67b9f3b4 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).start(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_stop_d7c464c12318fa44 = function() { return handleError(function (arg0) {
        getObject(arg0).stop();
    }, arguments) };
    imports.wbg.__wbg_sethref_90772480eb0439e5 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).href = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_newwithstr_1cda4846e9bc4738 = function() { return handleError(function (arg0, arg1) {
        var ret = new Request(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_debug_6df4b1a327dd2e94 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_error_ca520cb687b085a1 = function() { return logError(function (arg0) {
        console.error(getObject(arg0));
    }, arguments) };
    imports.wbg.__wbg_error_644d3bc8c0537e80 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_info_8bed0988e7416289 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_log_681299aef22afa27 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_warn_ca021eeadd0df9cd = function() { return logError(function (arg0, arg1, arg2, arg3) {
        console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    }, arguments) };
    imports.wbg.__wbg_newwithlength_9c398a17849b31ce = function() { return logError(function (arg0) {
        var ret = new Array(arg0 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_get_f45dff51f52d7222 = function() { return logError(function (arg0, arg1) {
        var ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_a42efa3c7f01c8b1 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    }, arguments) };
    imports.wbg.__wbg_from_4216160a11e086ef = function() { return logError(function (arg0) {
        var ret = Array.from(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_isArray_8480ed76e5369634 = function() { return logError(function (arg0) {
        var ret = Array.isArray(getObject(arg0));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_of_6e090615ff06688d = function() { return logError(function (arg0) {
        var ret = Array.of(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_ArrayBuffer_649f53c967aec9b3 = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof ArrayBuffer;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_values_71935f80778b5113 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).values();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_55259b13834a484c = function() { return logError(function (arg0, arg1) {
        var ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newnoargs_f579424187aa1717 = function() { return logError(function (arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_89558c3e96703ca1 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_94697a95cb7e239c = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_next_dd1a890d37e38d73 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_next_c7a2a6b012059a5e = function() { return logError(function (arg0) {
        var ret = getObject(arg0).next;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_done_982b1c7ac0cbc69d = function() { return logError(function (arg0) {
        var ret = getObject(arg0).done;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_value_2def2d1fb38b02cd = function() { return logError(function (arg0) {
        var ret = getObject(arg0).value;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_isSafeInteger_91192d88df6f12b9 = function() { return logError(function (arg0) {
        var ret = Number.isSafeInteger(getObject(arg0));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_is_3d73f4d91adacc37 = function() { return logError(function (arg0, arg1) {
        var ret = Object.is(getObject(arg0), getObject(arg1));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_d3138911a89329b0 = function() { return logError(function () {
        var ret = new Object();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_iterator_4b9cedbeda0c0e30 = function() { return logError(function () {
        var ret = Symbol.iterator;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_4beacc9c71572250 = function() { return logError(function (arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_605(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            var ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    }, arguments) };
    imports.wbg.__wbg_resolve_4f8f547f26b30b27 = function() { return logError(function (arg0) {
        var ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_then_a6860c82b90816ca = function() { return logError(function (arg0, arg1) {
        var ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_then_58a04e42527f52c6 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_d61b1f48a57191ae = function() { return handleError(function () {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_self_e23d74ae45fb17d1 = function() { return handleError(function () {
        var ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_b4be7f48b24ac56e = function() { return handleError(function () {
        var ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_e7669da72fd7f239 = function() { return handleError(function () {
        var ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_Uint8Array_8a8537f46e056474 = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof Uint8Array;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_e3b800e570795b3c = function() { return logError(function (arg0) {
        var ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_length_30803400a8f15c59 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_5b8081e9d002f0df = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_new_86a3fd385f9bcaf2 = function() { return logError(function (arg0) {
        var ret = new Uint32Array(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithbyteoffsetandlength_f6c2c5e40f6f5bda = function() { return logError(function (arg0, arg1, arg2) {
        var ret = new Uint32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_length_9cd802c9f902a90b = function() { return logError(function (arg0) {
        var ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_9b03dc359228a2db = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_new_f5438c0cea22a3aa = function() { return logError(function (arg0) {
        var ret = new Float32Array(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithbyteoffsetandlength_ad2916c6fa7d4c6f = function() { return logError(function (arg0, arg1, arg2) {
        var ret = new Float32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_length_44449d3b5928d07c = function() { return logError(function (arg0) {
        var ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_7cb6639737aebb39 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        var ret = typeof(getObject(arg0)) === 'function';
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_buffer_5e74a88a1424a2e0 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_get_8bbb82393651dd9c = function() { return handleError(function (arg0, arg1) {
        var ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_c42875065132a932 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
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
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper4164 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 13, __wbg_adapter_32);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper4166 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 11, __wbg_adapter_35);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper9991 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 644, __wbg_adapter_38);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper10647 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 674, __wbg_adapter_41);
        return addHeapObject(ret);
    }, arguments) };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

