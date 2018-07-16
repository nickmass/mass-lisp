
                (function() {
                    var wasm;
                    const __exports = {};
                    

const __wbg_f_random_random_n_target = Math.random;

__exports.__wbg_f_random_random_n = function() {
    return __wbg_f_random_random_n_target();
};

const __wbg_f_newWindow_new_window_n_target = massLispGfx.newWindow;

__exports.__wbg_f_newWindow_new_window_n = function(arg0, arg1) {
    return __wbg_f_newWindow_new_window_n_target(arg0, arg1);
};

const __wbg_f_pollEvents_poll_events_n_target = massLispGfx.pollEvents;

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null ||
        cachegetUint32Memory.buffer !== wasm.memory.buffer)
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    return cachegetUint32Memory;
}

let cachegetUint64Memory = null;
function getUint64Memory() {
    if (cachegetUint64Memory === null ||
        cachegetUint64Memory.buffer !== wasm.memory.buffer)
        cachegetUint64Memory = new BigUint64Array(wasm.memory.buffer);
    return cachegetUint64Memory;
}

function passArray32ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 4);
    getUint32Memory().set(arg, ptr / 4);
    return [ptr, arg.length];
}

__exports.__wbg_f_pollEvents_poll_events_n = function(ret, arg0) {
    const [retptr, retlen] = passArray32ToWasm(__wbg_f_pollEvents_poll_events_n_target(arg0));
    const mem = getUint32Memory();
                    mem[ret / 4] = retptr;
                    mem[ret / 4 + 1] = retlen;

};

const __wbg_f_pollMouse_poll_mouse_n_target = massLispGfx.pollMouse;

let cachegetFloat32Memory = null;
function getFloat32Memory() {
    if (cachegetFloat32Memory === null ||
        cachegetFloat32Memory.buffer !== wasm.memory.buffer)
        cachegetFloat32Memory = new Float32Array(wasm.memory.buffer);
    return cachegetFloat32Memory;
}

function passArrayF32ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 4);
    getFloat32Memory().set(arg, ptr / 4);
    return [ptr, arg.length];
}

__exports.__wbg_f_pollMouse_poll_mouse_n = function(ret, arg0) {
    const [retptr, retlen] = passArrayF32ToWasm(__wbg_f_pollMouse_poll_mouse_n_target(arg0));
    const mem = getUint32Memory();
                    mem[ret / 4] = retptr;
                    mem[ret / 4 + 1] = retlen;

};

const __wbg_f_setClearColor_set_clear_color_n_target = massLispGfx.setClearColor;

__exports.__wbg_f_setClearColor_set_clear_color_n = function(arg0, arg1) {
    let carg1 = Color.__construct(arg1);
    __wbg_f_setClearColor_set_clear_color_n_target(arg0, carg1);
};

const __wbg_f_setLineWidth_set_line_width_n_target = massLispGfx.setLineWidth;

__exports.__wbg_f_setLineWidth_set_line_width_n = function(arg0, arg1) {
    __wbg_f_setLineWidth_set_line_width_n_target(arg0, arg1);
};

const __wbg_f_drawLine_draw_line_n_target = massLispGfx.drawLine;

__exports.__wbg_f_drawLine_draw_line_n = function(arg0, arg1, arg2, arg3) {
    let carg1 = Point.__construct(arg1);
    let carg2 = Point.__construct(arg2);
    let carg3 = Color.__construct(arg3);
    __wbg_f_drawLine_draw_line_n_target(arg0, carg1, carg2, carg3);
};

const __wbg_f_drawLineList_draw_line_list_n_target = massLispGfx.drawLineList;

function getArrayF32FromWasm(ptr, len) {
    return getFloat32Memory().subarray(ptr / 4, ptr / 4 + len);
}

__exports.__wbg_f_drawLineList_draw_line_list_n = function(arg0, arg1, arg2, arg3, arg4, arg5) {
    let varg1 = getArrayF32FromWasm(arg1, arg2);
    let varg3 = getArrayF32FromWasm(arg3, arg4);
    let carg5 = Color.__construct(arg5);
    __wbg_f_drawLineList_draw_line_list_n_target(arg0, varg1, varg3, carg5);
};

const __wbg_f_drawCircle_draw_circle_n_target = massLispGfx.drawCircle;

