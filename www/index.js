import {
    drawTerm,
    makeVariable,
    makeApplication,
    makeLambda,
    init,
    initSvgRoot
} from './common.js';
import * as wasm from "lambda-wasm";

init(() => {
    const svgTarget = document.getElementById('tree');
    initSvgRoot(svgTarget);
    const term = makeApplication(
        makeLambda('x', makeVariable('x')),
        makeLambda('y', makeApplication(makeVariable('f'), makeVariable('y'))),
    );
    drawTerm(term, svgTarget);
});
