import {
    drawTerm,
    makeVariable,
    makeApplication,
    makeLambda,
    init
} from './common.js';
import * as wasm from "lambda-wasm";

init(() => {
    const svgTarget = document.getElementById('tree');
    const term = makeLambda('x', makeVariable('x'));
    drawTerm(term, svgTarget);
});