__exports.__wbg_f_drawCircle_draw_circle_n = function(arg0, arg1, arg2, arg3) {
    let carg1 = Point.__construct(arg1);
    let carg3 = Color.__construct(arg3);
    __wbg_f_drawCircle_draw_circle_n_target(arg0, carg1, arg2, carg3);
};

const __wbg_f_draw_draw_n_target = massLispGfx.draw;

__exports.__wbg_f_draw_draw_n = function(arg0) {
    __wbg_f_draw_draw_n_target(arg0);
};

const __wbg_f_log_log_n_target = console.log;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null ||
        cachegetUint8Memory.buffer !== wasm.memory.buffer)
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

__exports.__wbg_f_log_log_n = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    __wbg_f_log_log_n_target(varg0);
};

const __wbg_f_printLine_print_line_n_target = massLispConsole.printLine;

__exports.__wbg_f_printLine_print_line_n = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    varg0 = varg0.slice();
    wasm.__wbindgen_free(arg0, arg1 * 1);
    __wbg_f_printLine_print_line_n_target(varg0);
};

const __wbg_f_print_print_n_target = massLispConsole.print;

__exports.__wbg_f_print_print_n = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    varg0 = varg0.slice();
    wasm.__wbindgen_free(arg0, arg1 * 1);
    __wbg_f_print_print_n_target(varg0);
};

const __wbg_f_readLine_read_line_n_target = massLispConsole.readLine;

let cachedEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {

    const buf = cachedEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

__exports.__wbg_f_readLine_read_line_n = function(ret) {
    const [retptr, retlen] = passStringToWasm(__wbg_f_readLine_read_line_n_target());
    const mem = getUint32Memory();
                    mem[ret / 4] = retptr;
                    mem[ret / 4 + 1] = retlen;

};

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null)
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    return cachedGlobalArgumentPtr;
}

__exports.eval = function(arg0) {
    const [ptr0, len0] = passStringToWasm(arg0);
    const retptr = globalArgumentPtr();
    wasm.eval(retptr, ptr0, len0);
    const mem = getUint32Memory();
    const ptr = mem[retptr / 4];
    const len = mem[retptr / 4 + 1];
    const realRet = getStringFromWasm(ptr, len).slice();
    wasm.__wbindgen_free(ptr, len * 1);
    return realRet;
};

__exports.resume = function() {
    return (wasm.resume()) !== 0;
};

__exports.reset = function() {
    return wasm.reset();
};

class Color {

                static __construct(ptr) {
                    return new Color(ptr);
                }

