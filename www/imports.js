import { redraw } from "../../../voronoi.js"

var gl;
var canvas;

export function setupCanvas() {
    canvas = document.createElement('canvas');
    gl = canvas.getContext('webgl');
    if (!gl) {
        console.error("Failed to get a WebGL context for the canvas!");
        return;
    }
    document.body.append(canvas);
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    window.addEventListener('resize', recalc);
    setInterval(recalc, 10);
}

function recalc() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    gl.viewport(0, 0, canvas.width, canvas.height);
    redraw(canvas.width, canvas.height, Date.now() % 100000 / 1000);
}

export function clearToBlue() {
    gl.clearColor(0.1, 0.1, 0.9, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);
}

export function attachShader(program, shader) {
    gl.attachShader(program, shader);
}
export function bindBuffer(target, id) {
    gl.bindBuffer(target, id);
}
export function bufferDataF32(target, data, usage) {
    gl.bufferData(target, data, usage);
}
export function bufferDataU16(target, data, usage) {
    gl.bufferData(target, data, usage);
}
export function clear(mask) {
    gl.clear(mask)
}
export function enable(flag) {
    gl.enable(flag)
}
export function depthFunc(func) {
    gl.depthFunc(func)
}
export function getClientWidth() {
    return gl.canvas.clientWidth;
}
export function uniformMatrix4fv(location, transpose, value) {
    gl.uniformMatrix4fv(location, transpose, value);
}
export function getClientHeight() {
    return gl.canvas.clientHeight;
}
export function clearColor(r, g, b, a) {
    gl.clearColor(r, g, b, a);
}
export function clearDepth(d) {
    gl.clearDepth(d);
}
export function compileShader(shader) {
    gl.compileShader(shader);
}
export function createBuffer() {
    return gl.createBuffer();
}
export function createProgram() {
    return gl.createProgram();
}
export function createShader(shader_type) {
    return gl.createShader(shader_type);
}
export function drawElements(mode, count, type, offset) {
    gl.drawElements(mode, count, type, offset);
}
export function drawArrays(mode, offset, count) {
    gl.drawArrays(mode, offset, count);
}
export function enableVertexAttribArray(index) {
    gl.enableVertexAttribArray(index)
}
export function getAttribLocation(program, name) {
    return gl.getAttribLocation(program, name);
}
export function getUniformLocation(program, name) {
    return gl.getUniformLocation(program, name);
}
export function linkProgram(program) {
    gl.linkProgram(program);
}
export function shaderSource(shader, source) {
    gl.shaderSource(shader, source);
}
export function useProgram(program) {
    gl.useProgram(program);
}
export function vertexAttribPointer(index, size, type, normalized, stride, offset) {
    gl.vertexAttribPointer(index, size, type, normalized, stride, offset);
}