                constructor(ptr) {
                    this.ptr = ptr;
                }
            get r() {
    return wasm.__wbg_get_color_r(this.ptr);
}
set r(arg0) {
    return wasm.__wbg_set_color_r(this.ptr, arg0);
}get g() {
    return wasm.__wbg_get_color_g(this.ptr);
}
set g(arg0) {
    return wasm.__wbg_set_color_g(this.ptr, arg0);
}get b() {
    return wasm.__wbg_get_color_b(this.ptr);
}
set b(arg0) {
    return wasm.__wbg_set_color_b(this.ptr, arg0);
}get r() {
    return wasm.__wbg_get_color_r(this.ptr);
}
set r(arg0) {
    return wasm.__wbg_set_color_r(this.ptr, arg0);
}get g() {
    return wasm.__wbg_get_color_g(this.ptr);
}
set g(arg0) {
    return wasm.__wbg_set_color_g(this.ptr, arg0);
}get b() {
    return wasm.__wbg_get_color_b(this.ptr);
}
set b(arg0) {
    return wasm.__wbg_set_color_b(this.ptr, arg0);
}get r() {
    return wasm.__wbg_get_color_r(this.ptr);
}
set r(arg0) {
    return wasm.__wbg_set_color_r(this.ptr, arg0);
}get g() {
    return wasm.__wbg_get_color_g(this.ptr);
}
set g(arg0) {
    return wasm.__wbg_set_color_g(this.ptr, arg0);
}get b() {
    return wasm.__wbg_get_color_b(this.ptr);
}
set b(arg0) {
    return wasm.__wbg_set_color_b(this.ptr, arg0);
}get r() {
    return wasm.__wbg_get_color_r(this.ptr);
}
set r(arg0) {
    return wasm.__wbg_set_color_r(this.ptr, arg0);
}get g() {
    return wasm.__wbg_get_color_g(this.ptr);
}
set g(arg0) {
    return wasm.__wbg_set_color_g(this.ptr, arg0);
}get b() {
    return wasm.__wbg_get_color_b(this.ptr);
}
set b(arg0) {
    return wasm.__wbg_set_color_b(this.ptr, arg0);
}get r() {
    return wasm.__wbg_get_color_r(this.ptr);
}
set r(arg0) {
    return wasm.__wbg_set_color_r(this.ptr, arg0);
}get g() {
    return wasm.__wbg_get_color_g(this.ptr);
}
set g(arg0) {
    return wasm.__wbg_set_color_g(this.ptr, arg0);
}get b() {
    return wasm.__wbg_get_color_b(this.ptr);
}
set b(arg0) {
    return wasm.__wbg_set_color_b(this.ptr, arg0);
}get r() {
    return wasm.__wbg_get_color_r(this.ptr);
}
set r(arg0) {
    return wasm.__wbg_set_color_r(this.ptr, arg0);
}get g() {
    return wasm.__wbg_get_color_g(this.ptr);
}
set g(arg0) {
    return wasm.__wbg_set_color_g(this.ptr, arg0);
}get b() {
    return wasm.__wbg_get_color_b(this.ptr);
}
set b(arg0) {
    return wasm.__wbg_set_color_b(this.ptr, arg0);
}get r() {
    return wasm.__wbg_get_color_r(this.ptr);
}
set r(arg0) {
    return wasm.__wbg_set_color_r(this.ptr, arg0);
}get g() {
    return wasm.__wbg_get_color_g(this.ptr);
}
set g(arg0) {
    return wasm.__wbg_set_color_g(this.ptr, arg0);
}get b() {
    return wasm.__wbg_get_color_b(this.ptr);
}
set b(arg0) {
    return wasm.__wbg_set_color_b(this.ptr, arg0);
}get r() {
    return wasm.__wbg_get_color_r(this.ptr);
}
set r(arg0) {
    return wasm.__wbg_set_color_r(this.ptr, arg0);
}get g() {
    return wasm.__wbg_get_color_g(this.ptr);
}
set g(arg0) {
    return wasm.__wbg_set_color_g(this.ptr, arg0);
}get b() {
    return wasm.__wbg_get_color_b(this.ptr);
}
set b(arg0) {
    return wasm.__wbg_set_color_b(this.ptr, arg0);
}get r() {
    return wasm.__wbg_get_color_r(this.ptr);
}
set r(arg0) {
    return wasm.__wbg_set_color_r(this.ptr, arg0);
}get g() {
    return wasm.__wbg_get_color_g(this.ptr);
}
set g(arg0) {
    return wasm.__wbg_set_color_g(this.ptr, arg0);
}get b() {
    return wasm.__wbg_get_color_b(this.ptr);
}
set b(arg0) {
    return wasm.__wbg_set_color_b(this.ptr, arg0);
}get r() {
    return wasm.__wbg_get_color_r(this.ptr);
}
set r(arg0) {
    return wasm.__wbg_set_color_r(this.ptr, arg0);
}get g() {
    return wasm.__wbg_get_color_g(this.ptr);
}
set g(arg0) {
    return wasm.__wbg_set_color_g(this.ptr, arg0);
}get b() {
    return wasm.__wbg_get_color_b(this.ptr);
}
set b(arg0) {
    return wasm.__wbg_set_color_b(this.ptr, arg0);
}
            free() {
                const ptr = this.ptr;
                this.ptr = 0;
                wasm.__wbg_color_free(ptr);
            }
        }
__exports.Color = Color;

class Point {

                static __construct(ptr) {
                    return new Point(ptr);
                }

                constructor(ptr) {
                    this.ptr = ptr;
                }
            get x() {
    return wasm.__wbg_get_point_x(this.ptr);
}
set x(arg0) {
    return wasm.__wbg_set_point_x(this.ptr, arg0);
}get y() {
    return wasm.__wbg_get_point_y(this.ptr);
}
set y(arg0) {
    return wasm.__wbg_set_point_y(this.ptr, arg0);
}get x() {
    return wasm.__wbg_get_point_x(this.ptr);
}
set x(arg0) {
    return wasm.__wbg_set_point_x(this.ptr, arg0);
}get y() {
    return wasm.__wbg_get_point_y(this.ptr);
}
set y(arg0) {
    return wasm.__wbg_set_point_y(this.ptr, arg0);
}get x() {
    return wasm.__wbg_get_point_x(this.ptr);
}
set x(arg0) {
    return wasm.__wbg_set_point_x(this.ptr, arg0);
}get y() {
    return wasm.__wbg_get_point_y(this.ptr);
}
set y(arg0) {
    return wasm.__wbg_set_point_y(this.ptr, arg0);
}get x() {
    return wasm.__wbg_get_point_x(this.ptr);
}
set x(arg0) {
    return wasm.__wbg_set_point_x(this.ptr, arg0);
}get y() {
    return wasm.__wbg_get_point_y(this.ptr);
}
set y(arg0) {
    return wasm.__wbg_set_point_y(this.ptr, arg0);
}get x() {
    return wasm.__wbg_get_point_x(this.ptr);
}
set x(arg0) {
    return wasm.__wbg_set_point_x(this.ptr, arg0);
}get y() {
    return wasm.__wbg_get_point_y(this.ptr);
}
set y(arg0) {
    return wasm.__wbg_set_point_y(this.ptr, arg0);
}get x() {
    return wasm.__wbg_get_point_x(this.ptr);
}
set x(arg0) {
    return wasm.__wbg_set_point_x(this.ptr, arg0);
}get y() {
    return wasm.__wbg_get_point_y(this.ptr);
}
set y(arg0) {
    return wasm.__wbg_set_point_y(this.ptr, arg0);
}get x() {
    return wasm.__wbg_get_point_x(this.ptr);
}
set x(arg0) {
    return wasm.__wbg_set_point_x(this.ptr, arg0);
}get y() {
    return wasm.__wbg_get_point_y(this.ptr);
}
set y(arg0) {
    return wasm.__wbg_set_point_y(this.ptr, arg0);
}get x() {
    return wasm.__wbg_get_point_x(this.ptr);
}
set x(arg0) {
    return wasm.__wbg_set_point_x(this.ptr, arg0);
}get y() {
    return wasm.__wbg_get_point_y(this.ptr);
}
set y(arg0) {
    return wasm.__wbg_set_point_y(this.ptr, arg0);
}get x() {
    return wasm.__wbg_get_point_x(this.ptr);
}
set x(arg0) {
    return wasm.__wbg_set_point_x(this.ptr, arg0);
}get y() {
    return wasm.__wbg_get_point_y(this.ptr);
}
set y(arg0) {
    return wasm.__wbg_set_point_y(this.ptr, arg0);
}get x() {
    return wasm.__wbg_get_point_x(this.ptr);
}
set x(arg0) {
    return wasm.__wbg_set_point_x(this.ptr, arg0);
}get y() {
    return wasm.__wbg_get_point_y(this.ptr);
}
set y(arg0) {
    return wasm.__wbg_set_point_y(this.ptr, arg0);
}
            free() {
                const ptr = this.ptr;
                this.ptr = 0;
                wasm.__wbg_point_free(ptr);
            }
        }
__exports.Point = Point;

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

__exports.__wbindgen_sin = function(x) { return Math.sin(x); };

__exports.__wbindgen_cos = function(x) { return Math.cos(x); };

__exports.__wbindgen_Math_tan = function(x) { return Math.tan(x); };

__exports.__wbindgen_Math_asin = function(x) { return Math.asin(x); };

__exports.__wbindgen_Math_acos = function(x) { return Math.acos(x); };

__exports.__wbindgen_Math_atan = function(x) { return Math.atan(x); };

__exports.__wbindgen_Math_atan2 = function(x, y) { return Math.atan2(x, y); };

__exports.__wbindgen_pow = function(x, y) { return Math.pow(x, y); };

__exports.__wbindgen_fmod = function(a, b) { return a % b; };

                    function init(wasm_path) {
                        return fetch(wasm_path)
                            .then(response => response.arrayBuffer())
                            .then(buffer => WebAssembly.instantiate(buffer, { './lisp_web': __exports }))
                            .then(({instance}) => {
                                wasm = init.wasm = instance.exports;
                                return;
                            });
                    };
                    self.MassLisp = Object.assign(init, __exports);
                })();
            
